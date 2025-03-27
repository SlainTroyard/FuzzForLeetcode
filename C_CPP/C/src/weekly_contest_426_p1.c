// Problem: Weekly Contest 426 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>

int smallestNumber(int n) {
    int b = (int)(log2(n)) + 1; 
    return (1 << b) - 1;        
}

int main() {
    int n;

    scanf("%d", &n);

    int result = smallestNumber(n);

    printf("%d\n", result);

    return 0;
}
