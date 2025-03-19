// Problem: Weekly Contest 434 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// 获取两个数中的较大值
int max(int a, int b) {
    return a > b ? a : b;
}

// 主函数实现
int maxFrequency(int* nums, int numsSize, int k) {
    int f0 = 0;               // 跟踪到目前为止有多少个等于k的元素
    int f1[51] = {0};         // 保存每个数字的最大频率
    int max_f1 = 0;           // f1数组中的最大值
    int f2 = 0;               // 特定计算结果
    
    for (int i = 0; i < numsSize; i++) {
        int x = nums[i];
        
        // 更新f2 - 考虑当前元素
        f2 = max(f2, max_f1) + (x == k);
        
        // 更新当前数字的频率
        f1[x] = max(f1[x], f0) + 1;
        
        // 如果当前元素等于k，更新f0
        f0 += (x == k);
        
        // 更新最大频率
        max_f1 = max(max_f1, f1[x]);
    }
    
    // 返回最终结果
    return max(max_f1, f2);
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
    int result = maxFrequency(nums, n, k);
    
    // 输出结果
    printf("%d\n", result);
    
    // 释放内存
    free(nums);
    
    return 0;
}
