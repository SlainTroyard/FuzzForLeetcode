// Problem: Weekly Contest 435 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int maxDifference(char* s) {
    int cnt[26] = {0}, c1 = 0, c2 = 100;
    while (* s) 
        ++ cnt[* s ++ - 97];
    for (int i = 0; i < 26; ++ i) 
        if      (cnt[i] &  1) c1 = cnt[i] > c1 ? cnt[i] : c1;
        else if (cnt[i] != 0) c2 = cnt[i] < c2 ? cnt[i] : c2;
    return c1 - c2;
}

int main() {
    char s[100];
    scanf("%s", s);
    printf("%d\n", maxDifference(s));
    return 0;
}
