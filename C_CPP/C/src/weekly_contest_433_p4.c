// Problem: Weekly Contest 433 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

// 栈的实现
typedef struct {
    int* data;
    int top;
    int capacity;
} Stack;

// 创建栈
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

// 检查栈是否为空
bool isEmpty(Stack* stack) {
    return stack->top == -1;
}

// 检查栈的大小
int size(Stack* stack) {
    return stack->top + 1;
}

// 入栈
void push(Stack* stack, int value) {
    if (stack->top == stack->capacity - 1) return; // 栈满
    stack->data[++(stack->top)] = value;
}

// 出栈
int pop(Stack* stack) {
    if (isEmpty(stack)) return -1; // 栈空
    return stack->data[(stack->top)--];
}

// 查看栈顶元素
int top(Stack* stack) {
    if (isEmpty(stack)) return -1; // 栈空
    return stack->data[stack->top];
}

// 释放栈
void destroyStack(Stack* stack) {
    free(stack->data);
    free(stack);
}

// 计算组合数量的辅助函数
long long count(int m, int k) {
    if (m > k) {
        return (long long)(m * 2 - k + 1) * k / 2;
    } else {
        return (long long)(m + 1) * m / 2;
    }
}

// 计算子数组最小值的和
long long sumSubarrayMins(int* nums, int numsSize, int k) {
    long long res = 0;
    Stack* stack = createStack(numsSize + 1);
    
    push(stack, -1); // 添加哨兵元素
    
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

// 主函数
long long minMaxSubarraySum(int* nums, int numsSize, int k) {
    // 创建一个临时数组，包含原始数组和一个额外元素
    int* temp = (int*)malloc((numsSize + 1) * sizeof(int));
    if (!temp) return -1; // 内存分配失败
    
    // 复制原始数组
    memcpy(temp, nums, numsSize * sizeof(int));
    temp[numsSize] = INT_MIN / 2; // 添加一个非常小的值作为哨兵
    
    // 计算子数组最小值的和
    long long ans = sumSubarrayMins(temp, numsSize + 1, k);
    
    // 将所有元素取反，再次计算
    for (int i = 0; i < numsSize + 1; i++) {
        temp[i] = -temp[i];
    }
    temp[numsSize] = -temp[numsSize]; // 恢复哨兵元素的符号
    
    // 从总和中减去取反后的结果
    ans -= sumSubarrayMins(temp, numsSize + 1, k);
    
    // 释放临时数组
    free(temp);
    
    return ans;
}

int main() {
    // 读取输入
    int n, k;
    if (scanf("%d %d", &n, &k) != 2) {
        fprintf(stderr, "Error reading input for n and k\n");
        return 1;
    }
    
    // 分配内存并读取数组
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
    
    // 调用函数计算结果
    long long result = minMaxSubarraySum(nums, n, k);
    
    // 输出结果
    printf("%lld\n", result);
    
    // 释放内存
    free(nums);
    
    return 0;
}
