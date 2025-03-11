// Problem: Weekly Contest 429 Problem 2
#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int cmp_int_asc(const void *a, const void *b) {
    return (*(int*)a - *(int*)b);
}

int maxDistinctElements(int* arr, int arrSize, int diff) {
    int prev = INT_MIN;
    int distinctCount = 0;

    qsort(arr, arrSize, sizeof(int), cmp_int_asc);

    for (int i = 0; i < arrSize; i++) {
        int x = (prev + 1 > arr[i] - diff) ? prev + 1 : arr[i] - diff;

        if (x <= arr[i] + diff) {
            distinctCount++;
            prev = x;
        }
    }

    return distinctCount;
}

int main() {
    int n, diff;

    // Input array size and difference
    scanf("%d", &n);

    scanf("%d", &diff);

    // Allocate memory for the array
    int* arr = (int*)malloc(n * sizeof(int));

    // Input array elements
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }

    // Compute the result
    int result = maxDistinctElements(arr, n, diff);

    // Output the result
    printf("%d\n", result);

    // Free allocated memory
    free(arr);

    return 0;
}
