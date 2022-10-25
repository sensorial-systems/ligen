#pragma once

#include <assert.h>

#define cassert(a) {\
    assert(a);\
    printf("Assertion succeeded: %s\n" , #a);\
}
