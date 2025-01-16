// Problem: Weekly Contest 419 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int countWinningSequences(char* s) {
    const int max_size = 1e9 + 7;
    int dp[2][3][2001] = {0};
    int res[3][3] = {0};
    int c_arr[26] = {0};
    int s_len = strlen(s);
    int ans = 0;
    c_arr['F' - 'A'] = 0;
    c_arr['W' - 'A'] = 1;
    c_arr['E' - 'A'] = 2;
    res[0][2] = 1;
    res[0][1] = -1;
    res[2][0] = -1;
    res[1][0] = 1;
    res[2][1] = 1;
    res[1][2] = -1;

    for(int i = 0; i <= 2; ++i)
    {
        int c = c_arr[s[0] - 'A'];
        int score = res[i][c];
        dp[0][i][1000 + score] = 1;
    }

    for(int i = 1; i < s_len; ++i)
    {
        memset(dp[i % 2], 0, sizeof(dp[i % 2]));
        for(int j = 0; j <= 2; ++j)
        {
            int c = c_arr[s[i] - 'A'];
            int score = res[j][c];
            for(int k = 0; k <= 2000; ++k)
            {
                for(int j1 = 0; j1 <=2 ; ++j1)
                {
                    if(j1 != j)
                    {
                        if(k - score >= 0 && k - score <= 2000)
                        {
                            dp[i % 2][j][k] += dp[(i - 1) % 2][j1][k - score];
                            dp[i % 2][j][k] %= max_size;
                        }
                    }
                }
            }
        }
    }

    for(int i = 0; i <= 2; ++i)
    {
        for(int j = 1001; j <= 2000; ++j)
        {
            ans += dp[(s_len - 1) % 2][i][j];
            ans %= max_size;
        }
    }

    return ans;
}

int main() {
    int charSize;
    scanf("%d", &charSize);
    char *s = (char *)malloc(charSize * sizeof(char));
    scanf("%s", s);
    printf("%d\n", countWinningSequences(s));
    free(s);
    return 0;
}
