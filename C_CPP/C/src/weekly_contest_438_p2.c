// Problem: Weekly Contest 438 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int cmp (const void * a, const void * b) {
    return * (int *) b - * (int *) a;
}
long long maxSum(int** grid, int gridSize, int* gridColSize, int* limits, int limitsSize, int k) {
    int len = 0, size = sizeof(int); 
    for (int i = 0; i < limitsSize; ++ i) len += limits[i];
    int * lst = (int *)calloc(len, size);
    for (int i = 0, l = 0; i < gridSize; ++ i) {
        qsort(grid[i], * gridColSize, size, cmp);
        for (int j = 0; j < limits[i]; ++ j) lst[l ++] = grid[i][j];
    }
    qsort(lst, len, size, cmp);
    long long ans = 0LL;
    for (int i = 0; i < k; ++ i) ans += lst[i];
    return ans;
}

int main() {
    // TODO: Add the base I/O interface here
    int n, m, k;
    scanf("%d %d %d", &n, &m, &k);
    int ** grid = (int **)malloc(n * sizeof(int *));
    for (int i = 0; i < n; ++ i) {
        grid[i] = (int *)malloc(m * sizeof(int));
        for (int j = 0; j < m; ++ j) scanf("%d", &grid[i][j]);
    }
    int * limits = (int *)malloc(n * sizeof(int));
    for (int i = 0; i < n; ++ i) scanf("%d", &limits[i]);
    printf("%lld\n", maxSum(grid, n, &m, limits, n, k));
    free(limits);
    for (int i = 0; i < n; ++ i) free(grid[i]);
    free(grid);
    return 0;
}
