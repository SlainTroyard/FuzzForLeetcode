// Problem: Weekly Contest 424 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool isZeroArray(int* nums, int numsSize, int** queries, int queriesSize, int* queriesColSize) {
    int i, j;
    int *diff = calloc(numsSize, sizeof(int));
    int count = 0;
    int r;

    for (i = 0; i < queriesSize; i++)
    {
        diff[queries[i][0]]++;
        r = queries[i][1];
        if ((r+1) < numsSize)
        {
            diff[r+1]--;
        }
    }
    for (i = 0; i < numsSize; i++)
    {
        count += diff[i];
        if (nums[i] > count)
        {
            free(diff);
            return false;
        }
    }
    free(diff);
    return true;
}

int main() {
    int numsSize, queriesSize;
    
    // Read input for nums array size and elements
    scanf("%d", &numsSize);
    int* nums = malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }

    // Read input for queries size and elements
    scanf("%d", &queriesSize);
    int** queries = malloc(queriesSize * sizeof(int*));
    int* queriesColSize = malloc(queriesSize * sizeof(int));

    for (int i = 0; i < queriesSize; i++) {
        queries[i] = malloc(2 * sizeof(int));  // Each query contains two integers
        scanf("%d %d", &queries[i][0], &queries[i][1]);
        queriesColSize[i] = 2;  // Each query has exactly two elements
    }

    // Call the function to check if the array can be zeroed
    if (isZeroArray(nums, numsSize, queries, queriesSize, queriesColSize)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    // Free allocated memory
    free(nums);
    for (int i = 0; i < queriesSize; i++) {
        free(queries[i]);
    }
    free(queries);
    free(queriesColSize);

    return 0;
}