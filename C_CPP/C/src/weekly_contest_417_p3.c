// Problem: Weekly Contest 417 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

long long countOfSubstrings(char* word, int k) {
    const long long VOWEL_MASK = 1065233;
    long long ans = 0;
    
    int cnt_vowel1[26] = {0}, cnt_vowel2[26] = {0};
    int size_vowel1 = 0, size_vowel2 = 0; 
    int cnt_consonant1 = 0, cnt_consonant2 = 0;
    int left1 = 0, left2 = 0;
    
    int length = strlen(word);
    
    for (int i = 0; i < length; i++) {
        int b = word[i] - 'a';
        
        if (((VOWEL_MASK >> b) & 1)) {
            if (cnt_vowel1[b]++ == 0) {
                size_vowel1++;
            }
            if (cnt_vowel2[b]++ == 0) {
                size_vowel2++;
            }
        } else {
            cnt_consonant1++;
            cnt_consonant2++;
        }
        
        while (size_vowel1 == 5 && cnt_consonant1 >= k) {
            int out = word[left1] - 'a';
            if (((VOWEL_MASK >> out) & 1)) {
                if (--cnt_vowel1[out] == 0) {
                    size_vowel1--;
                }
            } else {
                cnt_consonant1--;
            }
            left1++;
        }
        
        while (size_vowel2 == 5 && cnt_consonant2 > k) {
            int out = word[left2] - 'a';
            if (((VOWEL_MASK >> out) & 1)) {
                if (--cnt_vowel2[out] == 0) {
                    size_vowel2--;
                }
            } else {
                cnt_consonant2--;
            }
            left2++;
        }
        
        ans += (long long)(left1 - left2);
    }
    
    return ans;
}

int main() {
    int wordSize, k;
    scanf("%d", &wordSize);
    char *word = (char *)malloc(wordSize + 1);
    scanf("%s", word);
    scanf("%d", &k);
    printf("%lld\n", countOfSubstrings(word, k));
    free(word);
    return 0;
}
