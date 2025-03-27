// Problem: Weekly Contest 434 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

int popcount(int n) {
    int count = 0;
    while (n) {
        count += n & 1;
        n >>= 1;
    }
    return count;
}

bool hasCycle(int sub, int g[26][26], int gSize[26]) {
    int color[26] = {0}; 
    
    for (int start = 0; start < 26; start++) {
        if ((sub >> start & 1) == 0 || color[start] == 2) {
            continue;
        }
        
        int stack[26];
        int stackPos[26]; 
        int top = 0;
        
        stack[top] = start;
        stackPos[top] = 0;
        color[start] = 1; 
        
        while (top >= 0) {
            int x = stack[top];
            
            if (stackPos[top] >= gSize[x]) {
                color[x] = 2; 
                top--;
                continue;
            }
            
            int y = g[x][stackPos[top]++];
            
            if ((sub >> y & 1) == 0) {
                continue;
            }
            
            if (color[y] == 1) {
                return true;
            }
            
            if (color[y] == 0) {
                color[y] = 1; 
                stack[++top] = y;
                stackPos[top] = 0;
            }
        }
    }
    
    return false;
}


int** supersequences(char** words, int wordsSize, int* returnSize, int** returnColumnSizes) {
    int all = 0, mask2 = 0;
    int g[26][26]; 
    int gSize[26] = {0}; 
    
    for (int i = 0; i < wordsSize; i++) {
        int x = words[i][0] - 'a';
        int y = words[i][1] - 'a';
        
        all |= (1 << x) | (1 << y);
        
        if (x == y) {
            mask2 |= (1 << x);
        }
        
        g[x][gSize[x]++] = y;
    }
    
    int mask1 = all ^ mask2;
    
    int* validSubsets = NULL;
    int validSubsetsCapacity = 0;
    int validSubsetsCount = 0;
    int maxSize = 0;
    
    int sub = mask1;
    do {
        int size = popcount(sub);
        
        if (size >= maxSize && !hasCycle(sub, g, gSize)) {
            if (size > maxSize) {
                maxSize = size;
                validSubsetsCount = 0;
            }
            
            if (validSubsetsCount >= validSubsetsCapacity) {
                validSubsetsCapacity = validSubsetsCapacity == 0 ? 16 : validSubsetsCapacity * 2;
                int* newArray = (int*)realloc(validSubsets, validSubsetsCapacity * sizeof(int));
                if (!newArray) {
                    free(validSubsets);
                    *returnSize = 0;
                    return NULL;
                }
                validSubsets = newArray;
            }
            
            validSubsets[validSubsetsCount++] = sub;
        }
        
        if (sub == 0) break;
        sub = (sub - 1) & mask1;
    } while (sub != mask1);
    
    *returnSize = validSubsetsCount;
    int** result = NULL;
    
    if (validSubsetsCount > 0) {
        result = (int**)malloc(validSubsetsCount * sizeof(int*));
        if (!result) {
            free(validSubsets);
            *returnSize = 0;
            return NULL;
        }
        
        *returnColumnSizes = (int*)malloc(validSubsetsCount * sizeof(int));
        if (!(*returnColumnSizes)) {
            free(validSubsets);
            free(result);
            *returnSize = 0;
            return NULL;
        }
        
        for (int i = 0; i < validSubsetsCount; i++) {
            int sub = validSubsets[i];
            result[i] = (int*)malloc(26 * sizeof(int));
            if (!result[i]) {
                for (int j = 0; j < i; j++) {
                    free(result[j]);
                }
                free(*returnColumnSizes);
                free(result);
                free(validSubsets);
                *returnSize = 0;
                return NULL;
            }
            
            (*returnColumnSizes)[i] = 26;
            
            for (int j = 0; j < 26; j++) {
                result[i][j] = ((all >> j) & 1) + (((all ^ sub) >> j) & 1);
            }
        }
    }
    
    free(validSubsets);
    
    return result;
}

int main() {
    int n;
    if (scanf("%d", &n) != 1) {
        fprintf(stderr, "Error reading input for n\n");
        return 1;
    }
    
    char** words = (char**)malloc(n * sizeof(char*));
    if (!words) {
        fprintf(stderr, "Memory allocation failed for words array\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        words[i] = (char*)malloc(3 * sizeof(char)); 
        if (!words[i]) {
            fprintf(stderr, "Memory allocation failed for words[%d]\n", i);
            for (int j = 0; j < i; j++) {
                free(words[j]);
            }
            free(words);
            return 1;
        }
        
        if (scanf("%2s", words[i]) != 1) {
            fprintf(stderr, "Error reading input for words[%d]\n", i);
            for (int j = 0; j <= i; j++) {
                free(words[j]);
            }
            free(words);
            return 1;
        }
    }
    
    int returnSize;
    int* returnColumnSizes;
    int** result = supersequences(words, n, &returnSize, &returnColumnSizes);
    
    for (int i = 0; i < returnSize; i++) {
        for (int j = 0; j < returnColumnSizes[i]; j++) {
            printf("%d ", result[i][j]);
        }
        printf("\n");
    }
    
    for (int i = 0; i < returnSize; i++) {
        free(result[i]);
    }
    free(result);
    free(returnColumnSizes);
    
    for (int i = 0; i < n; i++) {
        free(words[i]);
    }
    free(words);
    
    return 0;
}
