// Problem: Weekly Contest 422 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

typedef struct {
    int time;
    int row;
    int col;
} HeapNode;

typedef struct {
    HeapNode* array;
    int capacity;
    int size;
} MinHeap;

MinHeap* createMinHeap(int capacity) {
    MinHeap* minHeap = (MinHeap*)malloc(sizeof(MinHeap));
    minHeap->capacity = capacity;
    minHeap->size = 0;
    minHeap->array = (HeapNode*)malloc(capacity * sizeof(HeapNode));
    return minHeap;
}

void swapHeapNodes(HeapNode* a, HeapNode* b) {
    HeapNode temp = *a;
    *a = *b;
    *b = temp;
}

void minHeapify(MinHeap* minHeap, int idx) {
    int smallest = idx;
    int left = 2 * idx + 1;
    int right = 2 * idx + 2;

    if (left < minHeap->size && minHeap->array[left].time < minHeap->array[smallest].time)
        smallest = left;

    if (right < minHeap->size && minHeap->array[right].time < minHeap->array[smallest].time)
        smallest = right;

    if (smallest != idx) {
        swapHeapNodes(&minHeap->array[idx], &minHeap->array[smallest]);
        minHeapify(minHeap, smallest);
    }
}

void insertMinHeap(MinHeap* minHeap, int time, int row, int col) {
    if (minHeap->size == minHeap->capacity) {
        minHeap->capacity *= 2;
        minHeap->array = (HeapNode*)realloc(minHeap->array, minHeap->capacity * sizeof(HeapNode));
    }

    int i = minHeap->size;
    minHeap->array[i].time = time;
    minHeap->array[i].row = row;
    minHeap->array[i].col = col;
    minHeap->size++;

    while (i != 0 && minHeap->array[(i - 1) / 2].time > minHeap->array[i].time) {
        swapHeapNodes(&minHeap->array[i], &minHeap->array[(i - 1) / 2]);
        i = (i - 1) / 2;
    }
}

HeapNode extractMin(MinHeap* minHeap) {
    HeapNode minNode = minHeap->array[0];
    minHeap->array[0] = minHeap->array[minHeap->size - 1];
    minHeap->size--;
    minHeapify(minHeap, 0);
    return minNode;
}

int isEmpty(MinHeap* minHeap) {
    return minHeap->size == 0;
}

void freeMinHeap(MinHeap* minHeap) {
    free(minHeap->array);
    free(minHeap);
}

int minTimeToReach(int** moveTime, int moveTimeSize, int* moveTimeColSize) {
    if (moveTimeSize == 0 || moveTimeColSize[0] == 0) {
        return 0;
    }
    
    int rows = moveTimeSize;
    int cols = moveTimeColSize[0];
    
    int** vis = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        vis[i] = (int*)calloc(cols, sizeof(int)); 
    }
    
    MinHeap* minHeap = createMinHeap(rows * cols);
    
    insertMinHeap(minHeap, 0, 0, 0);
    vis[0][0] = 1;
    
    int drow[] = {-1, 0, 1, 0};
    int dcol[] = {0, 1, 0, -1};
    
    int time = 0;
    
    while (!isEmpty(minHeap)) {
        HeapNode currentNode = extractMin(minHeap);
        time = currentNode.time;
        int r = currentNode.row;
        int c = currentNode.col;
        
        if (r == rows - 1 && c == cols - 1) {
            for (int i = 0; i < rows; i++) {
                free(vis[i]);
            }
            free(vis);
            freeMinHeap(minHeap);
            return time;
        }
        
        for (int i = 0; i < 4; i++) {
            int nrow = r + drow[i];
            int ncol = c + dcol[i];
            
            if (nrow >= 0 && nrow < rows && ncol >= 0 && ncol < cols && vis[nrow][ncol] == 0) {
                int nextCost = 2 - (nrow + ncol) % 2;
                int newTime;
                
                if (moveTime[nrow][ncol] >= time) {
                    newTime = moveTime[nrow][ncol] + nextCost;
                } else {
                    newTime = time + nextCost;
                }
                
                insertMinHeap(minHeap, newTime, nrow, ncol);
                vis[nrow][ncol] = 1;
            }
        }
    }
    
    for (int i = 0; i < rows; i++) {
        free(vis[i]);
    }
    free(vis);
    freeMinHeap(minHeap);
    
    return time;
}

int main() {
    int rows, cols;
    
    scanf("%d %d", &rows, &cols);
    
    int** moveTime = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        moveTime[i] = (int*)malloc(cols * sizeof(int));
    }
    
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            scanf("%d", &moveTime[i][j]);
        }
    }
    
    int* moveTimeColSize = (int*)malloc(rows * sizeof(int));
    for (int i = 0; i < rows; i++) {
        moveTimeColSize[i] = cols;
    }
    
    int result = minTimeToReach(moveTime, rows, moveTimeColSize);
    printf("%d\n", result);
    
    for (int i = 0; i < rows; i++) {
        free(moveTime[i]);
    }
    free(moveTime);
    free(moveTimeColSize);
    
    return 0;
}
