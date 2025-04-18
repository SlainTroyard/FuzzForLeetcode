// Problem: Weekly Contest 416 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

void update(int *diff, int c, int add, int *cnt) {
    diff[c] += add;
    if (add == 1 && diff[c] == 0) {
        (*cnt)--;
    } else if (add == -1 && diff[c] == -1) {
        (*cnt)++;
    }
}

long long validSubstringCount(char* word1, char* word2) {
    int diff[26] = {0};
    for (const char *c = word2; *c; c++) {
        diff[*c - 'a']--;
    }

    int cnt = 0;
    for (int i = 0; i < 26; i++) {
        if (diff[i] < 0) {
            cnt++;
        }
    }
    long long res = 0;
    int l = 0, r = 0;
    int len1 = strlen(word1);
    while (l < len1) {
        while (r < len1 && cnt > 0) {
            update(diff, word1[r] - 'a', 1, &cnt);
            r++;
        }
        if (cnt == 0) {
            res += len1 - r + 1;
        }
        update(diff, word1[l] - 'a', -1, &cnt);
        l++;
    }

    return res;
}

int main() {
    int len1 = 0, len2 = 0;
    scanf("%d", &len1);
    char *word1 = (char *)malloc(len1 + 1);
    scanf("%s", word1);
    scanf("%d", &len2);
    char *word2 = (char *)malloc(len2 + 1);
    scanf("%s", word2);
    printf("%lld\n", validSubstringCount(word1, word2));
    free(word1);
    free(word2);
    return 0;
}

