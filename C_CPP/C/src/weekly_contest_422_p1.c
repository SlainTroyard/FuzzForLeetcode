// Problem: Weekly Contest 422 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Function to check if the number is balanced
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
    char num[101];  // Assuming the number is no longer than 100 digits
    
    // Input the number as a string
    scanf("%s", num);
    
    // Check if the number is balanced
    if (isBalanced(num)) {
        printf("true\n");
    } else {
        printf("false\n");
    }
    
    return 0;
}
