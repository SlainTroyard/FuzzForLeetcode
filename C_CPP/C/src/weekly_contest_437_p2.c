// Problem: Weekly Contest 437 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

static int cmp(const void* a,const void* b){
    return *(int*)b-*(int*)a;
}

long long maxWeight(int* pizzas, int pizzasSize) {
    int day=pizzasSize/4;
    int even=day/2,odd=(day+1)/2;

    qsort(pizzas,pizzasSize,sizeof(int),cmp);

    long long total=0;
    int index=0;

    for(int i=0;i<odd;++i){
        total+=pizzas[index++];
    }
    
    index++;
    for(int i=0;i<even;++i){
        total+=pizzas[index];
        index+=2;
    }
    return total;
    
}

int main() {
    // TODO: Add the base I/O interface here
    int n;
    scanf("%d", &n);
    int* arr = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &arr[i]);
    }
    printf("%lld\n", maxWeight(arr, n));
    return 0;
}
