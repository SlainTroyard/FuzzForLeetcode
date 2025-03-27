// Problem: Weekly Contest 438 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>


int binom_mod2(int n, int k) {
    while(n || k) {
        if ((k & 1) > (n & 1))
            return 0;
        n >>= 1;
        k >>= 1;
    }
    return 1;
}

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

int combine_mod10(int r2, int r5) {
    if ((r5 % 2) == r2)
        return r5 % 10;
    else
        return (r5 + 5) % 10;
}

int binom_mod10(int n, int k) {
    int r2 = binom_mod2(n, k);
    int r5 = binom_mod5(n, k);
    return combine_mod10(r2, r5);
}

bool hasSameDigits(char* s) {
    int n = strlen(s);
    int D = 0;
    for (int j = 0; j < n - 1; j++) {
        int weight = binom_mod10(n - 2, j); 
        int d1 = s[j] - '0';
        int d2 = s[j+1] - '0';
        int diff = d1 - d2;
        diff = (diff % 10 + 10) % 10;
        int contrib = (weight * diff) % 10;
        D = (D + contrib) % 10;
    }
    return (D % 10 == 0);
}

int main() {
    char s[100001]; 
    
    if (scanf("%s", s) != 1) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    bool result = hasSameDigits(s);
    
    printf("%s\n", result ? "true" : "false");
    
    return 0;
}
