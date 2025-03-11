// Problem: Weekly Contest 415 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int* getSneakyNumbers(int* nums, int numsSize, int* returnSize) {
    int* result = (int*)calloc(2, sizeof(int));
    int count = 0;
    for (int i = 0; i < numsSize; i++) {
        for (int j = i + 1; j < numsSize; j++) {
            if (nums[i] == nums[j]) {
                result[count] = nums[i];
                count++;
                break;
            }
        }
        if (count == 2)
            break;
    }
    *returnSize=2;
    return result;
}

int main() {
    int numSize;
    scanf("%d", &numSize);
    numSize += 2;
    int* nums = (int*)calloc(numSize, sizeof(int));
    for (int i = 0; i < numSize; i++) {
        scanf("%d", &nums[i]);
    }
    int returnSize;
    int* result = getSneakyNumbers(nums, numSize, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", result[i]);
    }
    free(nums);
    free(result);
    return 0;
}
