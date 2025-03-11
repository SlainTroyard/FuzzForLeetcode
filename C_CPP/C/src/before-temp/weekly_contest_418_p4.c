// Problem: Weekly Contest 418 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
 #define MAX_SIZE 50001

int cmp(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}
int* gcdValues(int* nums, int numsSize, long long* queries, int queriesSize, int* returnSize) {
    int mx = 0;

    for (int i = 0; i < numsSize; i++) {
        if (nums[i] > mx) {
            mx = nums[i];
        }
    }

    int* cnt_x = (int*)calloc(mx + 1, sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        cnt_x[nums[i]]++;
    }

    long long* cnt_gcd = (long long*)calloc(mx + 1, sizeof(long long));
    for (int i = mx; i > 0; i--) {
        int c = 0;
        for (int j = i; j <= mx; j += i) {
            c += cnt_x[j];
            cnt_gcd[i] -= cnt_gcd[j];
        }
        cnt_gcd[i] += (long long)c * (c - 1) / 2;
    }

    for (int i = 1; i <= mx; i++) {
        cnt_gcd[i] += cnt_gcd[i - 1];
    }

    int* ans = (int*)malloc(sizeof(int) * queriesSize);
    for (int i = 0; i < queriesSize; i++) {
        long long query = queries[i];
        int left = 1, right = mx;
        while (left < right) {
            int mid = (left + right) / 2;
            if (cnt_gcd[mid] <= query) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        ans[i] = left;
    }

    *returnSize = queriesSize;
    free(cnt_x);
    free(cnt_gcd);
    return ans;
}

int main() {
    int numsSize;
    scanf("%d", &numsSize);
    int* nums = (int*)malloc(sizeof(int) * numsSize);
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    int queriesSize;
    scanf("%d", &queriesSize);
    long long* queries = (long long*)malloc(sizeof(long long) * queriesSize);
    for (int i = 0; i < queriesSize; i++) {
        scanf("%lld", &queries[i]);
    }
    int returnSize;
    int* ans = gcdValues(nums, numsSize, queries, queriesSize, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", ans[i]);
    }
    printf("\n");
    free(nums);
    free(queries);
    free(ans);
    return 0;
}
