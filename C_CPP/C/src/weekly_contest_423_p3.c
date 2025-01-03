// Problem: Weekly Contest 423 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int sumOfGoodSubsequences(int* nums, int numsSize) {
    long long c, s, ans = 0LL;
    for (int i = 0, x, cnt[100003] = {0}, sum[100003] = {0}; i < numsSize; ++i) 
        x = ++nums[i], 
        cnt[x] = (cnt[x] + (c = cnt[x - 1] + 1 + cnt[x + 1])) % 1000000007, 
        sum[x] = (sum[x] + (s = c * (x - 1) + sum[x - 1] + sum[x + 1])) % 1000000007,
        ans = (ans + s) % 1000000007;
    return ans;
}

int main() {
    // Read the size of the array
    int n;
    scanf("%d", &n);
     
    // Dynamically allocate memory for the array using malloc
    int* nums = (int*) malloc(n * sizeof(int));
    if (nums == NULL) { // Check for memory allocation failure
        printf("Memory allocation failed!\n");
        return 1; // Exit if malloc fails
    }
    
    // Read the elements of the array
    for (int i = 0; i < n; ++i) {
        scanf("%d", &nums[i]);
    }
    
    // Call the function and output the result
    int result = sumOfGoodSubsequences(nums, n);
    printf("%d\n", result);
    
    // Free the dynamically allocated memory
    free(nums);
    
    return 0;
}