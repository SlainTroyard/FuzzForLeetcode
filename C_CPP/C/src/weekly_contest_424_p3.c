// Problem: Weekly Contest 424 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int minZeroArray(int* nums, int numsSize, int** queries, int queriesSize, int* queriesColSize) {
    if (!nums || numsSize == 0)
        return 0;

    if (!queries || queriesSize == 0)
        return -1;

    int* max = (int*)malloc(sizeof(int) * (numsSize+1));
    memset(max, 0, sizeof(int) * numsSize);

    int sum = 0;
    int k = 0;
    for (int i = 0; i < numsSize; i++) {
        while (sum + max[i] < nums[i]) {
            if (k == queriesSize)
                return -1;

            int start = queries[k][0];
            int end = queries[k][1];
            int increment = queries[k][2];
            k++;

            if (i > end)
                continue;

            if (i > start)
                max[i] += increment;
            else
                max[start] += increment;
            max[end+1] -= increment;
        }
        sum = sum + max[i];
    }

    return k;
}

int main() {
    int numsSize;
    scanf("%d", &numsSize);

    int* nums = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }

    int queriesSize;
    scanf("%d", &queriesSize);

    int** queries = (int**)malloc(queriesSize * sizeof(int*));
    int* queriesColSize = (int*)malloc(queriesSize * sizeof(int));

    for (int i = 0; i < queriesSize; i++) {
        queries[i] = (int*)malloc(3 * sizeof(int));
        for (int j = 0; j < 3; j++) {
            scanf("%d", &queries[i][j]);
        }
        queriesColSize[i] = 3;
    }

    int result = minZeroArray(nums, numsSize, queries, queriesSize, queriesColSize);

    printf("%d\n", result);

    free(nums);
    for (int i = 0; i < queriesSize; i++) {
        free(queries[i]);
    }
    free(queries);
    free(queriesColSize);

    return 0;
}
