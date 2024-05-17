/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::bindgen::config::Config;
use crate::bindgen::declarationtyperesolver::DeclarationTypeResolver;
use crate::bindgen::dependencies::Dependencies;
use crate::bindgen::ir::{
    AnnotationSet, Cfg, Documentation, GenericArgument, GenericParams, Item, ItemContainer,
    KnownErasedTypes, Path, Type,
};
use crate::bindgen::library::Library;
use crate::bindgen::mangle;
use crate::bindgen::monomorph::Monomorphs;

#[derive(Debug, Clone)]
pub struct OpaqueItem {
    pub path: Path,
    pub export_name: String,
    pub generic_params: GenericParams,
    pub cfg: Option<Cfg>,
    pub annotations: AnnotationSet,
    pub documentation: Documentation,
}

impl OpaqueItem {
    pub fn load(
        path: Path,
        generics: &syn::Generics,
        attrs: &[syn::Attribute],
        mod_cfg: Option<&Cfg>,
    ) -> Result<OpaqueItem, String> {
        Ok(Self::new(
            path,
            GenericParams::load(generics)?,
            Cfg::append(mod_cfg, Cfg::load(attrs)),
            AnnotationSet::load(attrs).unwrap_or_else(|_| AnnotationSet::new()),
            Documentation::load(attrs),
        ))
    }

    pub fn new(
        path: Path,
        generic_params: GenericParams,
        cfg: Option<Cfg>,
        annotations: AnnotationSet,
        documentation: Documentation,
    ) -> OpaqueItem {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generic_params,
            cfg,
            annotations,
            documentation,
        }
    }

    pub fn try_erase_type(&self, generics: &[GenericArgument], _config: &Config) -> Option<Type> {
        if let Some(GenericArgument::Type(ref ty)) = generics.first() {
            match self.name() {
                "Option" => {
                    if let Some(nullable) = ty.make_nullable() {
                        return Some(nullable);
                    }
                    if let Some(zeroable) = ty.make_zeroable(true) {
                        return Some(zeroable);
                    }
                }
                "NonNull" => {
                    return Some(Type::Ptr {
                        ty: Box::new(ty.clone()),
                        is_const: false,
                        is_nullable: false,
                        is_ref: false,
                    })
                }
                "NonZero" => {
                    if let Some(nonzero) = ty.make_zeroable(false) {
                        return Some(nonzero);
                    }
                }
                _ => {}
            }
        }
        None
    }
}

impl Item for OpaqueItem {
    fn path(&self) -> &Path {
        &self.path
    }

    fn export_name(&self) -> &str {
        &self.export_name
    }

    fn cfg(&self) -> Option<&Cfg> {
        self.cfg.as_ref()
    }

    fn annotations(&self) -> &AnnotationSet {
        &self.annotations
    }

    fn annotations_mut(&mut self) -> &mut AnnotationSet {
        &mut self.annotations
    }

    fn documentation(&self) -> &Documentation {
        &self.documentation
    }

    fn container(&self) -> ItemContainer {
        ItemContainer::OpaqueItem(self.clone())
    }

    fn collect_declaration_types(&self, resolver: &mut DeclarationTypeResolver) {
        resolver.add_struct(&self.path);
    }

    fn generic_params(&self) -> Option<&GenericParams> {
        Some(&self.generic_params)
    }

    fn erase_types_inplace(
        &mut self,
        _library: &Library,
        _erased: &mut KnownErasedTypes,
        _generics: &[GenericArgument],
    ) {
        // Nothing to do here, because we don't (transitively) embed any types and
        // `Type::erased_types` takes care of erasing generic args for us.
        warn!(
            "Not attempting to erase opaque type {:?} with generics {:?}",
            self, _generics
        );
    }

    fn rename_for_config(&mut self, config: &Config) {
        config.export.rename(&mut self.export_name);
    }

    fn add_dependencies(&self, _: &Library, _: &mut Dependencies) {}

    fn instantiate_monomorph(
        &self,
        generic_values: &[GenericArgument],
        library: &Library,
        out: &mut Monomorphs,
    ) {
        assert!(self.is_generic(), "{} is not generic", self.path);

        // We can be instantiated with less generic params because of default
        // template parameters, or because of empty types that we remove during
        // parsing (`()`).
        assert!(
            self.generic_params.len() >= generic_values.len(),
            "{} has {} params but is being instantiated with {} values",
            self.path,
            self.generic_params.len(),
            generic_values.len(),
        );

        let mangled_path = mangle::mangle_path(
            &self.path,
            generic_values,
            &library.get_config().export.mangle,
        );

        let monomorph = OpaqueItem::new(
            mangled_path,
            GenericParams::default(),
            self.cfg.clone(),
            self.annotations.clone(),
            self.documentation.clone(),
        );

        out.insert_opaque(self, monomorph, generic_values.to_owned());
    }
}
