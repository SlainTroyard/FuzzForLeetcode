// Problem: Weekly Contest 426 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>


void buildAdjacencyList(int** edges, int edgesSize, int* edgesColSize, int n, int*** adjList, int** adjLen) {
    *adjList = (int**)malloc(n * sizeof(int*));
    *adjLen = (int*)calloc(n, sizeof(int));

    for (int i = 0; i < edgesSize; i++) {
        (*adjLen)[edges[i][0]]++;
        (*adjLen)[edges[i][1]]++;
    }

    for (int i = 0; i < n; i++) {
        (*adjList)[i] = (int*)malloc((*adjLen)[i] * sizeof(int));
        (*adjLen)[i] = 0; 
    }

    for (int i = 0; i < edgesSize; i++) {
        int u = edges[i][0];
        int v = edges[i][1];
        (*adjList)[u][(*adjLen)[u]++] = v;
        (*adjList)[v][(*adjLen)[v]++] = u;
    }
}


void bfsCount(int** adjList, int* adjLen, int n, int* colorCount, int* nodeColor) {
    bool* visited = (bool*)calloc(n, sizeof(bool));
    int* queue = (int*)malloc(n * sizeof(int));
    int front = 0, rear = 0;

    queue[rear++] = 0; 
    nodeColor[0] = 0;  
    visited[0] = true;

    while (front < rear) {
        int curr = queue[front++];
        int color = nodeColor[curr];
        colorCount[color]++;

        for (int i = 0; i < adjLen[curr]; i++) {
            int neighbor = adjList[curr][i];
            if (!visited[neighbor]) {
                visited[neighbor] = true;
                nodeColor[neighbor] = 1 - color; 
                queue[rear++] = neighbor;
            }
        }
    }

    free(visited);
    free(queue);
}


int* maxTargetNodes(int** edges1, int edges1Size, int* edges1ColSize, 
                    int** edges2, int edges2Size, int* edges2ColSize, 
                    int* returnSize) {
    int n1 = edges1Size + 1; 
    int n2 = edges2Size + 1; 

    int **adjList1, **adjList2, *adjLen1, *adjLen2;
    buildAdjacencyList(edges1, edges1Size, edges1ColSize, n1, &adjList1, &adjLen1);
    buildAdjacencyList(edges2, edges2Size, edges2ColSize, n2, &adjList2, &adjLen2);

    int colorCount1[2] = {0}, colorCount2[2] = {0};
    int* nodeColor1 = (int*)calloc(n1, sizeof(int));
    int* nodeColor2 = (int*)calloc(n2, sizeof(int));

    bfsCount(adjList1, adjLen1, n1, colorCount1, nodeColor1);
    bfsCount(adjList2, adjLen2, n2, colorCount2, nodeColor2);

    int* result = (int*)malloc(n1 * sizeof(int));
    int maxColorInTree2 = (colorCount2[0] > colorCount2[1]) ? colorCount2[0] : colorCount2[1];

    for (int i = 0; i < n1; i++) {
        result[i] = colorCount1[nodeColor1[i]] + maxColorInTree2;
    }

    *returnSize = n1;

    for (int i = 0; i < n1; i++) free(adjList1[i]);
    for (int i = 0; i < n2; i++) free(adjList2[i]);
    free(adjList1);
    free(adjList2);
    free(adjLen1);
    free(adjLen2);
    free(nodeColor1);
    free(nodeColor2);

    return result;
}

int main() {
    int n1, n2;


    scanf("%d", &n1);
    int** edges1 = (int**)malloc(n1 * sizeof(int*));
    int* edges1ColSize = (int*)malloc(n1 * sizeof(int));

    for (int i = 0; i < n1; i++) {
        edges1[i] = (int*)malloc(2 * sizeof(int));
        edges1ColSize[i] = 2;
        scanf("%d %d", &edges1[i][0], &edges1[i][1]);
    }


    scanf("%d", &n2);
    int** edges2 = (int**)malloc(n2 * sizeof(int*));
    int* edges2ColSize = (int*)malloc(n2 * sizeof(int));

    for (int i = 0; i < n2; i++) {
        edges2[i] = (int*)malloc(2 * sizeof(int));
        edges2ColSize[i] = 2;
        scanf("%d %d", &edges2[i][0], &edges2[i][1]);
    }

    int returnSize;
    int* result = maxTargetNodes(edges1, n1, edges1ColSize, edges2, n2, edges2ColSize, &returnSize);

    for (int i = 0; i < returnSize; i++) {
        printf("%d ", result[i]);
    }
    printf("\n");

    for (int i = 0; i < n1; i++) free(edges1[i]);
    for (int i = 0; i < n2; i++) free(edges2[i]);
    free(edges1);
    free(edges2);
    free(edges1ColSize);
    free(edges2ColSize);
    free(result);

    return 0;
}
