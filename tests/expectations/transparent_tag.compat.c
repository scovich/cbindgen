#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

typedef struct DummyStruct TransparentComplexWrappingStructTuple;

typedef uint32_t TransparentPrimitiveWrappingStructTuple;

typedef struct DummyStruct TransparentComplexWrappingStructure;

typedef uint32_t TransparentPrimitiveWrappingStructure;

typedef struct DummyStruct TransparentComplexWrapper_i32;

typedef uint32_t TransparentPrimitiveWrapper_i32;

typedef uint32_t TransparentPrimitiveWithAssociatedConstants;
#define TransparentPrimitiveWithAssociatedConstants_ZERO 0
#define TransparentPrimitiveWithAssociatedConstants_ONE 1

typedef const uint32_t *TransparentPointerWrappingStructure;

enum EnumStruct_Tag {
  A,
};

struct A_Body {
  uint8_t only_field;
};

struct EnumStruct {
  enum EnumStruct_Tag tag;
  union {
    struct A_Body a;
  };
};

typedef int32_t *TransparentEnumStruct;

#define EnumWithAssociatedConstantInImpl_TEN 10

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void simple_root(int32_t n);

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper_i32 e,
          TransparentPrimitiveWrapper_i32 f,
          TransparentPrimitiveWithAssociatedConstants g,
          struct EnumWithAssociatedConstantInImpl h,
          TransparentPointerWrappingStructure i,
          uint32_t *j,
          uint32_t *k,
          uint32_t *l,
          TransparentPrimitiveWrappingStructure m,
          uint32_t *n,
          int32_t o,
          int32_t p,
          int32_t *q,
          struct EnumStruct r,
          TransparentEnumStruct s,
          int32_t *const *t,
          int32_t u,
          uint8_t v);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
