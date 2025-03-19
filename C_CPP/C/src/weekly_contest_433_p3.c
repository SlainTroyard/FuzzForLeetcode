// Problem: Weekly Contest 433 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

// 定义记忆化数组和递归函数所需的结构体
typedef struct {
    long long*** memo;  // 3D记忆化数组
    int** cost;         // 成本数组
    int n;              // 数组大小
} DFSContext;

// 递归函数的实现
long long dfs(DFSContext* ctx, int i, int pre_j, int pre_k) {
    if (i < 0) {
        return 0;
    }

    // 检查记忆化数组
    if (ctx->memo[i][pre_j][pre_k] != -1) {
        return ctx->memo[i][pre_j][pre_k];
    }

    long long min_res = LLONG_MAX;
    for (int j = 0; j < 3; j++) {
        if (j == pre_j) {
            continue;
        }
        for (int k = 0; k < 3; k++) {
            if (k != pre_k && k != j) {
                long long temp = dfs(ctx, i - 1, j, k) + 
                                ctx->cost[i][j] + 
                                ctx->cost[ctx->n - 1 - i][k];
                if (temp < min_res) {
                    min_res = temp;
                }
            }
        }
    }

    // 更新记忆化数组
    ctx->memo[i][pre_j][pre_k] = min_res;
    return min_res;
}

// 主要解决方案函数
long long minCost(int n, int** cost, int costSize, int* costColSize) {
    // 创建3D记忆化数组
    long long*** memo = (long long***)malloc(n / 2 * sizeof(long long**));
    for (int i = 0; i < n / 2; i++) {
        memo[i] = (long long**)malloc(4 * sizeof(long long*));
        for (int j = 0; j < 4; j++) {
            memo[i][j] = (long long*)malloc(4 * sizeof(long long));
            for (int k = 0; k < 4; k++) {
                memo[i][j][k] = -1;
            }
        }
    }

    // 创建DFS上下文
    DFSContext ctx;
    ctx.memo = memo;
    ctx.cost = cost;
    ctx.n = n;

    // 调用DFS函数
    long long result = dfs(&ctx, n / 2 - 1, 3, 3);

    // 释放记忆化数组
    for (int i = 0; i < n / 2; i++) {
        for (int j = 0; j < 4; j++) {
            free(memo[i][j]);
        }
        free(memo[i]);
    }
    free(memo);

    return result;
}

int main() {
    // 读取输入
    int n;
    if (scanf("%d", &n) != 1) {
        fprintf(stderr, "Error reading input for n\n");
        return 1;
    }
    
    // 分配内存并读取成本数组
    int** cost = (int**)malloc(n * sizeof(int*));
    int* costColSize = (int*)malloc(n * sizeof(int));
    
    if (!cost || !costColSize) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        cost[i] = (int*)malloc(3 * sizeof(int));
        costColSize[i] = 3;
        
        if (!cost[i]) {
            fprintf(stderr, "Memory allocation failed for cost[%d]\n", i);
            // 释放已分配的内存
            for (int j = 0; j < i; j++) {
                free(cost[j]);
            }
            free(cost);
            free(costColSize);
            return 1;
        }
        
        for (int j = 0; j < 3; j++) {
            if (scanf("%d", &cost[i][j]) != 1) {
                fprintf(stderr, "Error reading input for cost[%d][%d]\n", i, j);
                // 释放已分配的内存
                for (int k = 0; k <= i; k++) {
                    free(cost[k]);
                }
                free(cost);
                free(costColSize);
                return 1;
            }
        }
    }
    
    // 调用函数计算结果
    long long result = minCost(n, cost, n, costColSize);
    
    // 输出结果
    printf("%lld\n", result);
    
    // 释放内存
    for (int i = 0; i < n; i++) {
        free(cost[i]);
    }
    free(cost);
    free(costColSize);
    
    return 0;
}
