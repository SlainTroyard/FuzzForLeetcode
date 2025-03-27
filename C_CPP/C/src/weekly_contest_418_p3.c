// Problem: Weekly Contest 418 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>


int** constructGridLayout(int n, int** edges, int edgesSize, int* edgesColSize, int* returnSize, int** returnColumnSizes) {
    if(n==edgesSize+1){
        *returnSize=1;
        *returnColumnSizes=malloc(sizeof(int)*n);
        int**res=(int **)malloc(sizeof(int*));
        res[0]=(int*)malloc(sizeof(int)*n);
        (*returnColumnSizes)[0]=n;
        int son[n][2]={},sou[n]={};
        for(int i=0;i<edgesSize;i++){
            int a=edges[i][0],b=edges[i][1];
            son[a][sou[a]++]=b;
            son[b][sou[b]++]=a;
        }
        for(int i=0;;i++){
            if(sou[i]==1){
                res[0][0]=i;
                res[0][1]=son[i][0];
                for(int j=2;j<n;j++){
                    int a=res[0][j-1];
                    res[0][j]=son[a][0]+son[a][1]-res[0][j-2];
                }
                break;
            }
        }
        return res;
    }
    int son[n][4]={},sou[n]={},len=2;
    for(int i=0;i<edgesSize;i++){
        int a=edges[i][0],b=edges[i][1];
        son[a][sou[a]++]=b;
        son[b][sou[b]++]=a;
    }
    int num=2,mai[n]={},fow[n]={};
    for(int i=0;;i++){
        if(sou[i]==2){
            mai[0]=i;
            mai[1]=son[i][0];
            fow[0]=son[i][1];
            for(int j=1;;j++){
                int c=mai[j],d=fow[j-1];
                if(sou[c]!=2){
                    for(int k=0;k<2;k++){
                        if(son[c][k]==mai[j-1]){
                            son[c][k]=son[c][2];
                            break;
                        }
                    }
                    bool flag=false;
                    for(int k=0;k<sou[d];k++){
                        if(son[c][0]==son[d][k]){
                            flag=true;
                            break;
                        }
                    }                   
                    fow[j]=son[c][!flag];
                    mai[j+1]=son[c][!!flag];
                    num++;
                }
                else{
                    fow[j]=son[c][0]+son[c][1]-mai[j-1];
                    break;
                }
            }
            break;
        }       
    }
    *returnSize=n/num;
    *returnColumnSizes=malloc(sizeof(int)* *returnSize);
    int**res=(int**)malloc(sizeof(int*)* *returnSize);
    for(int i=0;i<*returnSize;i++){
        res[i]=(int*)malloc(sizeof(int)* num);
        (*returnColumnSizes)[i]=num;
    }
    for(int i=0;i<num;i++){
        res[0][i]=mai[i];
        res[1][i]=fow[i];
    }
    for(int i=2;i<*returnSize;i++){
        int a=res[i-1][0],b=res[i-1][num-1]; 
        res[i][0]=son[a][0]+son[a][1]+son[a][2]-res[i-1][1]-res[i-2][0];
        res[i][num-1]=son[b][0]+son[b][1]+son[b][2]-res[i-1][num-2]-res[i-2][num-1];
        for(int j=1;j<num-1;j++){
            int c=res[i-1][j];
            res[i][j]=son[c][0]+son[c][1]+son[c][2]+son[c][3]-res[i-2][j]-res[i-1][j+1]-res[i-1][j-1];
        }
    }
    return res;
}

int main() {
    int n, edgesSize;
    scanf("%d %d", &n, &edgesSize);
    int **edges = (int **)malloc(sizeof(int *) * edgesSize);
    int *edgesColSize = (int *)malloc(sizeof(int) * 2);
    for (int i = 0; i < edgesSize; i++) {
        edges[i] = (int *)malloc(sizeof(int) * 2);
        scanf("%d %d", &edges[i][0], &edges[i][1]);
    }
    int returnSize, *returnColumnSizes;
    int **res = constructGridLayout(n, edges, edgesSize, edgesColSize, &returnSize, &returnColumnSizes);
    for (int i = 0; i < returnSize; i++) {
        for (int j = 0; j < returnColumnSizes[i]; j++) {
            printf("%d ", res[i][j]);
        }
        printf("\n");
    }
    free(edgesColSize);
    for (int i = 0; i < edgesSize; i++) {
        free(edges[i]);
    }
    free(edges);
    for (int i = 0; i < returnSize; i++) {
        free(res[i]);
    }
    free(res);
    free(returnColumnSizes);
    return 0;
}
