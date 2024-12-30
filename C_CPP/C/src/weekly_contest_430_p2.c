// Problem: Weekly Contest 430 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>


char* answerString(char* word, int numFriends) {
    if (numFriends == 1) return word;
    int len = strlen(word), n = len - numFriends + 1, ans = 0;
    for (int i = 0; i < len; ++i)
        if (strncmp(word + i, word + ans, n) > 0)
            ans = i;
    if (ans + n < len)
        *(word + ans + n) = '\0';
    return word + ans;
}

int main() {
    char word[5001];
    int numFriends;

    // Input word and numFriends
    scanf("%s", word);
    scanf("%d", &numFriends);

    // Call answerString function
    char* result = answerString(word, numFriends);

    // Print the result
    printf("%s\n", result);
    return 0;
}