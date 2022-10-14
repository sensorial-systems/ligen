#include <stdio.h>
//#include <example/example.h>
//#include <example/functions/primitives.h>
#include <assert.h>

extern __declspec(dllimport) int example_functions_primitives_add(int a, int b);

int main(int argc, char **argv) {
    int a = 2, b = 3;
    printf("%d + %d = %d", a, b, example_functions_primitives_add(a, b));
    return 0;
}
