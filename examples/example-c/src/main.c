#include <stdio.h>
#include <assert.h>
#include <example/functions/primitives.h>
#include <ligen_rust_runtime.h>

#define cassert(a) {\
    assert(a);\
    printf("%-64s... passed.\n", #a);\
}

int main(int argc, char **argv) {
    cassert(example_functions_primitives_add(1, 2) == 3);
    cassert(ligen_rust_runtime_test() == 123);
    printf("All tests passed.\n");
    return 0;
}
