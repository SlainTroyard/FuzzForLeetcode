// Problem: Weekly Contest 420 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>
#include <limits.h>

int minOperations(int* nums, int numsSize) {
    int res = 0;
    int max_factor = 0;
    int j = 0;

    if(1 == numsSize)
    {
        return res;
    }

    for(int i = numsSize - 2; i >= 0; --i)
    {
        if(nums[i] > nums[i + 1])
        {
            int max = INT_MIN;
            int count = 1;
            int original = nums[i];
            while(1)
            {
                max = INT_MIN;
                max_factor = sqrt(nums[i]) + 1;
                for(j = 2; j <= max_factor; ++j)
                {
                    if(!(nums[i] % j))
                    {
                        max = max > j ? max : j;
                        if(!(nums[i] % (nums[i] / j)))
                        {
                            max = max > (nums[i] / j) ? max : (nums[i] / j);
                        }
                    }
                }
                if(max == INT_MIN || (count *= max) == original)
                {
                    return -1;
                }
                else
                {
                    nums[i] /= max;
                    if(nums[i] <= nums[i + 1])
                    {
                        ++res;
                        break;
                    }
                }
            }
        }
    }

    return res;
}

int main() {
    int numsSize = 0;
    scanf("%d", &numsSize);
    int* nums = (int*)malloc(sizeof(int) * numsSize);
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    printf("%d\n", minOperations(nums, numsSize));
    free(nums);
    return 0;
}
