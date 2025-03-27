// Problem: Weekly Contest 423 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#include <stdio.h>
#include <stdbool.h>

bool hasIncreasingSubarrays(int* nums, int numsSize, int k) {
    if (k == 1) {
        return true;  
    }

    int a = 0;
    int s;
    for (int j = 0; j < 2; j++) {
        for (int i = 0; i < numsSize - 2 * k + 1; i++) {
            s = i + k;
            a = 0;
            for (int z = 0; z < k - 1; z++) {
                if (nums[i + z] < nums[i + z + 1] && nums[s + z] < nums[s + z + 1]) {
                    a += 1;
                }
            }
            if (a == k - 1) {
                return true;  
            }
        }
    }
    return false;  
}

int main() {
    int n, k;
    
    scanf("%d", &n);
    
    int nums[n];
    
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    scanf("%d", &k);

    if (hasIncreasingSubarrays(nums, n, k)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    return 0;
}
