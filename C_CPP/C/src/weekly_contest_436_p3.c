// Problem: Weekly Contest 436 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

long long countSubstrings(char* s) {
    long long ans = 0;
    int f[10][9] = {0}; 
    
    int len = strlen(s);
    for (int i = 0; i < len; i++) {
        int d = s[i] - '0'; 
        
        for (int m = 1; m < 10; ++m) {
            int nf[9] = {0}; 
            nf[d % m] = 1;   
            
            for (int rem = 0; rem < m; ++rem) {
                nf[(rem * 10 + d) % m] += f[m][rem];
            }
            
            for (int rem = 0; rem < m; ++rem) {
                f[m][rem] = nf[rem];
            }
        }
        
        ans += f[d][0];
    }
    
    return ans;
}

int main() {
    char s[100001]; 
    
    if (scanf("%s", s) != 1) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    long long result = countSubstrings(s);
    
    printf("%lld\n", result);
    
    return 0;
}
