// Problem: Weekly Contest 426 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int linepots(int k, int** pots, int node, int* length, int visited) {
    if(k == -1) return 0; 
    if(k == 0) return 1;  
    int answer = 1;        
    for(int i = 0; i < length[node]; i++) {
        if(pots[node][i] != visited) {
            answer += linepots(k - 1, pots, pots[node][i], length, node); 
        }
    }
    return answer; 
}

int* maxTargetNodes(int** edges1, int edges1Size, int* edges1ColSize, 
                    int** edges2, int edges2Size, int* edges2ColSize, 
                    int k, int* returnSize) {
    int len1 = 0, len2 = 0;
    
    for(int i = 0; i < edges1Size; i++) if(edges1[i][1] > len1) len1 = edges1[i][1];
    for(int i = 0; i < edges2Size; i++) if(edges2[i][1] > len2) len2 = edges2[i][1];
    
    int **pots1 = (int**)malloc((len1 + 1) * sizeof(int*)),
        **pots2 = (int**)malloc((len2 + 1) * sizeof(int*)),
        *answer = (int*)calloc(len1 + 1, sizeof(int)),
        *length1 = (int*)calloc(len1 + 1, sizeof(int)),
        *length2 = (int*)calloc(len2 + 1, sizeof(int));

    for(int i = 0; i <= len1; i++) {
        int add = 0, *ccc = (int*)malloc((len1 + 1) * sizeof(int));
        for(int j = 0; j < edges1Size; j++) {
            if(edges1[j][0] == i) ccc[add++] = edges1[j][1];  
            if(edges1[j][1] == i) ccc[add++] = edges1[j][0];
        }
        pots1[i] = (int*)malloc(add * sizeof(int));  
        length1[i] = add;
        for(int j = 0; j < add; j++) pots1[i][j] = ccc[j];
        free(ccc);
    }

    for(int i = 0; i <= len2; i++) {
        int add = 0, *ccc = (int*)malloc((len2 + 1) * sizeof(int));
        for(int j = 0; j < edges2Size; j++) {
            if(edges2[j][0] == i) ccc[add++] = edges2[j][1];
            if(edges2[j][1] == i) ccc[add++] = edges2[j][0];
        }
        pots2[i] = (int*)malloc(add * sizeof(int));  
        length2[i] = add;
        for(int j = 0; j < add; j++) pots2[i][j] = ccc[j];
        free(ccc);
    }

    int max = 0;
    for(int i = 0; i <= len2; i++) {
        int temp = linepots(k - 1, pots2, i, length2, -1);
        if(temp > max) max = temp;
    }

    for(int i = 0; i <= len1; i++) {
        answer[i] = linepots(k, pots1, i, length1, -1) + max;
    }

    *returnSize = len1 + 1;
    return answer;
}

int main() {
    int n1, n2, k;
    
    scanf("%d", &n1);
    int edges1[n1][2];
    for (int i = 0; i < n1; ++i) {
        scanf("%d %d", &edges1[i][0], &edges1[i][1]);
    }

    scanf("%d", &n2);
    int edges2[n2][2];
    for (int i = 0; i < n2; ++i) {
        scanf("%d %d", &edges2[i][0], &edges2[i][1]);
    }

    scanf("%d", &k);

    int **edges1Ptr = (int**)malloc(n1 * sizeof(int*));
    int **edges2Ptr = (int**)malloc(n2 * sizeof(int*));
    for (int i = 0; i < n1; i++) {
        edges1Ptr[i] = edges1[i];
    }
    for (int i = 0; i < n2; i++) {
        edges2Ptr[i] = edges2[i];
    }

    int returnSize = 0;

    int* result = maxTargetNodes(edges1Ptr, n1, NULL, edges2Ptr, n2, NULL, k, &returnSize);

    for (int i = 0; i < returnSize; i++) {
        printf("%d ", result[i]);
    }
    printf("\n");

    free(result);
    free(edges1Ptr);
    free(edges2Ptr);

    return 0;
}
