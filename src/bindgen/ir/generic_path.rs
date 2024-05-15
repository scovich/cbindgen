use std::collections::HashMap;
use std::io::Write;
use std::ops::Deref;

use syn::ext::IdentExt;

use crate::bindgen::cdecl;
use crate::bindgen::config::{Config, Language};
use crate::bindgen::declarationtyperesolver::{DeclarationType, DeclarationTypeResolver};
use crate::bindgen::ir::{ConstExpr, Path, Type};
use crate::bindgen::language_backend::LanguageBackend;
use crate::bindgen::library::Library;
use crate::bindgen::utilities::IterHelpers;
use crate::bindgen::writer::SourceWriter;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GenericParamType {
    Type,
    Const(Type),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GenericParam {
    name: Path,
    ty: GenericParamType,
    default: Option<GenericArgument>,
}

impl GenericParam {
    pub fn new_type_param(name: &str) -> Self {
        GenericParam {
            name: Path::new(name),
            ty: GenericParamType::Type,
            default: None,
        }
    }

    pub fn load(param: &syn::GenericParam) -> Result<Option<Self>, String> {
        match *param {
            syn::GenericParam::Type(syn::TypeParam {
                ref ident,
                ref default,
                ..
            }) => {
                let default = match default.as_ref().map(Type::load).transpose()? {
                    None => None,
                    Some(None) => Err(format!("unsupported generic type default: {:?}", default))?,
                    Some(Some(ty)) => Some(GenericArgument::Type(ty)),
                };
                Ok(Some(GenericParam {
                    name: Path::new(ident.unraw().to_string()),
                    ty: GenericParamType::Type,
                    default,
                }))
            }

            syn::GenericParam::Lifetime(_) => Ok(None),

            syn::GenericParam::Const(syn::ConstParam {
                ref ident,
                ref ty,
                ref default,
                ..
            }) => match Type::load(ty)? {
                None => {
                    // A type that evaporates, like PhantomData.
                    Err(format!("unsupported const generic type: {:?}", ty))
                }
                Some(ty) => Ok(Some(GenericParam {
                    name: Path::new(ident.unraw().to_string()),
                    ty: GenericParamType::Const(ty),
                    default: default
                        .as_ref()
                        .map(ConstExpr::load)
                        .transpose()?
                        .map(GenericArgument::Const),
                })),
            },
        }
    }

    pub fn name(&self) -> &Path {
        &self.name
    }
}

/// If `generics` is empty, create a set of "default" generic arguments, which preserves the
/// existing parameter name. Useful to allow `call` to work when no generics are provided.
pub struct MaybeDefaultGenericAguments<'a> {
    generics: &'a [GenericArgument],
    defaults: Option<Vec<GenericArgument>>,
}

impl<'a> MaybeDefaultGenericAguments<'a> {
    pub fn new(params: &GenericParams, generics: &'a [GenericArgument]) -> Self {
        let defaults = if generics.is_empty() && !params.is_empty() {
            Some(
                params
                    .iter()
                    .cloned()
                    .map(|param| {
                        GenericArgument::Type(Type::Path(GenericPath::new(
                            param.name.clone(),
                            vec![],
                        )))
                    })
                    .collect(),
            )
        } else {
            None
        };
        Self { generics, defaults }
    }

    pub fn as_slice(&self) -> &[GenericArgument] {
        self.defaults
            .as_ref()
            .map_or(self.generics, |defaults| defaults.as_slice())
    }
}

#[derive(Default, Debug, Clone)]
pub struct GenericParams(pub Vec<GenericParam>);

impl GenericParams {
    pub fn load(generics: &syn::Generics) -> Result<Self, String> {
        let mut params = vec![];
        for param in &generics.params {
            if let Some(p) = GenericParam::load(param)? {
                params.push(p);
            }
        }

        Ok(GenericParams(params))
    }

