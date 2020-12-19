#include "array.h"

#include <stdio.h>
#include <stdlib.h>

int sum(const int *arr, size_t size)
{
    int total = 0;
    for (size_t i = 0; i < size; ++i)
    {
        total += arr[i];
    }
    return total;
}

int *get_sequential_array(size_t size)
{
    int *arr = (int *)malloc(size * sizeof(int));
    for (size_t i = 0; i < size; ++i)
    {
        arr[i] = i;
    }

    return arr;
}

void free_array(int *arr)
{
    free(arr);
    printf("free\n");
}
