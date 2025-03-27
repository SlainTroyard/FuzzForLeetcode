// Problem: Weekly Contest 433 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

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

bool isEmpty(Stack* stack) {
    return stack->top == -1;
}

int size(Stack* stack) {
    return stack->top + 1;
}

void push(Stack* stack, int value) {
    if (stack->top == stack->capacity - 1) return; 
    stack->data[++(stack->top)] = value;
}

int pop(Stack* stack) {
    if (isEmpty(stack)) return -1; 
    return stack->data[(stack->top)--];
}

int top(Stack* stack) {
    if (isEmpty(stack)) return -1; 
    return stack->data[stack->top];
}

void destroyStack(Stack* stack) {
    free(stack->data);
    free(stack);
}

long long count(int m, int k) {
    if (m > k) {
        return (long long)(m * 2 - k + 1) * k / 2;
    } else {
        return (long long)(m + 1) * m / 2;
    }
}

long long sumSubarrayMins(int* nums, int numsSize, int k) {
    long long res = 0;
    Stack* stack = createStack(numsSize + 1);
    
    push(stack, -1); 
    
    for (int r = 0; r < numsSize; r++) {
        while (size(stack) > 1 && nums[top(stack)] >= nums[r]) {
            int i = pop(stack);
            int l = top(stack);
            long long cnt = count(r - l - 1, k) - count(i - l - 1, k) - count(r - i - 1, k);
            res += (long long)nums[i] * cnt;
        }
        push(stack, r);
    }
    
    destroyStack(stack);
    return res;
}

long long minMaxSubarraySum(int* nums, int numsSize, int k) {
    int* temp = (int*)malloc((numsSize + 1) * sizeof(int));
    if (!temp) return -1; 
    
    memcpy(temp, nums, numsSize * sizeof(int));
    temp[numsSize] = INT_MIN / 2; 
    
    long long ans = sumSubarrayMins(temp, numsSize + 1, k);
    
    for (int i = 0; i < numsSize + 1; i++) {
        temp[i] = -temp[i];
    }
    temp[numsSize] = -temp[numsSize]; 
    
    ans -= sumSubarrayMins(temp, numsSize + 1, k);
    
    free(temp);
    
    return ans;
}

int main() {
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        fprintf(stderr, "Error reading input for n and k\n");
        return 1;
    }
    
    int* nums = (int*)malloc(n * sizeof(int));
    if (!nums) {
        fprintf(stderr, "Memory allocation failed for nums array\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &nums[i]) != 1) {
            fprintf(stderr, "Error reading input for nums[%d]\n", i);
            free(nums);
            return 1;
        }
    }
    
    long long result = minMaxSubarraySum(nums, n, k);
    
    printf("%lld\n", result);
    
    free(nums);
    
    return 0;
}
