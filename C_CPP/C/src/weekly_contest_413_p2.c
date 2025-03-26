// Problem: Weekly Contest 413 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// 交换两个整数
void swap(int* a, int* b) {
    int temp = *a;
    *a = *b;
    *b = temp;
}

// 向下调整堆
void heapifyDown(int* heap, int size, int idx) {
    int largest = idx;
    int left = 2 * idx + 1;
    int right = 2 * idx + 2;
    
    if (left < size && heap[left] > heap[largest])
        largest = left;
    if (right < size && heap[right] > heap[largest])
        largest = right;
    
    if (largest != idx) {
        swap(&heap[idx], &heap[largest]);
        heapifyDown(heap, size, largest);
    }
}

// 向上调整堆
void heapifyUp(int* heap, int idx) {
    while (idx > 0) {
        int parent = (idx - 1) / 2;
        if (heap[parent] < heap[idx]) {
            swap(&heap[parent], &heap[idx]);
            idx = parent;
        } else {
            break;
        }
    }
}

// 插入元素到堆中
void heapInsert(int* heap, int* size, int val) {
    heap[*size] = val;
    heapifyUp(heap, *size);
    (*size)++;
}

// 移除堆顶元素
void heapRemoveTop(int* heap, int* size) {
    if (*size <= 1) {
        *size = 0;
        return;
    }
    heap[0] = heap[*size - 1];
    (*size)--;
    heapifyDown(heap, *size, 0);
}

int* resultsArray(int** queries, int queriesSize, int* queriesColSize, int k, int* returnSize) {
    int* result = (int*)malloc(sizeof(int) * queriesSize);
    *returnSize = queriesSize;
    
    // 使用数组实现大顶堆，直接存储正距离
    int* heap = (int*)malloc(sizeof(int) * (k + 1));
    int heapSize = 0;
    
    for (int i = 0; i < queriesSize; i++) {
        // 计算正距离
        int distance = abs(queries[i][0]) + abs(queries[i][1]);
        
        // 将当前距离加入大顶堆
        heapInsert(heap, &heapSize, distance);
        
        // 如果堆的大小超过k，移除最大的元素
        if (heapSize > k) {
            heapRemoveTop(heap, &heapSize);
        }
        
        // 如果堆的大小等于k，返回堆顶元素（第k大的距离）
        if (heapSize == k) {
            result[i] = heap[0];
        } else {
            result[i] = -1;  // 堆中元素少于k个
        }
    }
    
    free(heap);
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
    free(result);
    for (int i = 0; i < queriesSize; i++) {
        free(queries[i]);
    }
    free(queries);
    return 0;
}
