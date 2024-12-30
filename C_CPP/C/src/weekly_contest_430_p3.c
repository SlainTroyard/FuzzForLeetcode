#include <stdio.h>
#include <stdlib.h>
#include <math.h>

long long numberOfSubsequences(int* nums, int numsSize) {
    #define HASH_SIZE 100003

    typedef struct HashNode {
        int key;
        int value;
        struct HashNode* next;
    } HashNode;

    HashNode* hashTable[HASH_SIZE] = {NULL};

    unsigned int hash(int key) {
        return ((unsigned int)key) % HASH_SIZE;
    }

    void hash_insert(int key, int value) {
        unsigned int h = hash(key);
        HashNode* node = hashTable[h];
        while (node) {
            if (node->key == key) {
                node->value += value;
                return;
            }
            node = node->next;
        }
        node = (HashNode*)malloc(sizeof(HashNode));
        node->key = key;
        node->value = value;
        node->next = hashTable[h];
        hashTable[h] = node;
    }

    int hash_get(int key) {
        unsigned int h = hash(key);
        HashNode* node = hashTable[h];
        while (node) {
            if (node->key == key) return node->value;
            node = node->next;
        }
        return 0;
    }

    void hash_clear() {
        for (int i = 0; i < HASH_SIZE; ++i) {
            HashNode* node = hashTable[i];
            while (node) {
                HashNode* temp = node;
                node = node->next;
                free(temp);
            }
            hashTable[i] = NULL;
        }
    }

    int gcd(int a, int b) {
        while (b) {
            int temp = b;
            b = a % b;
            a = temp;
        }
        return a;
    }

    hash_clear();

    long long ans = 0;

    for (int i = 4; i < numsSize - 2; i++) {
        int c = nums[i];
        for (int j = i + 2; j < numsSize; j++) {
            int d = nums[j];
            int g = gcd(c, d);
            int key = ((d / g) << 16) | (c / g);
            hash_insert(key, 1);
        }
    }

    for (int i = 2; i < numsSize - 4; i++) {
        int b = nums[i];
        for (int j = 0; j < i - 1; j++) {
            int a = nums[j];
            int g = gcd(a, b);
            int key = ((a / g) << 16) | (b / g);
            ans += hash_get(key);
        }

        int c = nums[i + 2];
        for (int j = i + 4; j < numsSize; j++) {
            int d = nums[j];
            int g = gcd(c, d);
            int key = ((d / g) << 16) | (c / g);
            hash_insert(key, -1);
        }
    }

    hash_clear();

    return ans;
}

int main() {
    int n;
    scanf("%d", &n);
    int* nums = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        scanf("%d", &nums[i]);
    }

    long long result = numberOfSubsequences(nums, n);
    printf("%lld\n", result);

    free(nums);
    return 0;
}
