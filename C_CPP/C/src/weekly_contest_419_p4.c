// Problem: Weekly Contest 419 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct {
    int val;   
    int freq;  
} Pair;

int cmp(const void* a, const void* b) {
    Pair* pa = (Pair*)a;
    Pair* pb = (Pair*)b;
    if (pa->freq != pb->freq) return pb->freq - pa->freq;
    return pb->val - pa->val;
}


long long* findXSum(int* nums, int numsSize, int k, int x, int* returnSize) {
    *returnSize = numsSize - k + 1;
    long long* result = (long long*)malloc(*returnSize * sizeof(long long));
    if (!result) return NULL;
    
    #define TABLE_SIZE 1031  
    #define MAX_CHAIN 32     
    
    Pair** hashTable = (Pair**)malloc(TABLE_SIZE * sizeof(Pair*));
    if (!hashTable) {
        free(result);
        return NULL;
    }
    
    for (int i = 0; i < TABLE_SIZE; i++) {
        hashTable[i] = (Pair*)calloc(MAX_CHAIN, sizeof(Pair));
        if (!hashTable[i]) {
            for (int j = 0; j < i; j++) free(hashTable[j]);
            free(hashTable);
            free(result);
            return NULL;
        }
    }
    
    Pair* activeElements = (Pair*)malloc(k * sizeof(Pair));
    int activeCount = 0;

    for (int i = 0; i < k; i++) {
        int val = nums[i];
        unsigned int hash = (unsigned int)val % TABLE_SIZE;
        bool found = false;
        
        for (int j = 0; j < MAX_CHAIN; j++) {
            if (hashTable[hash][j].freq == 0) break; 
            if (hashTable[hash][j].val == val) {
                hashTable[hash][j].freq++;
                found = true;
                break;
            }
        }
        
        if (!found) {
            for (int j = 0; j < MAX_CHAIN; j++) {
                if (hashTable[hash][j].freq == 0) {
                    hashTable[hash][j].val = val;
                    hashTable[hash][j].freq = 1;
                    break;
                }
            }
        }
    }
    
    for (int i = 0; i < TABLE_SIZE; i++) {
        for (int j = 0; j < MAX_CHAIN; j++) {
            if (hashTable[i][j].freq > 0) {
                activeElements[activeCount++] = hashTable[i][j];
                if (activeCount >= k) break; 
            }
        }
        if (activeCount >= k) break;
    }
    
    qsort(activeElements, activeCount, sizeof(Pair), cmp);
    
    long long sum = 0;
    int countToSum = (activeCount < x) ? activeCount : x;
    for (int i = 0; i < countToSum; i++) {
        sum += (long long)activeElements[i].val * activeElements[i].freq;
    }
    result[0] = sum;
    
    for (int i = 1; i <= numsSize - k; i++) {
        int outVal = nums[i-1];  
        int inVal = nums[i+k-1]; 
        
        unsigned int outHash = (unsigned int)outVal % TABLE_SIZE;
        for (int j = 0; j < MAX_CHAIN; j++) {
            if (hashTable[outHash][j].freq == 0) break;
            if (hashTable[outHash][j].val == outVal) {
                hashTable[outHash][j].freq--;
                break;
            }
        }
        
        unsigned int inHash = (unsigned int)inVal % TABLE_SIZE;
        bool found = false;
        for (int j = 0; j < MAX_CHAIN; j++) {
            if (hashTable[inHash][j].freq == 0) break;
            if (hashTable[inHash][j].val == inVal) {
                hashTable[inHash][j].freq++;
                found = true;
                break;
            }
        }
        
        if (!found) {
            for (int j = 0; j < MAX_CHAIN; j++) {
                if (hashTable[inHash][j].freq == 0) {
                    hashTable[inHash][j].val = inVal;
                    hashTable[inHash][j].freq = 1;
                    break;
                }
            }
        }
        
        activeCount = 0;
        
        for (int h = 0; h < TABLE_SIZE; h++) {
            for (int j = 0; j < MAX_CHAIN; j++) {
                if (hashTable[h][j].freq > 0) {
                    activeElements[activeCount++] = hashTable[h][j];
                }
                if (activeCount >= k) break;
            }
            if (activeCount >= k) break;
        }
        
        qsort(activeElements, activeCount, sizeof(Pair), cmp);
        
        sum = 0;
        countToSum = (activeCount < x) ? activeCount : x;
        for (int j = 0; j < countToSum; j++) {
            sum += (long long)activeElements[j].val * activeElements[j].freq;
        }
        result[i] = sum;
    }
    
    for (int i = 0; i < TABLE_SIZE; i++) {
        free(hashTable[i]);
    }
    free(hashTable);
    free(activeElements);
    
    return result;
}

int main() {
    int k, x;
    scanf("%d %d", &k, &x);
    int numsSize;
    scanf("%d", &numsSize);
    int* nums = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; ++i) {
        scanf("%d", &nums[i]);
    }
    
    int returnSize;
    long long* result = findXSum(nums, numsSize, k, x, &returnSize);
    
    for (int i = 0; i < returnSize; ++i) {
        printf("%lld ", result[i]);
    }
    printf("\n");
    
    free(nums);
    free(result);
    return 0;
}

