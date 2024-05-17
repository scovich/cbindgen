#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef int16_t Foo_i16;

typedef int32_t Foo_i32;

struct Bar_i32__u32 {
  Foo_i32 f;
  uint32_t p;
};

typedef int64_t Foo_i64;

typedef Foo_i64 Baz_i64;

void foo_root(Foo_i16 f, struct Bar_i32__u32 b, Baz_i64 z);
