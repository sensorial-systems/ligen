#include <stdio.h>
#include <assert.h>
#include <example/functions/primitives.h>
#include <rust.h>
#include <ligen_rust_runtime.h>

int main(int argc, char **argv) {
    assert(example_functions_primitives_add(1, 2) == 3);
    assert(ligen_rust_runtime_test() == 123);
    printf("All tests passed.\n");
    return 0;
}
