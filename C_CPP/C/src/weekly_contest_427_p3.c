// Problem: Weekly Contest 427 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

long long maxValue(long long a, long long b) {
    return (a > b) ? a : b;
}

long long maxSubarraySum(int* nums, int n, int k) {
    long long cur = 0, keep[n - k + 1];

    for (int i = 0; i < n; i++) {
        cur += nums[i];
        if (i >= k - 1) {
            keep[i - k + 1] = cur;
            cur -= nums[i - k + 1];
        }
    }

    long long ans = LONG_MIN, max;

    for (int i = 0; i < k && i < n - k + 1; i++) {
        cur = 0;
        max = keep[i];

        for (int l = i; l < n - k + 1; l += k) {
            cur += keep[l];

            if (cur > max) max = cur;
            if (cur < 0) cur = 0;
        }
        ans = maxValue(ans, max);
    }
    return ans;
}

int main() {
    int n, k;

    scanf("%d", &n);

    scanf("%d", &k);

    int nums[n];

    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    long long result = maxSubarraySum(nums, n, k);
    printf("%lld\n", result);

    return 0;
}
