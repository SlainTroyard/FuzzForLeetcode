// Problem: Weekly Contest 431 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

// TODO: Add your function declaration here
// Example:
// bool solutionFunction(type1 param1, type2 param2) {
//     // Implementation
// }
// 计算两个数的最大公因数
long long gcd(int a, int b) {
    while (b != 0) {
        int temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

// 计算两个数的最小公倍数
long long lcm(int a, int b) {
    return (a / gcd(a, b)) * b;
}

// 返回最长乘积等价子数组的长度
int maxLength(int* nums, int numsSize) {
    int maxLength = 0;
    for (int i = 0; i < numsSize; i++) {
        long long prod = 1, g = nums[i], l = nums[i];
        for (int j = i; j < numsSize; j++) {
            if (prod > LLONG_MAX / nums[j]) break; // 防止溢出
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
    // TODO: Add the base I/O interface here
    int numSize;
    scanf("%d", &numSize);
    int* nums = (int*)malloc(numSize * sizeof(int));
    for (int i = 0; i < numSize; i++) {
        scanf("%d", &nums[i]);
    }

    // Calculate the result
    int result = maxLength(nums, numSize);
    printf("%d\n", result);

    free(nums);
    return 0;
}
