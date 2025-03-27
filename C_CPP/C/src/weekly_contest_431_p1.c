// Problem: Weekly Contest 431 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

long long gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

long long lcm(int a, int b) {
    return (a / gcd(a, b)) * b;
}

int maxLength(int* nums, int numsSize) {
    int maxLength = 0;
    for (int i = 0; i < numsSize; i++) {
        long long prod = 1, g = nums[i], l = nums[i];
        for (int j = i; j < numsSize; j++) {
            if (prod > LLONG_MAX / nums[j]) break; 
            prod *= nums[j];
            g = gcd(g, nums[j]);
            l = lcm(l, nums[j]);
            if (prod == l * g) {
                int length = j - i + 1;
                if (length > maxLength) {
                    maxLength = length;
                }
            }
        }
    }
    return maxLength;
}


int main() {
    int numSize;
    scanf("%d", &numSize);
    int* nums = (int*)malloc(numSize * sizeof(int));
    for (int i = 0; i < numSize; i++) {
        scanf("%d", &nums[i]);
    }

    int result = maxLength(nums, numSize);
    printf("%d\n", result);

    free(nums);
    return 0;
}
