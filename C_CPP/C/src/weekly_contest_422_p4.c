// Problem: Weekly Contest 422 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MOD 1000000007
#define MAX_NUM_LENGTH 81
#define MAX_DIGITS 10
#define MAX_SUM 721  
#define MAX_COUNT 81

int n;
int cnt[MAX_DIGITS];
int left_s[MAX_DIGITS];
int left_c[MAX_DIGITS];
long dp[MAX_DIGITS][MAX_SUM][MAX_COUNT];
long r1[MAX_DIGITS + 1];
int cb[81][81];

void pascal() {
    memset(cb, 0, sizeof(cb));
    cb[0][0] = 1;
    for (int i = 1; i <= 80; ++i) {
        cb[i][0] = 1;
        cb[i][i] = 1;
        for (int j = 1; j < i; ++j)
            cb[i][j] = (cb[i-1][j-1] + cb[i-1][j]) % MOD;
    }
}

long dfs(int i, int s, int c) {
    if (s == 0 && c == 0) return r1[i];
    if (i == MAX_DIGITS) return 0;
    if (s > left_s[i] || c > left_c[i]) return 0;
    if (dp[i][s][c] != -1) return dp[i][s][c];
    
    long res = 0;
    int a = s;
    
    for (int x = 0, y = cnt[i]; x <= cnt[i] && a >= 0 && c >= x; ++x, --y, a -= i) {
        if (y > left_c[i] - c) continue;
        long b = (dfs(i + 1, a, c - x) * cb[c][x]) % MOD;
        res = (res + b * cb[left_c[i] - c][y]) % MOD;
    }
    
    return dp[i][s][c] = res;
}

int countBalancedPermutations(char* num) {
    int s = 0;
    memset(cnt, 0, sizeof(cnt));
    
    for (int i = 0; num[i] != '\0'; ++i) {
        int digit = num[i] - '0';
        s += digit;
        ++cnt[digit];
    }
    
    if (s % 2) return 0;
    
    pascal();
    
    r1[MAX_DIGITS] = 1;
    
    int ls = 0, lc = 0;
    for (int i = 9; i >= 0; --i) {
        ls += i * cnt[i];
        lc += cnt[i];
        left_s[i] = ls;
        left_c[i] = lc;
        r1[i] = (r1[i + 1] * cb[left_c[i]][cnt[i]]) % MOD;
    }
    
    n = strlen(num);
    
    memset(dp, -1, sizeof(dp));
    
    return dfs(0, s / 2, n / 2);
}

int main() {
    char num[MAX_NUM_LENGTH + 1]; 
    
    scanf("%s", num);
    
    if (strlen(num) > MAX_NUM_LENGTH - 1) {
        printf("Input too long, maximum allowed length is %d\n", MAX_NUM_LENGTH - 1);
        return 1;
    }
    
    int result = countBalancedPermutations(num);
    printf("%d\n", result);
    
    return 0;
}
