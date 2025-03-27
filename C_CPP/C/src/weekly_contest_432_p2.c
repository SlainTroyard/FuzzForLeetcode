// Problem: Weekly Contest 432 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

int maximumAmount(int** coins, int coinsSize, int* coinsColSize) {
    int row = coinsSize - 1;
    int col = coinsColSize[0] - 1;
    int dp[row + 1][col + 1][3];
    dp[row][col][0] = coins[row][col];
    dp[row][col][1] = fmax(coins[row][col],0);
    dp[row][col][2] = fmax(coins[row][col],0);
    for(int i = col - 1;i >= 0;i--){
        dp[row][i][0] = dp[row][i + 1][0] + coins[row][i];
        dp[row][i][1] = fmax(dp[row][i + 1][0],dp[row][i + 1][1] + coins[row][i]);
        dp[row][i][2] = fmax(dp[row][i + 1][1],dp[row][i + 1][2] + coins[row][i]);
    }
    for(int i = row - 1;i >= 0;i--){
        dp[i][col][0] = dp[i + 1][col][0] + coins[i][col];
        dp[i][col][1] = fmax(dp[i + 1][col][0],dp[i + 1][col][1] + coins[i][col]);
        dp[i][col][2] = fmax(dp[i + 1][col][1],dp[i + 1][col][2] + coins[i][col]);
    }
    for(int j = row - 1;j >= 0;j--){
        for(int i = col - 1;i >= 0;i--){
            dp[j][i][0] = fmax(dp[j + 1][i][0],dp[j][i + 1][0]) + coins[j][i];
            dp[j][i][1] = fmax(fmax(dp[j][i + 1][0],dp[j][i + 1][1] + coins[j][i]),fmax(dp[j + 1][i][0],dp[j + 1][i][1] + coins[j][i]));
            dp[j][i][2] = fmax(fmax(dp[j][i + 1][1],dp[j][i + 1][2] + coins[j][i]),fmax(dp[j + 1][i][1],dp[j + 1][i][2] + coins[j][i]));
        }
    }
    return dp[0][0][2];
}

int main() {
    int n, m;
    scanf("%d %d", &n, &m);
    int** coins = (int**)malloc(n * sizeof(int*));
    for (int i = 0; i < n; i++) {
        coins[i] = (int*)malloc(m * sizeof(int));
        for (int j = 0; j < m; j++) {
            scanf("%d", &coins[i][j]);
        }
    }
    int coinsColSize[n];
    for (int i = 0; i < n; i++) {
        coinsColSize[i] = m;
    }
    printf("%d\n", maximumAmount(coins, n, coinsColSize));
    return 0;
}
