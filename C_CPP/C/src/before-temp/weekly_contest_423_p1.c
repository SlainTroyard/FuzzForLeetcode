// Problem: Weekly Contest 423 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include <stdio.h>
#include <stdbool.h>

// Function to check if there are increasing subarrays
bool hasIncreasingSubarrays(int* nums, int numsSize, int k) {
    if (k == 1) {
        return true;  // Single element subarrays are always increasing
    }

    int a = 0;
    int s;
    for (int j = 0; j < 2; j++) {
        for (int i = 0; i < numsSize - 2 * k + 1; i++) {
            s = i + k;
            a = 0;
            // Check each pair within the subarray for increasing order
            for (int z = 0; z < k - 1; z++) {
                if (nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1]) {
                    a += 1;
                }
            }
            if (a == k - 1) {
                return true;  // Found a valid increasing subarray
            }
        }
    }
    return false;  // No valid increasing subarray found
}

int main() {
    int n, k;
    
    // Read the size of the array and the subarray length
    scanf("%d", &n);
    
    int nums[n];
    
    // Read the elements of the array
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    // Read the length of the subarray
    scanf("%d", &k);

    // Call the function to check for increasing subarrays
    if (hasIncreasingSubarrays(nums, n, k)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    return 0;
}
