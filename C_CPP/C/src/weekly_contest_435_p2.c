// Problem: Weekly Contest 435 Problem 2
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

int abs_val(int x) {
    return x < 0 ? -x : x;
}

int min(int a, int b) {
    return a < b ? a : b;
}

int max(int a, int b) {
    return a > b ? a : b;
}

int maxDistance(char* s, int k) {
    int ans = 0, x = 0, y = 0;
    int length = strlen(s);
    
    for (int i = 0; i < length; i++) {
        if (s[i] == 'N') y++;
        else if (s[i] == 'S') y--;
        else if (s[i] == 'E') x++;
        else if (s[i] == 'W') x--;
        
        int current_max = min(abs_val(x) + abs_val(y) + k * 2, i + 1);
        ans = max(ans, current_max);
    }
    
    return ans;
}

int main() {
    char s[100001]; 
    int k;
    
    if (scanf("%s %d", s, &k) != 2) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    int result = maxDistance(s, k);
    
    printf("%d\n", result);
    
    return 0;
}
