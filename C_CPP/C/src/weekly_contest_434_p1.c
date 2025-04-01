// Problem: Weekly Contest 434 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int countPartitions(int* nums, int numsSize) {
    int count = 0;
    for (int i = 0; i < numsSize - 1; i++) {
        int leftSum = 0, rightSum = 0;
        for (int j = 0; j <= i; j++) {
            leftSum += nums[j];
        }
        for (int j = i + 1; j < numsSize; j++) {
            rightSum += nums[j];
        }
        if ((leftSum - rightSum) % 2 == 0) {
            count++;
        }
    }
    return count;
}

int main() {
    
    int n;
    scanf("%d", &n);
    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%d\n", countPartitions(nums, n));
    free(nums);
    return 0;
}
