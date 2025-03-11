// Problem: Weekly Contest 423 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Function to return the minimum of two integers
int min(int a, int b) {
    return (a < b) ? a : b;
}

// Function to return the maximum of three integers
int max(int a, int b, int c) {
    int d = a > b ? a : b;  // Find the maximum of a and b
    return d > c ? d : c;    // Return the maximum of d and c
}

// Function to find the length of the longest increasing subarrays
int maxIncreasingSubarrays(int* nums, int numsSize) {
    int maxlen = 0;  // Variable to store the maximum length of subarrays
    int i = 1;       // Start from the second element
    int max1 = 1;    // Variable to store the length of the previous increasing subarray
    
    // Traverse the array
    while (i < numsSize) {
        int max2 = 1;  // Variable to store the length of the current increasing subarray
        // Find the length of the current increasing subarray
        while (i < numsSize && nums[i] > nums[i - 1]) {
            max2++;
            i++;
        }
        int temp = min(max1, max2);  // Find the minimum length between the previous and current subarray
        maxlen = max(maxlen, temp, max2 / 2);  // Update the maximum length based on the calculated values
        max1 = max2;  // Update the length of the previous subarray
        i++;  // Move to the next element
    }
    return maxlen;  // Return the maximum length found
}

int main() {
    int numsSize;
    
    // Input the size of the array
    scanf("%d", &numsSize);
    
    int nums[numsSize];
    
    // Input the array elements
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    
    // Call the function and display the result
    int result = maxIncreasingSubarrays(nums, numsSize);
    printf("%d\n", result);
    
    return 0;
}
