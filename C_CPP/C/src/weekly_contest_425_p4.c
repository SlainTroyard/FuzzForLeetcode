// Problem: Weekly Contest 425 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MAX_NODES 100005
#define MAX_EDGES 200010
#define MAX_STACK 100005

typedef struct {
    int to;     
    int weight; 
    int next;   
} Edge;

typedef struct {
    int node;      
    int parent;    
    int processed; 
} StackNode;

Edge edgeList[MAX_EDGES];
int headList[MAX_NODES];
int edgeCount = 0;

long long dp0_arr[MAX_NODES];
long long dp1_arr[MAX_NODES];

int cmp_desc(const void* a, const void* b) {
    long long aa = *((long long*)a);
    long long bb = *((long long*)b);
    if (aa < bb) return 1;
    if (aa > bb) return -1;
    return 0;
}


void add_edge(int u, int v, int w) {
    edgeList[edgeCount].to = v;
    edgeList[edgeCount].weight = w;
    edgeList[edgeCount].next = headList[u];
    headList[u] = edgeCount++;
    
    edgeList[edgeCount].to = u;
    edgeList[edgeCount].weight = w;
    edgeList[edgeCount].next = headList[v];
    headList[v] = edgeCount++;
}


long long maximizeSumOfWeights(int** edges, int edgesSize, int* edgesColSize, int k) {
    int n = edgesSize + 1;
    
    for(int i = 0; i < n; i++) {
        headList[i] = -1;
    }
    edgeCount = 0;
    
    for(int i = 0; i < edgesSize; i++) {
        int u = edges[i][0];
        int v = edges[i][1];
        int w = edges[i][2];
        add_edge(u, v, w);
    }
    
    StackNode stack[MAX_STACK];
    int top = 0;
    
    stack[top].node = 0;
    stack[top].parent = -1; 
    stack[top].processed = 0;
    top++;
    
    while(top > 0){
        StackNode current = stack[--top];
        int node = current.node;
        int parent = current.parent;
        
        if(!current.processed){
            stack[top].node = node;
            stack[top].parent = parent;
            stack[top].processed = 1;
            top++;
            
            int edge_idx = headList[node];
            while(edge_idx != -1){
                int child = edgeList[edge_idx].to;
                if(child != parent){
                    stack[top].node = child;
                    stack[top].parent = node;
                    stack[top].processed = 0;
                    top++;
                }
                edge_idx = edgeList[edge_idx].next;
            }
        }
        else{
            int children_count = 0;
            int edge_idx = headList[node];
            
            while(edge_idx != -1){
                int child = edgeList[edge_idx].to;
                if(child != parent){
                    children_count++;
                }
                edge_idx = edgeList[edge_idx].next;
            }
            
            long long* gains = (long long*)malloc(sizeof(long long) * children_count);
            if(gains == NULL){
                exit(1);
            }
            int idx = 0;
            edge_idx = headList[node];
            long long sum_dp0 = 0;
            
            while(edge_idx != -1){
                int child = edgeList[edge_idx].to;
                int weight = edgeList[edge_idx].weight;
                if(child != parent){
                    gains[idx] = (long long)weight + dp1_arr[child] - dp0_arr[child];
                    sum_dp0 += dp0_arr[child];
                    idx++;
                }
                edge_idx = edgeList[edge_idx].next;
            }
            
            qsort(gains, children_count, sizeof(long long), cmp_desc);
            
            long long sum0 = sum_dp0;
            for(int i = 0; i < children_count && i < k; i++){
                if(gains[i] > 0){
                    sum0 += gains[i];
                }
                else{
                    break;
                }
            }
            dp0_arr[node] = sum0;
            
            if(k - 1 < 0){
                dp1_arr[node] = 0;
            }
            else{
                long long sum1 = sum_dp0;
                for(int i = 0; i < children_count && i < (k - 1); i++){
                    if(gains[i] > 0){
                        sum1 += gains[i];
                    }
                    else{
                        break;
                    }
                }
                dp1_arr[node] = sum1;
            }
            
            free(gains);
        }
    }
    
    return dp0_arr[0];
}

int main(){
    int n, k;
    
    scanf("%d %d", &n, &k);

    int edgesSize = n - 1;
    
    int** edges = (int**)malloc(edgesSize * sizeof(int*));
    for(int i = 0; i < edgesSize; i++){
        edges[i] = (int*)malloc(3 * sizeof(int));
    }
    
    for(int i = 0; i < edgesSize; i++) {
        scanf("%d %d %d", &edges[i][0], &edges[i][1], &edges[i][2]);
    }
    
    int* edgesColSize = (int*)malloc(edgesSize * sizeof(int));
    for(int i = 0; i < edgesSize; i++) edgesColSize[i] = 3;
    
    long long result = maximizeSumOfWeights(edges, edgesSize, edgesColSize, k);
    
    printf("%lld\n", result);
    
    for(int i = 0; i < edgesSize; i++) free(edges[i]);
    free(edges);
    free(edgesColSize);
    
    return 0;
}
