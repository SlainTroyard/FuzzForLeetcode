// Problem: Weekly Contest 424 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

// Function to calculate the minimum difference as in the C++ solution
int minDifference(int* nums, int n) {
    int min_l = INT_MAX, max_r = 0;
    
    // Find min_l and max_r by checking for the adjacent '-1' elements
    for (int i = 0; i < n; i++) {
        if (nums[i] != -1 && 
            ((i > 0 && nums[i - 1] == -1) || (i < n - 1 && nums[i + 1] == -1))) {
            if (nums[i] < min_l) min_l = nums[i];
            if (nums[i] > max_r) max_r = nums[i];
        }
    }

    int ans = 0;

    // Helper function to update the result (same logic as the C++ lambda function)
    void update_ans(int l, int r, int big) {
        int d = (r - min_l < max_r - l ? r - min_l : max_r - l) + 1;
        d /= 2;
        if (big) {
            d = d < (max_r - min_l + 2) / 3 ? d : (max_r - min_l + 2) / 3;
        }
        if (d > ans) ans = d;
    }

    // Calculate the answer by iterating through the elements
    int pre_i = -1;
    for (int i = 0; i < n; i++) {
        if (nums[i] == -1) {
            continue;
        }
        if (pre_i >= 0) {
            if (i - pre_i == 1) {
                int diff = abs(nums[i] - nums[pre_i]);
                if (diff > ans) ans = diff;
            } else {
                update_ans((nums[pre_i] < nums[i] ? nums[pre_i] : nums[i]),
                           (nums[pre_i] > nums[i] ? nums[pre_i] : nums[i]),
                           i - pre_i > 2);
            }
        } else if (i > 0) {
            update_ans(nums[i], nums[i], 0);
        }
        pre_i = i;
    }

    if (0 <= pre_i && pre_i < n - 1) {
        update_ans(nums[pre_i], nums[pre_i], 0);
    }

    return ans;
}

int main() {
    int n;
    scanf("%d", &n);

    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; ++i) {
        scanf("%d", &nums[i]);
    }

    int result = minDifference(nums, n);
    printf("%d\n", result);

    free(nums);
    return 0;
}
