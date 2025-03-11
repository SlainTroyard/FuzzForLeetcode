// Problem: Weekly Contest 420 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int numberOfSubstrings(char* s, int k) {
    int hash_arr[26] = {0};
    int left = 0, right = 0;
    int s_l = strlen(s);
    int res = 0;
    
    while(left < s_l && right < s_l)
    {
        if(++hash_arr[s[right] - 'a'] == k)
        {
            res += s_l - right;
            while(left <= right)
            {
                if(--hash_arr[s[left++] - 'a'] != k - 1)
                {
                    res += s_l - right;
                }
                else
                {
                    break;
                }
            }
            ++right;
        }
        else
        {
            ++right;
        }
    }

    return res;
}


int main() {
    char s[3000];
    int k;
    scanf("%s %d", s, &k);
    printf("%d\n", numberOfSubstrings(s, k));
    return 0;
}
