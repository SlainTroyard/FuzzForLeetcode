// Problem: Weekly Contest 414 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

long long findMaximumScore(int* nums, int numsSize) {
    long long ans = 0LL, l = 0LL, r = 1LL;
    for (; r < numsSize; ++r)
        if (nums[l] < nums[r]) 
            ans += (r - l) * nums[l], l = r;
    return ans + (r - l - 1) * nums[l];
}

int main() {
    int numsSize;
    scanf("%d", &numsSize);
    int* nums = (int*)malloc(sizeof(int) * numsSize);
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%lld\n", findMaximumScore(nums, numsSize));
    free(nums);
    return 0;
}
