// Problem: Weekly Contest 424 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int countValidSelections(int* nums, int numsSize) {
    int sumLeft = 0, sumRight = 0, ret = 0;
    for(int i = 0; i < numsSize; i++)
    sumRight += nums[i];
    for(int i = 0; i < numsSize; i++)
    {
        if(nums[i] != 0)
        {
            sumLeft += nums[i];
            sumRight -= nums[i];
        }
        else
        {
            if(sumLeft == sumRight)
            ret += 2;
            else if((int)abs(sumLeft - sumRight) == 1)
            ret++;
        }
    }
    return ret;
}

int main() {
    int n;

    scanf("%d", &n);
    
    int nums[n];

    for(int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }
    
    int result = countValidSelections(nums, n);
    printf("%d\n", result);
    
    return 0;
}
