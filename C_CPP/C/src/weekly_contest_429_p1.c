// Problem: Weekly Contest 429 Problem 1
#include <stdio.h>
#include <stdlib.h>

int minimumOperations(int* nums, int numsSize) {
    int count[101] = {0};
    for (int i = numsSize - 1; i >= 0; i--) {
        count[nums[i]]++;
        if (count[nums[i]] > 1) {
            return (i + 3) / 3;
        }
    }
    return 0;
}

int main() {
    int n;
    scanf("%d", &n);

    int* nums = (int*)malloc(n * sizeof(int));

    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    int result = minimumOperations(nums, n);
    printf("%d\n", result);

    free(nums);
    return 0;
}
