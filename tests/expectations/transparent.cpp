#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

template<typename T = void>
struct Box;

struct DummyStruct;

struct EnumWithAssociatedConstantInImpl;

using TransparentComplexWrappingStructTuple = DummyStruct;

using TransparentPrimitiveWrappingStructTuple = uint32_t;

using TransparentComplexWrappingStructure = DummyStruct;

using TransparentPrimitiveWrappingStructure = uint32_t;

template<typename T>
using TransparentComplexWrapper = DummyStruct;

template<typename T>
using TransparentPrimitiveWrapper = uint32_t;

using TransparentPrimitiveWithAssociatedConstants = uint32_t;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ZERO = 0;
constexpr static const TransparentPrimitiveWithAssociatedConstants TransparentPrimitiveWithAssociatedConstants_ONE = 1;

using TransparentPointerWrappingStructure = const uint32_t*;

struct EnumStruct {
  enum class Tag {
    A,
  };

  struct A_Body {
    uint8_t only_field;
  };

  Tag tag;
  union {
    A_Body a;
  };
};

using TransparentEnumStruct = int32_t*;

constexpr static const TransparentPrimitiveWrappingStructure EnumWithAssociatedConstantInImpl_TEN = 10;

extern "C" {

void simple_root(int32_t n);

void root(TransparentComplexWrappingStructTuple a,
          TransparentPrimitiveWrappingStructTuple b,
          TransparentComplexWrappingStructure c,
          TransparentPrimitiveWrappingStructure d,
          TransparentComplexWrapper<int32_t> e,
          TransparentPrimitiveWrapper<int32_t> f,
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
          const Box<int32_t> *t,
          int32_t u,
          uint8_t v);

}  // extern "C"
