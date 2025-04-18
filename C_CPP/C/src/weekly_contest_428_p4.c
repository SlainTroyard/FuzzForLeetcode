// Problem: Weekly Contest 428 Problem 4
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <limits.h>

#define INF 1000000

int dfs(int* dp, int* a, int k, int i, int size) {
    if (i >= size) {
        return 0;
    }

    if (dp[i] != -1) {
        return dp[i];
    }

    int ans = INF;
    if (a[i] >= k) {
        int more = a[i] - k;
        ans = (ans < a[i] - k + dfs(dp, a, k, i + 1, size)) ? ans : a[i] - k + dfs(dp, a, k, i + 1, size);
        if (i + 1 < size && a[i + 1] <= k) {
            if (a[i + 1] + more >= k) {
                ans = (ans < more + dfs(dp, a, k, i + 2, size)) ? ans : more + dfs(dp, a, k, i + 2, size);
            } else {
                ans = (ans < (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2, size)) 
                      ? ans 
                      : (more + k - (a[i + 1] + more)) + dfs(dp, a, k, i + 2, size);
            }
        }
    } else {
        ans = (ans < (k - a[i]) + dfs(dp, a, k, i + 1, size)) ? ans : (k - a[i]) + dfs(dp, a, k, i + 1, size);
        if (i + 1 < size && a[i + 1] <= k) {
            if (a[i + 1] + a[i] >= k) {
                ans = (ans < a[i] + dfs(dp, a, k, i + 2, size)) ? ans : a[i] + dfs(dp, a, k, i + 2, size);
            } else {
                ans = (ans < a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size)) 
                      ? ans 
                      : a[i] + (k - (a[i] + a[i + 1])) + dfs(dp, a, k, i + 2, size);
            }
        }
    }

    dp[i] = ans;
    return ans;
}

int makeStringGood(const char* s) {
    int n = strlen(s);
    int a[26] = {0};
    for (int i = 0; i < n; i++) {
        a[s[i] - 'a']++;
    }

    int ans = n;
    for (int i = 0; i < 26; i++) {
        ans = (ans < n - a[i]) ? ans : n - a[i];
    }

    for (int k = 1; k <= n; k++) {
        int dp[26];
        for (int i = 0; i < 26; i++) {
            dp[i] = -1;
        }
        ans = (ans < dfs(dp, a, k, 0, 26)) ? ans : dfs(dp, a, k, 0, 26);
    }

    return ans;
}

int main() {
    char s[20001];

    // Input the string
    scanf("%s", s);

    // Call makeStringGood and output the result
    int result = makeStringGood(s);
    printf("%d\n", result);

    return 0;
}
