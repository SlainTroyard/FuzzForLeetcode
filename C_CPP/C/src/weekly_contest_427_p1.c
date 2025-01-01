// Problem: Weekly Contest 427 Problem 1
#include <stdio.h>
#include <stdlib.h>

// Function to construct the transformed array
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
                targetIndex += numsSize; // Handle negative wrapping
            }

            result[i] = nums[targetIndex];
        }
    }

    return result;
}

// Main function
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

    // Free allocated memory
    free(nums);
    free(transformedArray);

    return 0;
}
