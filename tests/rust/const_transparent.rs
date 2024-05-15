#[repr(transparent)]
struct TransparentStruct { field: u8 }

impl TransparentStruct {
    pub const FOO: TransparentStruct = TransparentStruct { field: 1 };
    pub const BAR: i64 = 2;
}

#[repr(transparent)]
struct TransparentTupleStruct(u8);

/// cbindgen:transparent-typedef
#[repr(transparent)]
struct Wrapper<T> { field: T }

pub const FOO: TransparentStruct = TransparentStruct { field: 0 };
pub const BAR: TransparentTupleStruct = TransparentTupleStruct(0);
pub const BAZ: Wrapper<TransparentStruct> = Wrapper { field: TransparentStruct { field: 0 } };

#[repr(transparent)]
struct TransparentStructWithErasedField<T> {
    field: Wrapper<T>,
}

pub const BLAH: TransparentStructWithErasedField<TransparentStruct> = TransparentStructWithErasedField {
    field: Wrapper { field: TransparentStruct { field: 0 } }
};

#[repr(transparent)]
enum TransparentEnum {
    A { field: u8 },
}

impl TransparentEnum {
    pub const FOO: TransparentEnum = TransparentEnum::A { field: 1 };
    pub const BAR: i64 = 2;
}

#[repr(transparent)]
enum TransparentTupleEnum {
    A(u8),
}

#[repr(transparent)]
enum TransparentWrapperEnum<T> {
    A { field: T },
}

pub const EFOO: TransparentEnum = TransparentEnum::A { field: 0 };
pub const EBAR: TransparentTupleEnum = TransparentTupleEnum::A(0);
pub const EBAZ: TransparentWrapperEnum<TransparentEnum> = TransparentWrapperEnum::A {
    field: TransparentEnum::A { field: 0 }
};
