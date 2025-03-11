// Problem: Weekly Contest 419 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

typedef struct {
    int value;
    int count;
} Element;

int cmp(const void* a, const void* b) {
    Element* ea = (Element*)a;
    Element* eb = (Element*)b;
    if (ea->count == eb->count) {
        return eb->value - ea->value; 
    }
    return eb->count - ea->count; 
}

int* findXSum(int* nums, int numsSize, int k, int x, int* returnSize) {
    int numResults = numsSize - k + 1;
    int* answer = (int*)malloc(numResults * sizeof(int));
    *returnSize = numResults;

    for (int i = 0; i < numResults; i++) {
        int hash[51] = {0};
        for (int j = 0; j < k; j++) {
            hash[nums[i + j]]++;
        }

        Element elements[51];
        int elementCount = 0;
        for (int j = 0; j < 51; j++) {
            if (hash[j] > 0) {
                elements[elementCount].value = j;
                elements[elementCount].count = hash[j];
                elementCount++;
            }
        }

        qsort(elements, elementCount, sizeof(Element), cmp);

        int sum = 0;
        int elementsToSum = (elementCount < x) ? elementCount : x;
        for (int j = 0; j < elementsToSum; j++) {
            sum += elements[j].value * elements[j].count;
        }

        answer[i] = sum;
    }

    return answer;
}

int main() {
    int k, x;
    scanf("%d %d", &k, &x);
    int numsSize;
    scanf("%d", &numsSize);
    int* nums = (int*)malloc(numsSize * sizeof(int));
    for (int i = 0; i < numsSize; i++) {
        scanf("%d", &nums[i]);
    }
    int returnSize;
    int* answer = findXSum(nums, numsSize, k, x, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        printf("%d ", answer[i]);
    }
    printf("\n");
    free(nums);
    free(answer);
    return 0;
}
