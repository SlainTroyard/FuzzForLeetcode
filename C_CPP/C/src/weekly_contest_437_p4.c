// Problem: Weekly Contest 437 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

int dp[500][500];

bool suc(int a, int b) { 
    return (a == 0 && b == 2) || (a == 2 && b == 0); 
}

void update_dp(int** G, int m, int n, int i, int j) {
    dp[i][j] = 1;
    i--;
    j++;
    while (i >= 0 && j < n) {
        if (suc(G[i][j], G[i + 1][j - 1]))
            dp[i][j] = dp[i + 1][j - 1] + 1;
        else
            dp[i][j] = 1;
        i--;
        j++;
    }
}

int solve(int** G, int m, int n) {
    for (int i = 0; i < m; i++)
        update_dp(G, m, n, i, 0);

    for (int j = 1; j < n; j++)
        update_dp(G, m, n, m - 1, j);

    int ans = 0;
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            
            if (G[i][j] == 1) {
                ans = fmax(ans, 1);
                int ii = i + 1;
                int jj = j + 1;
                int len = 1;

                while (ii < m && jj < n) {
                    if (len == 1 && G[ii][jj] != 2)
                        break;

                    if (len > 1 && !suc(G[ii][jj], G[ii - 1][jj - 1]))
                        break;

                    ans = fmax(len + dp[ii][jj], ans);
                    
                    len++;
                    ii++;
                    jj++;
                }
            }
        }
    }

    return ans;
}

int** rotate(int** grid, int m, int n) {
    int** arr = malloc(sizeof(int*) * n);
    for (int i = 0; i < n; i++) {
        arr[i] = malloc(sizeof(int) * m);
        for (int j = 0; j < m; j++) {
            arr[i][j] = grid[j][n - 1 - i];
        }
    }
    return arr;
}

int lenOfVDiagonal(int** grid, int gridSize, int* gridColSize) {
    int m = gridSize;
    int n = gridColSize[0];

    int** arr_1 = rotate(grid, m, n);
    int** arr_2 = rotate(arr_1, n, m);
    int** arr_3 = rotate(arr_2, m, n);

    int res_1 = solve(grid, m, n);
    int res_2 = solve(arr_1, n, m);
    int res_3 = solve(arr_2, m, n);
    int res_4 = solve(arr_3, n, m);

    return fmax(fmax(fmax(res_1, res_2), res_3), res_4);
}

int main() {
    // TODO: Add the base I/O interface here
    int n, m;
    scanf("%d %d", &n, &m);
    int** grid = malloc(sizeof(int*) * n);
    for (int i = 0; i < n; i++) {
        grid[i] = malloc(sizeof(int) * m);
        for (int j = 0; j < m; j++) {
            scanf("%d", &grid[i][j]);
        }
    }
    int gridColSize[1] = {m};
    printf("%d\n", lenOfVDiagonal(grid, n, gridColSize));
    free(grid);
    return 0;
}
