// Problem: Weekly Contest 422 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool isBalanced(char* num) {
    int total = 0;
    while (*num) {
        total += '0' - *num;
        total = -total;
        ++num;
    }
    return total == 0;
}

int main() {
    char num[101];  
    
    scanf("%s", num);
    
    if (isBalanced(num)) {
        printf("true\n");
    } else {
        printf("false\n");
    }
    
    return 0;
}
