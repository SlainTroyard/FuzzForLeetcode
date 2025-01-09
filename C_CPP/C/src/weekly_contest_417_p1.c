// Problem: Weekly Contest 417 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char kthCharacter(int k) {
    for (int * l = (int *)calloc(k + 1, sizeof(int)), a = 0; true; ++ a) 
        for (int i = 0, j = 1 << a; i < 1 << a; ++ i) 
            if (l[i + j] = l[i] + 1, i + j >= k - 1) 
                return 97 + l[k - 1] % 26;
}

int main() {
    int k = 0;
    scanf("%d", &k);
    printf("%c\n", kthCharacter(k));
    return 0;
}
