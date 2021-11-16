/**
 * @file
 * @brief [Selection sort](https://en.wikipedia.org/wiki/Selection_sort)
 * algorithm implementation.
 */
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

/**
 * Swapped two numbers using pointer
 * @param first first pointer of first number
 * @param second second pointer of second number
 */
void swap(int *first, int *second)
{
    int temp = *first;
    *first = *second;
    *second = temp;
}

/**
 * Selection sort algorithm implements
 * @param arr array to be sorted
 * @param size size of array
 */
void selectionSort(int *arr, int size)
{
    for (int i = 0; i < size - 1; i++)
    {
        int min_index = i;
        for (int j = i + 1; j < size; j++)
        {
            if (arr[min_index] > arr[j])
            {
                min_index = j;
            }
        }
        if (min_index != i)
        {
            swap(arr + i, arr + min_index);
        }
    }
}

/** Test function
  * @returns None
  */
static void test(int len)
{
    if(len == 0) len = rand() % 500; /* random array size */
    const int size = len;
    int *arr = (int *)calloc(size, sizeof(int));

    /* generate size random numbers from -50 to 49 */
    for (int i = 0; i < size; i++)
    {
        arr[i] = (rand() % 10000) - 5000; /* signed random numbers */
    }
    printf("Array before sorting:\n[");
    for(int i = 0; i < size - 1; i++){
        printf("%d, ", arr[i]);
    }
    printf("%d]\n", arr[size - 1]);
    selectionSort(arr, size);
    printf("Array after sorting:\n[");
    for(int i = 0; i < size - 1; i++){
        printf("%d, ", arr[i]);
    }
    printf("%d]\n", arr[size - 1]);
    for (int i = 0; i < size - 1; ++i)
    {
        assert(arr[i] <= arr[i + 1]);
    }
    free(arr);
    printf("Test Passed\n");
}

/** Driver Code */
int main(int argc, const char *argv[])
{
    /* Intializes random number generator */
    srand(time(NULL));
    if(argc > 1) test(atoi(argv[1]));
    else test(0);
    return 0;
}