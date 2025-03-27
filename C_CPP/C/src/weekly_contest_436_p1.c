// Problem: Weekly Contest 436 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int cmp_asc(const void *a, const void *b) {
    return *(int *)a - *(int *)b;
}

int cmp_desc(const void *a, const void *b) {
    return *(int *)b - *(int *)a;
}

int** sortMatrix(int** grid, int gridSize, int* gridColSize, int* returnSize, int** returnColumnSizes) {
    *returnSize = gridSize;
    *returnColumnSizes = malloc(sizeof(int) * gridSize);
    for (int i = 0; i < gridSize; i++) {
        (*returnColumnSizes)[i] = gridSize;
    }
    for (int i = 0; i < gridSize; i++) {
        int len = gridSize - i; 
        int *a = (int *)malloc(sizeof(int) * len);
        int index = 0;
        for (int k = i, m = 0; k < gridSize && m < gridSize; k++, m++) {
            a[index++] = grid[k][m];
        }
        qsort(a, index, sizeof(int), cmp_desc);
        index = 0;
        for (int k = i, m = 0; k < gridSize && m < gridSize; k++, m++) {
            grid[k][m] = a[index++];
        }
    }
    for (int i = 1; i < gridSize; i++) {
        int len = gridSize - i;
        int *a = (int *)malloc(sizeof(int) * len);
        int index = 0;
        for (int k = 0, m = i; k < gridSize && m < gridSize; k++, m++) {
            a[index++] = grid[k][m];
        }
        qsort(a, index, sizeof(int), cmp_asc);
        index = 0;
        for (int k = 0, m = i; k < gridSize && m < gridSize; k++, m++) {
            grid[k][m] = a[index++];
        }
    }

    return grid;
}

int main() {
    int n;
    scanf("%d", &n);
    int **grid = (int **)malloc(sizeof(int *) * n);
    for (int i = 0; i < n; i++) {
        grid[i] = (int *)malloc(sizeof(int) * n);
        for (int j = 0; j < n; j++) {
            scanf("%d", &grid[i][j]);
        }
    }
    int returnSize;
    int *returnColumnSizes;
    int **result = sortMatrix(grid, n, &n, &returnSize, &returnColumnSizes);
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            printf("%d ", result[i][j]);
        }
        printf("\n");
    }
    free(grid);
    free(returnColumnSizes);
    return 0;
}
