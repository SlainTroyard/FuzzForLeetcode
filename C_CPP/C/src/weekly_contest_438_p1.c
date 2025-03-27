// Problem: Weekly Contest 438 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool hasSameDigits(char* s) {
    int len = strlen(s), i = 0;
    while (i < len) s[i ++] &= 0x0f;
    while (-- len > 1)
        for (i = 0; i < len; ++ i)
            s[i] = (s[i] + s[i + 1]) % 10;
    return s[0] == s[1];
}

int main() {
    char s[100];
    scanf("%s", s);
    printf("%d\n", hasSameDigits(s));
    return 0;
}
