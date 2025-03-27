// Problem: Weekly Contest 433 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

typedef struct {
    long long*** memo;  
    int** cost;         
    int n;              
} DFSContext;

long long dfs(DFSContext* ctx, int i, int pre_j, int pre_k) {
    if (i < 0) {
        return 0;
    }

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

    ctx->memo[i][pre_j][pre_k] = min_res;
    return min_res;
}

long long minCost(int n, int** cost, int costSize, int* costColSize) {
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

    DFSContext ctx;
    ctx.memo = memo;
    ctx.cost = cost;
    ctx.n = n;

    long long result = dfs(&ctx, n / 2 - 1, 3, 3);

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
    int n;
    if (scanf("%d", &n) != 1) {
        fprintf(stderr, "Error reading input for n\n");
        return 1;
    }
    
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
                for (int k = 0; k <= i; k++) {
                    free(cost[k]);
                }
                free(cost);
                free(costColSize);
                return 1;
            }
        }
    }
    
    long long result = minCost(n, cost, n, costColSize);
    
    printf("%lld\n", result);
    
    for (int i = 0; i < n; i++) {
        free(cost[i]);
    }
    free(cost);
    free(costColSize);
    
    return 0;
}
