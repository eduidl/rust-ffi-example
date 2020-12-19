#pragma once

#include <stddef.h>

int sum(const int *arr, size_t size);

int *get_sequential_array(size_t size);

void free_array(int *arr);
