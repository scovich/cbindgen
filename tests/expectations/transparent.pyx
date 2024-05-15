from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  ctypedef struct DummyStruct:
    pass

  ctypedef struct EnumWithAssociatedConstantInImpl:
    pass

  ctypedef DummyStruct TransparentComplexWrappingStructTuple;

  ctypedef uint32_t TransparentPrimitiveWrappingStructTuple;

  ctypedef DummyStruct TransparentComplexWrappingStructure;

  ctypedef uint32_t TransparentPrimitiveWrappingStructure;

  ctypedef DummyStruct TransparentComplexWrapper_i32;

  ctypedef uint32_t TransparentPrimitiveWrapper_i32;

  ctypedef uint32_t TransparentPrimitiveWithAssociatedConstants;
  const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ZERO # = 0
  const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ONE # = 1

  ctypedef const uint32_t *TransparentPointerWrappingStructure;

  ctypedef enum EnumStruct_Tag:
    A,

  ctypedef struct A_Body:
    uint8_t only_field;

  ctypedef struct EnumStruct:
    EnumStruct_Tag tag;
    A_Body a;

  ctypedef int32_t *TransparentEnumStruct;

  const TransparentPrimitiveWrappingStructure EnumWithAssociatedConstantInImpl_TEN # = 10

  void simple_root(int32_t n);

  void root(TransparentComplexWrappingStructTuple a,
            TransparentPrimitiveWrappingStructTuple b,
            TransparentComplexWrappingStructure c,
            TransparentPrimitiveWrappingStructure d,
            TransparentComplexWrapper_i32 e,
            TransparentPrimitiveWrapper_i32 f,
            TransparentPrimitiveWithAssociatedConstants g,
            EnumWithAssociatedConstantInImpl h,
            TransparentPointerWrappingStructure i,
            uint32_t *j,
            uint32_t *k,
            uint32_t *l,
            TransparentPrimitiveWrappingStructure m,
            uint32_t *n,
            int32_t o,
            int32_t p,
            int32_t *q,
            EnumStruct r,
            TransparentEnumStruct s,
            int32_t *const *t,
            int32_t u,
            uint8_t v);
