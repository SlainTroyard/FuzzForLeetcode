// Problem: Weekly Contest 426 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int getLargestOutlier(int* nums, int numsSize) {
    int total_sum = 0;
    for(int i = 0; i < numsSize; i++) total_sum += nums[i];

    int set[2001];
    memset(&set, 0, sizeof(set));
    for(int i = 0; i < numsSize; i++) {
        set[nums[i] + 1000]++;
    }

    int curr_sum, threshold, half;
    int ans = -1001;
    for(int i = 0; i < numsSize; i++) {
        curr_sum = total_sum - nums[i];
        if((curr_sum & 1) == 0) { 
            half = curr_sum / 2;
            if(half == nums[i]) threshold = 1;
            else threshold = 0;

            if(half >= -1000 && half <= 1000) {
                if(set[half + 1000] > threshold) {
                    if(ans < nums[i]) ans = nums[i];
                }
            }
        }
    }
    return ans;
}

int main() {
    int numsSize;

    scanf("%d", &numsSize);

    int* nums = (int*)malloc(numsSize * sizeof(int));

    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }

    int result = getLargestOutlier(nums, numsSize);

    printf("%d\n", result);

    free(nums);
    return 0;
}