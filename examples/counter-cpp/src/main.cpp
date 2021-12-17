#include <Counter.hpp>
#include <RString.hpp>
#include <Person.hpp>

#include <iostream>
#include <assert.h>
#include <string.h>

#define assert_eq(a, b) { printf("assert(%d == %d)\n", a, b); assert(a == b); }
#define string_assert_eq(a, b) { printf("assert(\"%s\" == \"%s\")\n", a, b); assert(!strcmp(a, b)); }

int main(int argc, char **argv) {
    Counter counter(2);
    assert_eq(counter.get_count(), 2);
    counter.count(1);
    assert_eq(counter.get_count(), 3);
    counter.count(3);
    assert_eq(counter.get_count(), 6);

    RString string("Hello!");
    string_assert_eq("Hello!", string.as_ptr());

    Person person("Danilo", "Guanabara");

    RString fullName = person.full_name();
    string_assert_eq("Danilo Guanabara", fullName.as_ptr());

    return 0;
}