    /// Associate each parameter with an argument.
    pub fn call<'out>(
        &'out self,
        item_name: &str,
        arguments: &'out [GenericArgument],
    ) -> Vec<(&'out Path, &'out GenericArgument)> {
        assert!(
            self.len() >= arguments.len(),
            "{} has {} params but is being instantiated with {} values",
            item_name,
            self.len(),
            arguments.len(),
        );
        self.iter()
            .enumerate()
            .map(|(i, param)| {
                let arg = arguments
                    .get(i)
                    .or(param.default.as_ref())
                    .expect("not enough arguments for generic");
                (param.name(), arg)
            })
            .collect()
    }

    pub(crate) fn write_internal<F: Write, LB: LanguageBackend>(
        &self,
        language_backend: &mut LB,
        config: &Config,
        out: &mut SourceWriter<F>,
        with_default: bool,
    ) {
        if !self.0.is_empty() && config.language == Language::Cxx {
            out.write("template<");
            for (i, item) in self.0.iter().enumerate() {
                if i != 0 {
                    out.write(", ");
                }
                match item.ty {
                    GenericParamType::Type => {
                        write!(out, "typename {}", item.name);
                        if let Some(GenericArgument::Type(ref ty)) = item.default {
                            write!(out, " = ");
                            cdecl::write_type(language_backend, out, ty, config);
                        } else if with_default {
                            write!(out, " = void");
                        }
                    }
                    GenericParamType::Const(ref ty) => {
                        cdecl::write_field(language_backend, out, ty, item.name.name(), config);
                        if let Some(GenericArgument::Const(ref expr)) = item.default {
                            write!(out, " = {}", expr.as_str());
                        } else if with_default {
                            write!(out, " = 0");
                        }
                    }
                }
            }
            out.write(">");
            out.new_line();
        }
    }

    pub fn write_with_default<F: Write, LB: LanguageBackend>(
        &self,
        language_backend: &mut LB,
        config: &Config,
        out: &mut SourceWriter<F>,
    ) {
        self.write_internal(language_backend, config, out, true);
    }
}

impl Deref for GenericParams {
    type Target = [GenericParam];

    fn deref(&self) -> &[GenericParam] {
        &self.0
    }
}

/// A (non-lifetime) argument passed to a generic, either a type or a constant expression.
///
/// Note: Both arguments in a type like `Array<T, N>` are represented as
/// `GenericArgument::Type`s, even if `N` is actually the name of a const. This
/// is a consequence of `syn::GenericArgument` doing the same thing.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GenericArgument {
    Type(Type),
    Const(ConstExpr),
}

impl GenericArgument {
    pub fn specialize(&self, mappings: &[(&Path, &GenericArgument)]) -> GenericArgument {
        match *self {
            GenericArgument::Type(ref ty) => {
                if let Type::Path(ref path) = *ty {
                    if path.is_single_identifier() {
                        // See note on `GenericArgument` above: `ty` may
                        // actually be the name of a const. Check for that now.
                        for &(name, value) in mappings {
                            if *name == path.path {
                                return value.clone();
                            }
                        }
                    }
                }
                GenericArgument::Type(ty.specialize(mappings))
            }
            GenericArgument::Const(ref expr) => GenericArgument::Const(expr.clone()),
        }
    }

    pub fn rename_for_config(&mut self, config: &Config, generic_params: &GenericParams) {
        match *self {
            //GenericArgument::Generic(ref param) => warn!("Cannot rename generic param {:?}", param),
            GenericArgument::Type(ref mut ty) => ty.rename_for_config(config, generic_params),
            GenericArgument::Const(ref mut expr) => expr.rename_for_config(config),
        }
    }
}

#[derive(Default)]
pub struct KnownErasedTypes {
    // Remember paths we've already visited, so we don't repeat unnecessary work.
    // TODO: how to handle recursive types such as `struct Foo { next: Box<Foo> }`?
    known_types: HashMap<Type, Option<Type>>,
}

impl KnownErasedTypes {
    pub fn is_erased(&self, path: &Path) -> bool {
        let path = Type::Path(GenericPath::new(path.clone(), vec![]));
        self.known_types.contains_key(&path)
    }

