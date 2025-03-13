// Problem: Weekly Contest 421 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int lengthAfterTransformations(char* s, int t) {
    int lst[26] = {0}, z, i, ans = 0;
    while (* s) 
        ++ lst[* s ++ - 'a'];
    while (t --) {
        for (i = 25, z = lst[25]; i > 1; -- i) 
            lst[i] = lst[i - 1];            
        lst[1] = (lst[0] + z) % 1000000007;
        lst[0] = z;
    }
    for (i = 0; i < 26; ++ i) 
        ans = (ans + lst[i]) % 1000000007;
    return ans;
}

int main() {
    char s[100000];
    int t;
    scanf("%s", s);
    scanf("%d", &t);
    printf("%d", lengthAfterTransformations(s, t));
    return 0;
}
