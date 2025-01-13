// Problem: Weekly Contest 418 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

/**
 * Note: The returned array must be malloced, assume caller calls free().
 */
static void dfs(int x, int** adj, int* adjSize, int* visited) {
    visited[x] = 1;
    for(int i = 0; i < adjSize[x]; i++) {
        int nxt = adj[x][i];
        if(!visited[nxt]) dfs(nxt, adj, adjSize, visited);
    }
}

int* remainingMethods(int n, int k, int** invocations, int invocationsSize, int* invocationsColSize, int* returnSize) {
    int** adj = (int**)calloc(n, sizeof(int*));
    int* cap = (int*)calloc(n, sizeof(int));
    int* adjSize = (int*)calloc(n, sizeof(int));
    int* suspicious = (int*)calloc(n, sizeof(int));

    for(int i = 0; i < invocationsSize; i++) {
        int a = invocations[i][0], b = invocations[i][1];
        if(adjSize[a] == cap[a]) {
            cap[a] = cap[a] * 2 + 1;
            adj[a] = (int*)realloc(adj[a], sizeof(int) * cap[a]);
        }
        adj[a][adjSize[a]++] = b;
    }

    dfs(k, adj, adjSize, suspicious);

    for(int i = 0; i < invocationsSize; i++) {
        if(!suspicious[invocations[i][0]] && suspicious[invocations[i][1]]) {
            int* ans = (int*)malloc(sizeof(int) * n);
            for(int j = 0; j < n; j++) ans[j] = j;
            *returnSize = n;
            return ans;
        }
    }

    int count = 0;
    for(int i = 0; i < n; i++) {
        if(!suspicious[i]) count++;
    }
    int* ans = (int*)malloc(sizeof(int) * count);
    *returnSize = count;
    int idx = 0;
    for(int i = 0; i < n; i++) {
        if(!suspicious[i]) ans[idx++] = i;
    }

    free(adj);
    free(cap);
    free(adjSize);
    free(suspicious);

    return ans;
}

int main() {
    int n, k, invocationsSize;
    scanf("%d %d %d", &n, &k, &invocationsSize);
    int** invocations = (int**)malloc(sizeof(int*) * invocationsSize);
    int* invocationsColSize = (int*)malloc(sizeof(int) * 2);
    for(int i = 0; i < invocationsSize; i++) {
        invocations[i] = (int*)malloc(sizeof(int) * 2);
        scanf("%d %d", &invocations[i][0], &invocations[i][1]);
    }
    int returnSize;
    int* ans = remainingMethods(n, k, invocations, invocationsSize, invocationsColSize, &returnSize);
    for(int i = 0; i < returnSize; i++) {
        printf("%d ", ans[i]);
    }
    printf("\n");

    free(invocationsColSize);
    for(int i = 0; i < invocationsSize; i++) {
        free(invocations[i]);
    }
    free(invocations);
    free(ans);

    return 0;
}
