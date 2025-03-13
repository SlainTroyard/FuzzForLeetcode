// Problem: Weekly Contest 416 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int cmp(const void *a, const void *b){
    return strcmp(*(char**)a, *(char**)b);
}

bool reportSpam(char** message, int messageSize, char** bannedWords, int bannedWordsSize){
    qsort(bannedWords, bannedWordsSize, sizeof(char*), cmp);
    int cnt = 0;
    for (int i = 0; i < messageSize; i++)
    {
        if (bsearch(&message[i], bannedWords, bannedWordsSize, sizeof(char*), cmp) != NULL)
        {
            cnt++;
        }
        if (cnt >= 2)
        {
            return true;
        }
    }
    return false;
}

int main() {
    int messageSize, bannedWordsSize;
    scanf("%d", &messageSize);
    char **message = (char**)malloc(sizeof(char*) * messageSize);
    for (int i = 0; i < messageSize; i++)
    {
        message[i] = (char*)malloc(sizeof(char) * 100);
        scanf("%s", message[i]);
    }
    scanf("%d", &bannedWordsSize);
    char **bannedWords = (char**)malloc(sizeof(char*) * bannedWordsSize);
    for (int i = 0; i < bannedWordsSize; i++)
    {
        bannedWords[i] = (char*)malloc(sizeof(char) * 100);
        scanf("%s", bannedWords[i]);
    }
    if (reportSpam(message, messageSize, bannedWords, bannedWordsSize))
    {
        printf("true\n");
    }
    else
    {
        printf("false\n");
    }
    free(message);
    free(bannedWords);
    return 0;
}
