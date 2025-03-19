// Problem: Weekly Contest 431 Problem 2
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Stack implementation for storing positions
typedef struct {
    int* data;
    int size;
    int capacity;
} Stack;

void initStack(Stack* s, int capacity) {
    s->data = (int*)malloc(capacity * sizeof(int));
    s->size = 0;
    s->capacity = capacity;
}

void pushStack(Stack* s, int value) {
    if (s->size < s->capacity) {
        s->data[s->size++] = value;
    }
}

int popStack(Stack* s) {
    if (s->size > 0) {
        return s->data[--s->size];
    }
    return -1; // Error case, shouldn't happen
}

int topStack(Stack* s) {
    if (s->size > 0) {
        return s->data[s->size - 1];
    }
    return -1; // Empty stack
}

int isEmptyStack(Stack* s) {
    return s->size == 0;
}

void freeStack(Stack* s) {
    free(s->data);
    s->size = 0;
    s->capacity = 0;
}

// Main solution function
long long calculateScore(char* s) {
    int len = strlen(s);
    Stack stacks[26]; // One stack for each letter
    long long ans = 0;
    
    // Initialize all stacks
    for (int i = 0; i < 26; i++) {
        initStack(&stacks[i], len); // Maximum possible size is the length of the string
    }
    
    for (int i = 0; i < len; i++) {
        int c = s[i] - 'a';
        if (!isEmptyStack(&stacks[25 - c])) {
            // Found a match - calculate score and pop
            ans += i - topStack(&stacks[25 - c]);
            popStack(&stacks[25 - c]);
        } else {
            // No match - push current position
            pushStack(&stacks[c], i);
        }
    }
    
    // Clean up
    for (int i = 0; i < 26; i++) {
        freeStack(&stacks[i]);
    }
    
    return ans;
}

int main() {
    char s[100001]; // Maximum length of the string plus 1 for null terminator
    scanf("%s", s);
    
    printf("%lld\n", calculateScore(s));
    
    return 0;
}
