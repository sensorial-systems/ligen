#include <stdio.h>
//#include <example/example.h>
//#include <example/functions/primitives.h>
#include <assert.h>

#include <example/functions/primitives.h>

int main(int argc, char **argv) {
    int a = 2, b = 3;
    printf("%d + %d = %d", a, b, example_functions_primitives_add(a, b));
    return 0;
}
