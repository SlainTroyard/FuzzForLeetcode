// Problem: Weekly Contest 413 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int* resultsArray(int** queries, int queriesSize, int* queriesColSize, int k, int* returnSize) {
    int* result = (int*)malloc(sizeof(int) * queriesSize);
    *returnSize = queriesSize;
    memset(result, -1, sizeof(int) * queriesSize);
    
    int* distanceArr = (int*)malloc(sizeof(int) * (k + 1));
    int distanceSize = 0;
    
    for (int i = 0; i < queriesSize; i++) {
        int distance = abs(queries[i][0]) + abs(queries[i][1]);
        
        int j = distanceSize;
        while (j > 0 && distanceArr[j - 1] < distance) {
            if (j < k) {
                distanceArr[j] = distanceArr[j - 1];
            }
            j--;
        }
        if (j < k) {
            distanceArr[j] = distance;
            if (distanceSize < k) distanceSize++;
        }
        
        if (distanceSize == k) {
            result[i] = distanceArr[k - 1];
        }
    }
    
    free(distanceArr);
    return result;
}

int main() {
    int queriesSize, k;
    scanf("%d %d", &queriesSize, &k);
    int** queries = (int**)malloc(sizeof(int*) * queriesSize);
    for (int i = 0; i < queriesSize; i++) {
        queries[i] = (int*)malloc(sizeof(int) * 2);
        scanf("%d %d", &queries[i][0], &queries[i][1]);
    }
    int queriesColSize = 2;
    int returnSize;
    int* result = resultsArray(queries, queriesSize, &queriesColSize, k, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", result[i]);
    }
    return 0;
}
