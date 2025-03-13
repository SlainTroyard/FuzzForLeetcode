// Problem: Weekly Contest 430 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int minimumOperations(int** grid, int gridSize, int* gridColSize) {
    int** calGrid = (int**)malloc(gridSize * sizeof(int*));
    for (int i = 0; i < gridSize; i++) {
        calGrid[i] = (int*)malloc(gridColSize[i] * sizeof(int));
        for (int j = 0; j < gridColSize[i]; j++) {
            calGrid[i][j] = grid[i][j];
        }
    }

    int ans = 0;
    if (gridSize == 1) {
        for (int i = 0; i < gridSize; i++) {
            free(calGrid[i]);
        }
        free(calGrid);
        return 0;
    }

    for (int i = 0; i < gridColSize[0]; i++) {
        for (int j = 1; j < gridSize; j++) {
            if (calGrid[j][i] <= calGrid[j - 1][i]) {
                ans += calGrid[j - 1][i] + 1 - calGrid[j][i];
                calGrid[j][i] = calGrid[j - 1][i] + 1;
            }
        }
    }

    for (int i = 0; i < gridSize; i++) {
        free(calGrid[i]);
    }
    free(calGrid);
    return ans;
}

int main() {
    int gridSize, gridColSize;
    scanf("%d %d", &gridSize, &gridColSize); // Input rows and columns

    int* colSizes = (int*)malloc(gridSize * sizeof(int));
    int** grid = (int**)malloc(gridSize * sizeof(int*));
    for (int i = 0; i < gridSize; i++) {
        grid[i] = (int*)malloc(gridColSize * sizeof(int));
        colSizes[i] = gridColSize;
        for (int j = 0; j < gridColSize; j++) {
            scanf("%d", &grid[i][j]); // Input matrix elements
        }
    }

    int result = minimumOperations(grid, gridSize, colSizes);
    printf("%d\n", result); // Output the result

    for (int i = 0; i < gridSize; i++) {
        free(grid[i]);
    }
    free(grid);
    free(colSizes);

    return 0;
}
