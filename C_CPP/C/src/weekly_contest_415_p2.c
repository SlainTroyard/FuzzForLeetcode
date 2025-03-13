// Problem: Weekly Contest 415 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>
#include <math.h>

long long maxScore(int* a, int aSize, int* b, int bSize) {
    int n = bSize;
    long long f[bSize + 1][5];

    for(int i = 0; i <= bSize; i++) {
        for(int j = 0; j <= 4; j++) {
            f[i][j] = INT_MIN;
        }
    }
    f[0][0] = 0;
    for(int i = 1; i <= n; i++) {
        for(int j = 0; j <= 4; j++) {

            f[i][j] = f[i-1][j];
            if(j > 0) {
                f[i][j] = fmax(f[i][j], f[i-1][j-1] + 1LL * a[j-1] * b[i-1]);
            }
        }
    }
    return f[n][4];
}

int main() {
    int aSize, bSize;
    scanf("%d %d", &aSize, &bSize);
    int a[aSize], b[bSize];
    for(int i = 0; i < aSize; i++) {
        scanf("%d", &a[i]);
    }
    for(int i = 0; i < bSize; i++) {
        scanf("%d", &b[i]);
    }
    printf("%lld\n", maxScore(a, aSize, b, bSize));
    return 0;
}
