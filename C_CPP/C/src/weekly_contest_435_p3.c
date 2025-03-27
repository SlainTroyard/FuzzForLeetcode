// Problem: Weekly Contest 435 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

long long gcd(long long a, long long b) {
    while (b) {
        long long temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

long long min(long long a, long long b) {
    return a < b ? a : b;
}

int minimumIncrements(int* nums, int numsSize, int* target, int targetSize) {
    int n = numsSize;
    int m = targetSize;
    
    long long* g = (long long*)malloc((1 << m) * sizeof(long long));
    if (!g) return -1; 
    
    for (int i = 0; i < (1 << m); i++) {
        g[i] = 1;
        for (int j = 0; j < m; j++) {
            if ((i >> j) & 1) {
                g[i] = g[i] / gcd(g[i], target[j]) * target[j];
            }
        }
    }
    
    const long long INF = 1e18;
    long long** f = (long long**)malloc(2 * sizeof(long long*));
    if (!f) {
        free(g);
        return -1; 
    }
    
    for (int i = 0; i < 2; i++) {
        f[i] = (long long*)malloc((1 << m) * sizeof(long long));
        if (!f[i]) {
            if (i > 0) free(f[0]);
            free(f);
            free(g);
            return -1; 
        }
    }
    
    for (int j = 0; j < (1 << m); j++) {
        f[0][j] = INF;
    }
    f[0][0] = 0;
    
    for (int i = 1; i <= n; i++) {
        for (int j = 0; j < (1 << m); j++) {
            f[i & 1][j] = f[(i & 1) ^ 1][j];
        }
        
        for (int j = 0; j < (1 << m); j++) {
            for (int k = j; k > 0; k = (k - 1) & j) {
                long long v = ((nums[i - 1] + g[k] - 1) / g[k]) * g[k] - nums[i - 1];
                f[i & 1][j] = min(f[i & 1][j], f[(i & 1) ^ 1][j ^ k] + v);
            }
        }
    }
    
    int result = (int)f[n & 1][(1 << m) - 1];
    
    for (int i = 0; i < 2; i++) {
        free(f[i]);
    }
    free(f);
    free(g);
    
    return result;
}

int main() {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) {
        fprintf(stderr, "Error reading input for n and m\n");
        return 1;
    }
    
    int* nums = (int*)malloc(n * sizeof(int));
    int* target = (int*)malloc(m * sizeof(int));
    
    if (!nums || !target) {
        fprintf(stderr, "Memory allocation failed\n");
        if (nums) free(nums);
        if (target) free(target);
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &nums[i]) != 1) {
            fprintf(stderr, "Error reading input for nums[%d]\n", i);
            free(nums);
            free(target);
            return 1;
        }
    }
    
    for (int i = 0; i < m; i++) {
        if (scanf("%d", &target[i]) != 1) {
            fprintf(stderr, "Error reading input for target[%d]\n", i);
            free(nums);
            free(target);
            return 1;
        }
    }
    
    int result = minimumIncrements(nums, n, target, m);
    
    printf("%d\n", result);
    
    free(nums);
    free(target);
    
    return 0;
}
