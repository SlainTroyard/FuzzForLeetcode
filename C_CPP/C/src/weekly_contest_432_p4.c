// Problem: Weekly Contest 432 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    int* data;
    int top;
    int capacity;
} Stack;

Stack* createStack(int capacity) {
    Stack* stack = (Stack*)malloc(sizeof(Stack));
    if (!stack) return NULL;
    stack->data = (int*)malloc(capacity * sizeof(int));
    if (!stack->data) {
        free(stack);
        return NULL;
    }
    stack->top = -1;
    stack->capacity = capacity;
    return stack;
}

bool stackIsEmpty(Stack* stack) {
    return stack->top == -1;
}

void stackPush(Stack* stack, int item) {
    if (stack->top == stack->capacity - 1) return; 
    stack->data[++stack->top] = item;
}

int stackPop(Stack* stack) {
    if (stackIsEmpty(stack)) return -1; 
    return stack->data[stack->top--];
}

int stackTop(Stack* stack) {
    if (stackIsEmpty(stack)) return -1; 
    return stack->data[stack->top];
}

void destroyStack(Stack* stack) {
    free(stack->data);
    free(stack);
}

typedef struct {
    int* data;
    int front;
    int rear;
    int size;
    int capacity;
} Deque;

Deque* createDeque(int capacity) {
    Deque* deque = (Deque*)malloc(sizeof(Deque));
    if (!deque) return NULL;
    deque->data = (int*)malloc(capacity * sizeof(int));
    if (!deque->data) {
        free(deque);
        return NULL;
    }
    deque->front = 0;
    deque->rear = -1;
    deque->size = 0;
    deque->capacity = capacity;
    return deque;
}

bool dequeIsEmpty(Deque* deque) {
    return deque->size == 0;
}

void dequePushBack(Deque* deque, int item) {
    if (deque->size == deque->capacity) return; 
    deque->rear = (deque->rear + 1) % deque->capacity;
    deque->data[deque->rear] = item;
    deque->size++;
}

int dequePopBack(Deque* deque) {
    if (dequeIsEmpty(deque)) return -1; 
    int item = deque->data[deque->rear];
    deque->rear = (deque->rear - 1 + deque->capacity) % deque->capacity;
    deque->size--;
    return item;
}

int dequePopFront(Deque* deque) {
    if (dequeIsEmpty(deque)) return -1; 
    int item = deque->data[deque->front];
    deque->front = (deque->front + 1) % deque->capacity;
    deque->size--;
    return item;
}

int dequeFront(Deque* deque) {
    if (dequeIsEmpty(deque)) return -1; 
    return deque->data[deque->front];
}

int dequeBack(Deque* deque) {
    if (dequeIsEmpty(deque)) return -1; 
    return deque->data[deque->rear];
}

void destroyDeque(Deque* deque) {
    free(deque->data);
    free(deque);
}

typedef struct {
    int* data;
    int size;
    int capacity;
} Vector;

Vector* createVector(int capacity) {
    Vector* vector = (Vector*)malloc(sizeof(Vector));
    if (!vector) return NULL;
    vector->data = (int*)malloc(capacity * sizeof(int));
    if (!vector->data) {
        free(vector);
        return NULL;
    }
    vector->size = 0;
    vector->capacity = capacity;
    return vector;
}

void vectorPushBack(Vector* vector, int item) {
    if (vector->size == vector->capacity) {
        int newCapacity = vector->capacity * 2;
        int* newData = (int*)realloc(vector->data, newCapacity * sizeof(int));
        if (!newData) return;
        vector->data = newData;
        vector->capacity = newCapacity;
    }
    vector->data[vector->size++] = item;
}

void destroyVector(Vector* vector) {
    free(vector->data);
    free(vector);
}

long long countNonDecreasingSubarrays(int* nums, int numsSize, int k) {
    Vector** g = (Vector**)malloc(numsSize * sizeof(Vector*));
    for (int i = 0; i < numsSize; i++) {
        g[i] = createVector(10); 
    }
    
    int* pos_r = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        pos_r[i] = numsSize;
    }
    
    Stack* st = createStack(numsSize);
    for (int i = 0; i < numsSize; i++) {
        int x = nums[i];
        while (!stackIsEmpty(st) && x >= nums[stackTop(st)]) {
            pos_r[stackTop(st)] = i;
            stackPop(st);
        }
        if (!stackIsEmpty(st)) {
            vectorPushBack(g[stackTop(st)], i);
        }
        stackPush(st, i);
    }
    
    long long ans = 0;
    int cnt = 0, l = 0;
    Deque* q = createDeque(numsSize);
    
    for (int r = 0; r < numsSize; r++) {
        int x = nums[r];
        while (!dequeIsEmpty(q) && nums[dequeBack(q)] <= x) {
            dequePopBack(q);
        }
        dequePushBack(q, r);
        cnt += nums[dequeFront(q)] - x;
        
        while (cnt > k) {
            int out = nums[l];
            for (int j = 0; j < g[l]->size; j++) {
                int i = g[l]->data[j];
                if (i > r) {
                    break;
                }
                int min_pos = pos_r[i] < r + 1 ? pos_r[i] : r + 1;
                cnt -= (out - nums[i]) * (min_pos - i);
            }
            l++;
            if (!dequeIsEmpty(q) && dequeFront(q) < l) {
                dequePopFront(q);
            }
        }
        ans += r - l + 1;
    }
    
    for (int i = 0; i < numsSize; i++) {
        destroyVector(g[i]);
    }
    free(g);
    free(pos_r);
    destroyStack(st);
    destroyDeque(q);
    
    return ans;
}

int main() {
    int numsSize, k;
    if (scanf("%d %d", &numsSize, &k) != 2) {
        fprintf(stderr, "Error reading input for numsSize and k\n");
        return 1;
    }
    
    int* nums = (int*)malloc(numsSize * sizeof(int));
    if (!nums) {
        fprintf(stderr, "Memory allocation failed for nums array\n");
        return 1;
    }
    
    for (int i = 0; i < numsSize; i++) {
        if (scanf("%d", &nums[i]) != 1) {
            fprintf(stderr, "Error reading input for nums[%d]\n", i);
            free(nums);
            return 1;
        }
    }
    
    long long result = countNonDecreasingSubarrays(nums, numsSize, k);
    
    printf("%lld\n", result);
    
    free(nums);
    
    return 0;
}
