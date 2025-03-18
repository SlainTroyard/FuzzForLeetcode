// Problem: Weekly Contest 422 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

// Structure for min heap
typedef struct {
    int time;
    int x;
    int y;
} HeapNode;

typedef struct {
    HeapNode* array;
    int capacity;
    int size;
} MinHeap;

// Function to initialize min heap
MinHeap* createMinHeap(int capacity) {
    MinHeap* minHeap = (MinHeap*)malloc(sizeof(MinHeap));
    minHeap->capacity = capacity;
    minHeap->size = 0;
    minHeap->array = (HeapNode*)malloc(capacity * sizeof(HeapNode));
    return minHeap;
}

// Function to swap two heap nodes
void swapHeapNodes(HeapNode* a, HeapNode* b) {
    HeapNode temp = *a;
    *a = *b;
    *b = temp;
}

// Function to heapify at given index
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

// Function to insert a new node to min heap
void insertMinHeap(MinHeap* minHeap, int time, int x, int y) {
    if (minHeap->size == minHeap->capacity) {
        // Resize heap if needed
        minHeap->capacity *= 2;
        minHeap->array = (HeapNode*)realloc(minHeap->array, minHeap->capacity * sizeof(HeapNode));
    }

    int i = minHeap->size;
    minHeap->array[i].time = time;
    minHeap->array[i].x = x;
    minHeap->array[i].y = y;
    minHeap->size++;

    // Fix the min heap property if it is violated
    while (i != 0 && minHeap->array[(i - 1) / 2].time > minHeap->array[i].time) {
        swapHeapNodes(&minHeap->array[i], &minHeap->array[(i - 1) / 2]);
        i = (i - 1) / 2;
    }
}

// Function to extract the minimum node from heap
HeapNode extractMin(MinHeap* minHeap) {
    HeapNode minNode = minHeap->array[0];
    minHeap->array[0] = minHeap->array[minHeap->size - 1];
    minHeap->size--;
    minHeapify(minHeap, 0);
    return minNode;
}

// Function to check if min heap is empty
int isEmpty(MinHeap* minHeap) {
    return minHeap->size == 0;
}

// Function to free min heap
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

    // Create min heap
    MinHeap* minHeap = createMinHeap(rows * cols);
    
    // Create time matrix to track minimum time to reach each cell
    int** time = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        time[i] = (int*)malloc(cols * sizeof(int));
        for (int j = 0; j < cols; j++) {
            time[i][j] = INT_MAX;
        }
    }
    
    // Insert the starting point
    insertMinHeap(minHeap, 0, 0, 0);
    time[0][0] = 0;
    
    // Directions: up, right, down, left
    int dx[] = {-1, 0, 1, 0};
    int dy[] = {0, 1, 0, -1};
    
    while (!isEmpty(minHeap)) {
        HeapNode currentNode = extractMin(minHeap);
        int currentTime = currentNode.time;
        int x = currentNode.x;
        int y = currentNode.y;
        
        // If destination is reached
        if (x == rows - 1 && y == cols - 1) {
            // Free allocated memory
            for (int i = 0; i < rows; i++) {
                free(time[i]);
            }
            free(time);
            freeMinHeap(minHeap);
            return currentTime;
        }
        
        // Check all four directions
        for (int i = 0; i < 4; i++) {
            int newX = x + dx[i];
            int newY = y + dy[i];
            
            if (newX >= 0 && newX < rows && newY >= 0 && newY < cols) {
                // Calculate wait time if needed
                int waitTime = moveTime[newX][newY] > currentTime ? moveTime[newX][newY] - currentTime : 0;
                int newTime = currentTime + 1 + waitTime;
                
                if (newTime < time[newX][newY]) {
                    time[newX][newY] = newTime;
                    insertMinHeap(minHeap, newTime, newX, newY);
                }
            }
        }
    }
    
    // Free allocated memory
    for (int i = 0; i < rows; i++) {
        free(time[i]);
    }
    free(time);
    freeMinHeap(minHeap);
    
    return -1; // Unreachable
}

int main() {
    int rows, cols;
    
    // Read the number of rows and columns
    scanf("%d %d", &rows, &cols);
    
    // Allocate memory for moveTime matrix
    int** moveTime = (int**)malloc(rows * sizeof(int*));
    for (int i = 0; i < rows; i++) {
        moveTime[i] = (int*)malloc(cols * sizeof(int));
    }
    
    // Read the grid values
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            scanf("%d", &moveTime[i][j]);
        }
    }
    
    // Create moveTimeColSize array for LeetCode format
    int* moveTimeColSize = (int*)malloc(rows * sizeof(int));
    for (int i = 0; i < rows; i++) {
        moveTimeColSize[i] = cols;
    }
    
    // Call the function and output the result
    int result = minTimeToReach(moveTime, rows, moveTimeColSize);
    printf("%d\n", result);
    
    // Free allocated memory
    for (int i = 0; i < rows; i++) {
        free(moveTime[i]);
    }
    free(moveTime);
    free(moveTimeColSize);
    
    return 0;
}
