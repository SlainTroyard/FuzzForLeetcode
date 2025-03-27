// Problem: Weekly Contest 434 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

int max(int a, int b) {
    return a > b ? a : b;
}

int maxFrequency(int* nums, int numsSize, int k) {
    int f0 = 0;               
    int f1[51] = {0};         
    int max_f1 = 0;           
    int f2 = 0;               
    
    for (int i = 0; i < numsSize; i++) {
        int x = nums[i];
        
        f2 = max(f2, max_f1) + (x == k);
        
        f1[x] = max(f1[x], f0) + 1;
        
        f0 += (x == k);
        
        max_f1 = max(max_f1, f1[x]);
    }
    
    return max(max_f1, f2);
}

int main() {
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        fprintf(stderr, "Error reading input for n and k\n");
        return 1;
    }
    
    int* nums = (int*)malloc(n * sizeof(int));
    if (!nums) {
        fprintf(stderr, "Memory allocation failed for nums array\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &nums[i]) != 1) {
            fprintf(stderr, "Error reading input for nums[%d]\n", i);
            free(nums);
            return 1;
        }
    }
    
    int result = maxFrequency(nums, n, k);
    
    printf("%d\n", result);
    
    free(nums);
    
    return 0;
}
