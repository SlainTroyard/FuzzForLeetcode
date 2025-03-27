// Problem: Weekly Contest 423 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int min(int a, int b) {
    return (a < b) ? a : b;
}

int max(int a, int b, int c) {
    int d = a > b ? a : b;  
    return d > c ? d : c;    
}

int maxIncreasingSubarrays(int* nums, int numsSize) {
    int maxlen = 0;  
    int i = 1;       
    int max1 = 1;    
    
    while (i < numsSize) {
        int max2 = 1;  
        while (i < numsSize && nums[i] > nums[i - 1]) {
            max2++;
            i++;
        }
        int temp = min(max1, max2);  
        maxlen = max(maxlen, temp, max2 / 2);  
        max1 = max2;  
        i++;  
    }
    return maxlen;  
}

int main() {
    int numsSize;
    
    scanf("%d", &numsSize);
    
    int nums[numsSize];
    
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    
    int result = maxIncreasingSubarrays(nums, numsSize);
    printf("%d\n", result);
    
    return 0;
}
