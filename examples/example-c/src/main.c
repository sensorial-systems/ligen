#include <stdio.h>
#include <assert.h>
#include <example/functions/primitives.h>

int main(int argc, char **argv) {
    assert(example_functions_primitives_add(1, 2) == 3);
    return 0;
}
