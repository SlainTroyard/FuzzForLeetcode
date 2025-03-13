// Problem: Weekly Contest 426 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <stdio.h>
#include <math.h>

int smallestNumber(int n) {
    int b = (int)(log2(n)) + 1; // Calculate the number of bits
    return (1 << b) - 1;        // Return 2^b - 1
}

int main() {
    int n;

    // Input the number
    scanf("%d", &n);

    // Calculate the smallest number with the same number of bits as `n`
    int result = smallestNumber(n);

    // Output the result
    printf("%d\n", result);

    return 0;
}
