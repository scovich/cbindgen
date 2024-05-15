struct DummyStruct;

// Transparent struct tuple wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStructTuple(DummyStruct);

// Transparent struct tuple wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStructTuple(u32);

// Transparent structure wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrappingStructure { only_field: DummyStruct }

// Transparent structure wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrappingStructure { only_field: u32 }

// Transparent structure wrapping a pointer
#[repr(transparent)]
struct TransparentPointerWrappingStructure { only_field: *const u32 }

// Transparent structure wrapping a non-null pointer
#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentNonNullPointerWrappingStructure { only_field: NonNull<u32> }

// Transparent structure wrapping an optional non-null pointer
#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentOptionalNonNullPointerWrappingStructure { only_field: Option<NonNull<u32>> }

// Transparent structure wrapping another transparent structure.
#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentWrappingAnotherTransparentStructure { only_field: TransparentPrimitiveWrappingStructure }

// Transparent structure wrapping a transparent non-null pointer structure.
/// cbindgen:transparent-typedef
#[repr(transparent)]
struct ErasedTransparentWrappingTransparentNonNullPointerStructure { only_field: ErasedTransparentNonNullPointerWrappingStructure }

// Transparent structure wrapping another type
#[repr(transparent)]
/// cbindgen:transparent-typedef
struct ErasedTransparentWrappingAnotherType<T> { only_field: T }

/// cbindgen:transparent-typedef
type ErasedTransparentInt = ErasedTransparentWrappingAnotherType<i32>;
type TransparentComplex = ErasedTransparentWrappingAnotherType<DummyStruct>;
type TransparentTransparent = ErasedTransparentWrappingAnotherType<TransparentPrimitiveWrappingStructure>;
type TransparentNonNull = ErasedTransparentWrappingAnotherType<NonNull<u32>>;
type TransparentOptionNonNull = ErasedTransparentWrappingAnotherType<Option<NonNull<u32>>>;

// Transparent struct wrapper with a marker wrapping a struct.
#[repr(transparent)]
struct TransparentComplexWrapper<T> {
    only_non_zero_sized_field: DummyStruct,
    marker: PhantomData<T>,
}

// Transparent struct wrapper with a marker wrapping a primitive.
#[repr(transparent)]
struct TransparentPrimitiveWrapper<T> {
    only_non_zero_sized_field: u32,
    marker: PhantomData<T>,
}

// Associated constant declared before struct declaration.
impl TransparentPrimitiveWithAssociatedConstants {
    pub const ZERO: TransparentPrimitiveWithAssociatedConstants = TransparentPrimitiveWithAssociatedConstants {
        bits: 0
    };
}

// Transparent structure wrapping a primitive with associated constants.
#[repr(transparent)]
struct TransparentPrimitiveWithAssociatedConstants { bits: u32 }

// Associated constant declared after struct declaration.
impl TransparentPrimitiveWithAssociatedConstants {
    pub const ONE: TransparentPrimitiveWithAssociatedConstants = TransparentPrimitiveWithAssociatedConstants {
        bits: 1
    };
}

enum EnumWithAssociatedConstantInImpl { A }

impl EnumWithAssociatedConstantInImpl {
    pub const TEN: TransparentPrimitiveWrappingStructure = TransparentPrimitiveWrappingStructure { only_field: 10 };
}

#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentEnumTuple {
    A(i32),
}

#[repr(transparent)]
enum TransparentEnumStruct {
    A { only_field: Cell<NonNull<ErasedTransparentInt>> },
}

#[repr(transparent)]
/// cbindgen:transparent-typedef
enum ErasedTransparentEnumStruct {
    A { only_field: NonNull<ErasedTransparentInt> },
}

#[repr(C)]
enum EnumStruct {
    A { only_field: Option<NonZeroU8> },
}

#[no_mangle]
pub extern "C" fn simple_root(
    n: ErasedTransparentInt,
) { }

#[no_mangle]
pub extern "C" fn root(
    a: TransparentComplexWrappingStructTuple,
    b: TransparentPrimitiveWrappingStructTuple,
    c: TransparentComplexWrappingStructure,
    d: TransparentPrimitiveWrappingStructure,
    e: TransparentComplexWrapper<i32>,
    f: TransparentPrimitiveWrapper<i32>,
    g: TransparentPrimitiveWithAssociatedConstants,
    h: EnumWithAssociatedConstantInImpl,
    i: TransparentPointerWrappingStructure,
    j: ErasedTransparentNonNullPointerWrappingStructure,
    k: ErasedTransparentOptionalNonNullPointerWrappingStructure,
    l: Option<ErasedTransparentNonNullPointerWrappingStructure>,
    m: ErasedTransparentWrappingAnotherTransparentStructure,
    n: ErasedTransparentWrappingTransparentNonNullPointerStructure,
    o: ErasedTransparentInt,
    p: ErasedTransparentEnumTuple,
    q: ErasedTransparentEnumStruct,
    r: EnumStruct,
    s: TransparentEnumStruct,
    t: &Box<i32>,
    u: ErasedTransparentWrappingAnotherType<ErasedTransparentInt>,
    v: NonZeroU8,
) { }