    pub fn erase_types_inplace(
        &mut self,
        library: &Library,
        target: &mut Type,
        mappings: &[(&Path, &GenericArgument)],
    ) {
        if let Some(erased_type) = self.erase_types(library, target, mappings) {
            warn!("Replacing {:?} with {:?}", target, erased_type);
            *target = erased_type;
        }
    }
    #[must_use]
    pub fn erase_types(
        &mut self,
        library: &Library,
        target: &Type,
        mappings: &[(&Path, &GenericArgument)],
    ) -> Option<Type> {
        // NOTE: If there are template type mappings, apply them before performing type erasure and
        // cache the result of _that_ replacement; the target will be replaced with the erased type
        // (if any), or else the specialized type (if any).
        let specialized = if mappings.is_empty() {
            None
        } else {
            let specialized = target.specialize(mappings);
            if specialized != *target {
                warn!(
                    "specialized {:?} as {:?} using mappings {:?}",
                    target, specialized, mappings
                );
                Some(specialized)
            } else {
                None
            }
        };
        //let target = specialized.as_ref().unwrap_or(target);
        let maybe_known_type = self.known_types.get(target);
        let unknown_type = maybe_known_type.is_none();
        let erased_type: Option<Type> = if let Some(known_type) = maybe_known_type {
            known_type.clone()
        } else {
            let mut erased_ty = target.erase_types(library, mappings, self);
            while let Some(ref ty) = erased_ty {
                match ty.erase_types(library, mappings, self) {
                    Some(ty) => {
                        warn!("Erased type {:?} is itself erasable as {:?}", erased_ty, ty);
                        erased_ty = Some(ty);
                    }
                    None => break,
                }
            }
            if let Type::Path(path) = target {
                if path.name() == "T" {
                    warn!("^^^ Replacing a template param!\n");
                }
            };
            erased_ty
        };
        if unknown_type {
            warn!("Caching erasure of {:?} as {:?}", target, erased_type);
            self.known_types.insert(target.clone(), erased_type.clone());
        }
        erased_type.or(specialized)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GenericPath {
    path: Path,
    export_name: String,
    generics: Vec<GenericArgument>,
    ctype: Option<DeclarationType>,
}

impl GenericPath {
    pub fn new(path: Path, generics: Vec<GenericArgument>) -> Self {
        let export_name = path.name().to_owned();
        Self {
            path,
            export_name,
            generics,
            ctype: None,
        }
    }

    pub fn self_path() -> Self {
        Self::new(Path::new("Self"), vec![])
    }

    pub fn replace_self_with(&mut self, self_ty: &Path) {
        if self.path.replace_self_with(self_ty) {
            self_ty.name().clone_into(&mut self.export_name);
        }
        // Caller deals with generics.
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn generics(&self) -> &[GenericArgument] {
        &self.generics
    }

    pub fn generics_mut(&mut self) -> &mut [GenericArgument] {
        &mut self.generics
    }

    pub fn ctype(&self) -> Option<&DeclarationType> {
        self.ctype.as_ref()
    }

    pub fn name(&self) -> &str {
        self.path.name()
    }

    pub fn export_name(&self) -> &str {
        &self.export_name
    }

    pub fn is_single_identifier(&self) -> bool {
        self.generics.is_empty()
    }

    pub fn rename_for_config(&mut self, config: &Config, generic_params: &GenericParams) {
        for generic in &mut self.generics {
            generic.rename_for_config(config, generic_params);
        }
        if !generic_params.iter().any(|param| param.name == self.path) {
            config.export.rename(&mut self.export_name);
        }
    }

    pub fn resolve_declaration_types(&mut self, resolver: &DeclarationTypeResolver) {
        self.ctype = resolver.type_for(&self.path);
    }

    pub fn load(path: &syn::Path) -> Result<Self, String> {
        assert!(
            !path.segments.is_empty(),
            "{:?} doesn't have any segments",
            path
        );
        let last_segment = path.segments.last().unwrap();
        let name = last_segment.ident.unraw().to_string();

        let path = Path::new(name);
        let phantom_data_path = Path::new("PhantomData");
        if path == phantom_data_path {
            return Ok(Self::new(path, Vec::new()));
        }

        let generics = match last_segment.arguments {
            syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                ref args,
                ..
            }) => args.iter().try_skip_map(|x| match *x {
                syn::GenericArgument::Type(ref x) => Ok(Type::load(x)?.map(GenericArgument::Type)),
                syn::GenericArgument::Lifetime(_) => Ok(None),
                syn::GenericArgument::Const(ref x) => {
                    Ok(Some(GenericArgument::Const(ConstExpr::load(x)?)))
                }
                _ => Err(format!("can't handle generic argument {:?}", x)),
            })?,
            syn::PathArguments::Parenthesized(_) => {
                return Err("Path contains parentheses.".to_owned());
            }
            _ => Vec::new(),
        };

        Ok(Self::new(path, generics))
    }
}
