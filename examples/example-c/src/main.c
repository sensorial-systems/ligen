#include <stdio.h>
#include <assert.h>
#include <example/functions/primitives.h>
#include <ligen_rust_runtime.h>

#define cassert(a) {\
    assert(a);\
    printf("%-64s... passed.\n", #a);\
}

int main(int argc, char **argv) {
    cassert(example_functions_primitives_add_i8 (1, -2) == -1);
    cassert(example_functions_primitives_add_i16(1, -2) == -1);
    cassert(example_functions_primitives_add_i32(1, -2) == -1);
    cassert(example_functions_primitives_add_i64(1, -2) == -1);
    cassert(example_functions_primitives_add_u8 (1,  2) ==  3);
    cassert(example_functions_primitives_add_u16(1,  2) ==  3);
    cassert(example_functions_primitives_add_u32(1,  2) ==  3);
    cassert(example_functions_primitives_add_u64(1,  2) ==  3);

    cassert(example_functions_primitives_add_f32(1.0f, 2.0f) == 3.0f);
    cassert(example_functions_primitives_add_f64(1.0 , 2.0 ) == 3.0);

    cassert(ligen_rust_runtime_test() == 123);
    printf("All tests passed.\n");
    return 0;
}
