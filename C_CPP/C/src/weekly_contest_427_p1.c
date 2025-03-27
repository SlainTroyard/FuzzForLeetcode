// Problem: Weekly Contest 427 Problem 1
#include <stdio.h>
#include <stdlib.h>

int* constructTransformedArray(int* nums, int numsSize, int* returnSize) {
    int *result = (int *)malloc(numsSize * sizeof(int));
    *returnSize = numsSize;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] == 0) {
            result[i] = nums[i];
        } else {
            int steps = nums[i];
            int targetIndex = (i + steps) % numsSize;

            if (targetIndex < 0) {
                targetIndex += numsSize; 
            }

            result[i] = nums[targetIndex];
        }
    }

    return result;
}

int main() {
    int numsSize;
    scanf("%d", &numsSize);

    int* nums = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }

    int returnSize;
    int* transformedArray = constructTransformedArray(nums, numsSize, &returnSize);

    for (int i = 0; i < returnSize; i++) {
        printf("%d ", transformedArray[i]);
    }
    printf("\n");

    free(nums);
    free(transformedArray);

    return 0;
}
