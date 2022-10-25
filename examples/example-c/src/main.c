#include <stdio.h>
#include <ctest.h>

#include <example/functions/primitives.h>
#include <ligen_rust_runtime.h>

void example_functions_primitives_tests() {
    cassert(example_functions_primitives_add_i8 (1   , -2   ) == -1   );
    cassert(example_functions_primitives_add_i16(1   , -2   ) == -1   );
    cassert(example_functions_primitives_add_i32(1   , -2   ) == -1   );
    cassert(example_functions_primitives_add_i64(1   , -2   ) == -1   );
    cassert(example_functions_primitives_add_u8 (1   ,  2   ) ==  3   );
    cassert(example_functions_primitives_add_u16(1   ,  2   ) ==  3   );
    cassert(example_functions_primitives_add_u32(1   ,  2   ) ==  3   );
    cassert(example_functions_primitives_add_u64(1   ,  2   ) ==  3   );
    cassert(example_functions_primitives_add_f32(1.0f,  2.0f) ==  3.0f);
    cassert(example_functions_primitives_add_f64(1.0 ,  2.0 ) ==  3.0 );

    int8_t   i8  = -3 ;
    int16_t  i16 = -5 ;
    int32_t  i32 = -32;
    int64_t  i64 = -64;
    uint8_t  u8  =  3 ;
    uint16_t u16 =  5 ;
    uint32_t u32 =  32;
    uint64_t u64 =  64;
    example_functions_primitives_double_i8 (&i8 );
    example_functions_primitives_double_i16(&i16);
    example_functions_primitives_double_i32(&i32);
    example_functions_primitives_double_i64(&i64);

    example_functions_primitives_double_u8 (&u8 );
    example_functions_primitives_double_u16(&u16);
    example_functions_primitives_double_u32(&u32);
    example_functions_primitives_double_u64(&u64);

    cassert(i8  == -6  );
    cassert(i16 == -10 );
    cassert(i32 == -64 );
    cassert(i64 == -128);
    cassert(u8  ==  6  );
    cassert(u16 ==  10 );
    cassert(u32 ==  64 );
    cassert(u64 ==  128);
}

int main(int argc, char **argv) {
    example_functions_primitives_tests();
    cassert(ligen_rust_runtime_test() == 123);
    printf("All tests passed.\n");
    return 0;
}
