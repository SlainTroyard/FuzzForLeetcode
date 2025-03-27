// Problem: Weekly Contest 430 Problem 4
#include <stdio.h>
#define MOD 1000000007

long long mult(long long n, long long m) {
    return (n * m) % MOD;
}

long long power(long long n, long long m) {
    long long res = 1;
    long long base = n;
    while (m > 0) {
        if (m & 1) {
            res = mult(res, base);
        }
        base = mult(base, base);
        m >>= 1;
    }
    return res;
}

long long inv(long long n) {
    return power(n, MOD - 2);
}

long long factorial(long long n) {
    long long res = 1;
    for (long long i = 2; i <= n; ++i) {
        res = mult(res, i);
    }
    return res;
}

long long comb(long long n, long long m) {
    return mult(factorial(n), inv(mult(factorial(m), factorial(n - m))));
}

long long countGoodArrays(int n, int m, int k) {
    return mult(mult(comb(n - 1, n - 1 - k), m), power(m - 1, n - 1 - k));
}

int main() {
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);

    long long result = countGoodArrays(n, m, k);

    printf("%lld\n", result);

    return 0;
}
