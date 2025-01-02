// Problem: Weekly Contest 425 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int LEN;

int cmp(const void *a, const void *b) {
    return strncmp((char *)a, (char *)b, LEN);
}

bool isPossibleToRearrange(char* s, char* t, int k) {
    LEN = strlen(s) / k;
    qsort(s, k, LEN * sizeof(char), cmp);
    qsort(t, k, LEN * sizeof(char), cmp);
    return strcmp(s, t) == 0;
}

int main() {
    // Input the strings s, t, and the integer k
    char s[1001], t[1001];
    int k;

    scanf("%s", s);

    scanf("%s", t);

    scanf("%d", &k);

    // Check if it is possible to rearrange the strings
    if (isPossibleToRearrange(s, t, k)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    return 0;
}
