// Problem: Weekly Contest 433 Problem 2
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define MOD 1000000007
#define MX 100000

long long F[MX]; 
long long INV_F[MX]; 

int compare(const void* a, const void* b) {
    return (*(int*)a - *(int*)b);
}

long long power(long long x, int n) {
    long long res = 1;
    while (n > 0) {
        if (n % 2 == 1) {
            res = (res * x) % MOD;
        }
        x = (x * x) % MOD;
        n /= 2;
    }
    return res;
}

long long comb(int n, int m) {
    if (m > n) return 0;
    return (((F[n] * INV_F[m]) % MOD) * INV_F[n - m]) % MOD;
}

void initialize() {
    F[0] = 1;
    for (int i = 1; i < MX; i++) {
        F[i] = (F[i - 1] * i) % MOD;
    }

    INV_F[MX - 1] = power(F[MX - 1], MOD - 2);
    for (int i = MX - 1; i > 0; i--) {
        INV_F[i - 1] = (INV_F[i] * i) % MOD;
    }
}

int minMaxSums(int* nums, int numsSize, int k) {
    static bool initialized = false;
    if (!initialized) {
        initialize();
        initialized = true;
    }
    
    qsort(nums, numsSize, sizeof(int), compare);
    
    long long ans = 0, s = 1;
    for (int i = 0; i < numsSize; i++) {
        ans = (ans + s * (nums[i] + nums[numsSize - 1 - i])) % MOD;
        s = (s * 2 - comb(i, k - 1) + MOD) % MOD;
    }
    
    return (int)ans;
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
    
    int result = minMaxSums(nums, n, k);
    
    printf("%d\n", result);
    
    free(nums);
    
    return 0;
}
