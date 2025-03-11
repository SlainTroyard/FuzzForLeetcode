// Problem: Weekly Contest 416 Problem 3
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int get(int l, int r, int preCount[][26], int* count) {
    int border = l;
    while (l < r) {
        int m = (l + r) >> 1;
        int f = 1;
        for (int i = 0; i < 26; i++) {
            if (preCount[m][i] - preCount[border - 1][i] < count[i]) {
                f = 0;
                break;
            }
        }
        if (f) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    return l;
}

long long validSubstringCount(char* word1, char* word2) {
    int count[26] = {0};
    for (int i = 0; word2[i]; i++) {
        count[word2[i] - 'a']++;
    }

    int n = strlen(word1);
    int preCount[n + 1][26];
    memset(preCount, 0, sizeof(preCount));
    for (int i = 1; i <= n; i++) {
        memcpy(preCount[i], preCount[i - 1], sizeof(preCount[i]));
        preCount[i][word1[i - 1] - 'a']++;
    }

    long long res = 0;
    for (int l = 1; l <= n; l++) {
        int r = get(l, n + 1, preCount, count);
        res += n - r + 1;
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
