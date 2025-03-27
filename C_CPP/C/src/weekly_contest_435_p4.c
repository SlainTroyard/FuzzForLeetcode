// Problem: Weekly Contest 435 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int max(int a, int b) {
    return a > b ? a : b;
}

int min(int a, int b) {
    return a < b ? a : b;
}

int maxDifference(char* s, int k) {
    const int inf = INT_MAX / 2;
    int ans = -inf;
    int len = strlen(s);
    
    for (int x = 0; x < 5; x++) {
        for (int y = 0; y < 5; y++) {
            if (y == x) {
                continue;
            }
            
            int cur_s[5] = {0}; 
            int pre_s[5] = {0}; 
            int min_s[2][2] = {{inf, inf}, {inf, inf}}; 
            int left = 0;
            
            for (int i = 0; i < len; i++) {
                cur_s[s[i] - '0']++; 
                int r = i + 1;
                
                while (r - left >= k && cur_s[x] > pre_s[x] && cur_s[y] > pre_s[y]) {
                    int* p = &min_s[pre_s[x] & 1][pre_s[y] & 1];
                    *p = min(*p, pre_s[x] - pre_s[y]); 
                    pre_s[s[left] - '0']++; 
                    left++;
                }
                
                ans = max(ans, cur_s[x] - cur_s[y] - min_s[(cur_s[x] & 1) ^ 1][cur_s[y] & 1]);
            }
        }
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
    
    int result = maxDifference(s, k);
    
    printf("%d\n", result);
    
    return 0;
}
