// Problem: Weekly Contest 437 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool hasSpecialSubstring(char* s, int k) {
    int length = strlen(s);
    int consecutive = 0;
    for (int i = 0; i < length; i++) {
        consecutive++;
        if (i == length - 1 || s[i] != s[i + 1]) {
            if (consecutive == k) {
                return true;
            } else {
                consecutive = 0;
            }
        }
    }
    return false;
}

int main() {
    // TODO: Add the base I/O interface here
    char s[100];
    int k;
    scanf("%s %d", s, &k);
    printf("%d\n", hasSpecialSubstring(s, k));
    return 0;
}
