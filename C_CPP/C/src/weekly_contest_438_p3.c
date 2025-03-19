// Problem: Weekly Contest 438 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>


// Compute C(n,k) mod 2. 
// Using the fact that C(n,k) mod 2 = 1 if and only if (n & k) == k.
int binom_mod2(int n, int k) {
    while(n || k) {
        if ((k & 1) > (n & 1))
            return 0;
        n >>= 1;
        k >>= 1;
    }
    return 1;
}

// Compute C(n,k) mod 5 using Lucas' theorem.
// We precompute a table for n, k in [0,4].
int binom_mod5(int n, int k) {
    static int table[5][5] = {
        {1, 0, 0, 0, 0},
        {1, 1, 0, 0, 0},
        {1, 2, 1, 0, 0},
        {1, 3, 3, 1, 0},
        {1, 4, 1, 4, 1}
    };
    int res = 1;
    while(n || k) {
        int n_i = n % 5;
        int k_i = k % 5;
        if (k_i > n_i)
            return 0;
        res = (res * table[n_i][k_i]) % 5;
        n /= 5;
        k /= 5;
    }
    return res;
}

// Combine the results modulo 2 and 5 to obtain mod 10.
// Let r2 be the result mod 2 (0 or 1) and r5 the result mod 5.
// We need x such that x ≡ r5 (mod 5) and x ≡ r2 (mod 2).
int combine_mod10(int r2, int r5) {
    // The two possible candidates are r5 and r5+5.
    // Since 5 ≡ 1 (mod 2), we have:
    //   (r5 + t*5) mod 2 = (r5 + t) mod 2.
    // So choose t = 0 if r5 mod2 == r2, otherwise t = 1.
    if ((r5 % 2) == r2)
        return r5 % 10;
    else
        return (r5 + 5) % 10;
}

// Compute C(n,k) mod 10 using Lucas' theorem on mod2 and mod5.
int binom_mod10(int n, int k) {
    int r2 = binom_mod2(n, k);
    int r5 = binom_mod5(n, k);
    return combine_mod10(r2, r5);
}

bool hasSameDigits(char* s) {
    int n = strlen(s);
    // We need to compute D = sum_{j=0}^{n-2} binom(n-2, j)*(a[j] - a[j+1]) mod 10.
    int D = 0;
    // Loop j from 0 to n-2 (since s[j+1] is defined for j in 0..n-2).
    for (int j = 0; j < n - 1; j++) {
        int weight = binom_mod10(n - 2, j); // weight = C(n-2, j) mod 10.
        // Convert characters to digits.
        int d1 = s[j] - '0';
        int d2 = s[j+1] - '0';
        int diff = d1 - d2;
        // Normalize diff into [0,9] mod 10.
        diff = (diff % 10 + 10) % 10;
        int contrib = (weight * diff) % 10;
        D = (D + contrib) % 10;
    }
    // The final two digits are equal if and only if D ≡ 0 (mod 10).
    return (D % 10 == 0);
}

int main() {
    // 读取输入
    char s[100001]; // 假设字符串最大长度为100000
    
    if (scanf("%s", s) != 1) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    // 调用函数计算结果
    bool result = hasSameDigits(s);
    
    // 输出结果
    printf("%s\n", result ? "true" : "false");
    
    return 0;
}
