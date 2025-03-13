// Problem: Weekly Contest 421 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int gcd(int a, int b) {
    while (b) {
        int temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

int subsequencePairCount(int* nums, int numsSize) {
    const int MOD = 1000000007;
    const int MX = 201;

    static int initialized = 0;
    static int lcms[201][201];
    static int pow2[201];
    static int pow3[201];
    static int mu[201];

    if (!initialized) {
        // Initialize lcms
        for (int i = 1; i < MX; i++) {
            for (int j = 1; j < MX; j++) {
                int g = gcd(i, j);
                lcms[i][j] = (i * j) / g;
            }
        }
        // Initialize pow2 and pow3
        pow2[0] = 1;
        pow3[0] = 1;
        for (int i = 1; i < MX; i++) {
            pow2[i] = (pow2[i - 1] * 2) % MOD;
            pow3[i] = (pow3[i - 1] * 3LL) % MOD;
        }
        // Initialize mu
        memset(mu, 0, sizeof(mu));
        mu[1] = 1;
        for (int i = 1; i < MX; i++) {
            for (int j = 2 * i; j < MX; j += i) {
                mu[j] -= mu[i];
            }
        }
        initialized = 1;
    }

    // Find maximum value in nums
    int m = 0;
    for (int i = 0; i < numsSize; i++) {
        if (nums[i] > m) m = nums[i];
    }

    // Count occurrences and their multiples
    int* cnt = (int*)calloc(m + 1, sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        cnt[nums[i]]++;
    }
    for (int i = 1; i <= m; i++) {
        for (int j = 2 * i; j <= m; j += i) {
            cnt[i] += cnt[j];
        }
    }

    // Initialize f array
    int** f = (int**)malloc((m + 1) * sizeof(int*));
    for (int i = 0; i <= m; i++) {
        f[i] = (int*)malloc((m + 1) * sizeof(int));
    }

    // Fill f array
    for (int g1 = 1; g1 <= m; g1++) {
        for (int g2 = 1; g2 <= m; g2++) {
            int l = lcms[g1][g2];
            int c = (l <= m) ? cnt[l] : 0;
            int c1 = cnt[g1];
            int c2 = cnt[g2];
            long long term1 = (long long)pow3[c] * pow2[c1 + c2 - 2 * c] % MOD;
            long long term2 = (term1 - pow2[c1] - pow2[c2] + 1) % MOD;
            f[g1][g2] = (int)((term2 + MOD) % MOD);
        }
    }

    // Calculate answer using inclusion-exclusion
    long long ans = 0;
    for (int i = 1; i <= m; i++) {
        int max_jk = m / i;
        for (int j = 1; j <= max_jk; j++) {
            for (int k = 1; k <= max_jk; k++) {
                int gj = j * i;
                int gk = k * i;
                ans += (long long)mu[j] * mu[k] * f[gj][gk];
            }
        }
    }
    ans = (ans % MOD + MOD) % MOD;

    // Free allocated memory
    free(cnt);
    for (int i = 0; i <= m; i++) {
        free(f[i]);
    }
    free(f);

    return (int)ans;
}

int main() {
    int n;
    scanf("%d", &n);
    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }
    int result = subsequencePairCount(nums, n);
    printf("%d\n", result);
    free(nums);
    return 0;
}
