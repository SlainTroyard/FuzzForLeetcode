// Problem: Weekly Contest 417 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

char kchar_search(long long k, int* operations, int pos)
{
    long long pow_sum = 1;
    int tmp_pos = 0;
    if(!pos || 1 == k)
    {
        if(operations[pos])
        {
            return 'b';
        }
        
        return 'a';
    }

    while(k > pow_sum)
    {
        pow_sum *= 2;
        ++tmp_pos;
    }

    if(operations[pos])
    {
        char kchar = kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
        if(++kchar > 'z')
        {
            return 'a';
        }
        return kchar;
    }

    return kchar_search(k - pow_sum / 2, operations, tmp_pos - 1);
}

char kthCharacter(long long k, int* operations, int operationsSize) {
    long long pow_sum = 1;
    int pos = 0;
    
    if(1 == k)
    {
        return 'a';
    }

    while(pow_sum < k)    
    {
        pow_sum *= 2;
        ++pos;
    }
    
    return kchar_search(k - pow_sum / 2, operations, pos - 1);
}

int main() {
    long long k;
    int operationSize;
    scanf("%lld %d", &k, &operationSize);
    int* operations = (int*)malloc(sizeof(int) * operationSize);
    for(int i = 0; i < operationSize; i++) {
        scanf("%d", &operations[i]);
    }
    printf("%c\n", kthCharacter(k, operations, operationSize));
    free(operations);
    return 0;
}
