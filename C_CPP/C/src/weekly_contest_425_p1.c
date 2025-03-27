// Problem: Weekly Contest 425 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int minimumSumSubarray(int* nums, int numsSize, int l, int r) {
    int i,j,sum,minSum='\0',count=0;
    for (l;l<=r;l++) {
        sum=0;
        for (i=0;i<numsSize;i++) {
            sum+=nums[i];
            if (i>=l) {
                sum-=nums[i-l];
            }
            if (sum>0 && i>=l-1) {
                if (minSum=='\0' || minSum>sum) {
                    minSum=sum;
                }
            }
        }
    }
    return minSum=='\0'? -1 : minSum;
}

int main() {
    int numsSize, l, r;

    scanf("%d", &numsSize);

    int* nums = (int*)malloc(numsSize * sizeof(int));

    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }

    scanf("%d %d", &l, &r);

    int result = minimumSumSubarray(nums, numsSize, l, r);

    printf("%d\n", result);

    free(nums);

    return 0;
}
