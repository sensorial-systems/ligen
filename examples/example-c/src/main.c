#include <stdio.h>
//#include <example/example.h>
//#include <example/functions/primitives.h>
#include <assert.h>

extern int example_add(int a, int b);

int main(int argc, char **argv) {
    int a = 2;
    int b = 3;
    printf("%d + %d = %d", a, b, example_add(a, b));
    return 0;
}
