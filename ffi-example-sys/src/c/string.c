#include "string.h"

#include <stdio.h>

void print_str(const char *str)
{
    printf("From C: %s\n", str);
}

const char *hello()
{
    return "Hello FFI";
}
