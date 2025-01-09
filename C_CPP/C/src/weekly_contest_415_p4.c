// Problem: Weekly Contest 415 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

// Define the modulus as per the C++ code
#define MOD 1070777777

// Structure to hold a dynamic array of hashes for each prefix length
typedef struct {
    int* data;      // Array of hash values
    int size;       // Current number of elements
    int capacity;   // Current capacity of the array
} HashSet;

// Initialize a HashSet
void initHashSet(HashSet* set) {
    set->data = NULL;
    set->size = 0;
    set->capacity = 0;
}

// Add a hash value to the HashSet
void addHash(HashSet* set, int hash) {
    if (set->size == set->capacity) {
        set->capacity = set->capacity ? set->capacity * 2 : 16;
        set->data = realloc(set->data, set->capacity * sizeof(int));
        if (!set->data) {
            // Handle memory allocation failure
            fprintf(stderr, "Memory allocation failed.\n");
            exit(EXIT_FAILURE);
        }
    }
    set->data[set->size++] = hash;
}

// Comparator function for qsort
int cmp_int(const void* a, const void* b) {
    int x = *(const int*)a;
    int y = *(const int*)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

// Sort the HashSet and remove duplicates
void sortAndUniqueHashSet(HashSet* set) {
    if (set->size == 0) return;
    qsort(set->data, set->size, sizeof(int), cmp_int);
    int unique_size = 1;
    for(int i = 1; i < set->size; i++) {
        if(set->data[i] != set->data[unique_size - 1]) {
            set->data[unique_size++] = set->data[i];
        }
    }
    set->size = unique_size;
}

// Function to generate a random base between 8e8 and 9e8
unsigned int getRandomBase() {
    // Combine multiple rand() calls to get a larger number
    // Note: rand() typically returns a value between 0 and 32767
    return 800000000 + (rand() % 100000000);
}

// The main function to compute the minimum number of valid strings
int minValidStrings(char** words, int wordsSize, char* target) {
    // Seed the random number generator
    srand((unsigned int)time(NULL));
    
    // Step 1: Determine the length of the target string
    int n = 0;
    while(target[n] != '\0') n++;
    
    // Step 2: Find the maximum length among all words
    int max_len = 0;
    for(int i = 0; i < wordsSize; i++) {
        int len = 0;
        while(words[i][len] != '\0') len++;
        if(len > max_len) max_len = len;
    }
    
    // Step 3: Initialize the base for hashing
    unsigned int base = getRandomBase();
    
    // Step 4: Precompute powers of the base modulo MOD
    unsigned long long* pow_base = malloc((n + 1) * sizeof(unsigned long long));
    if (!pow_base) {
        fprintf(stderr, "Memory allocation failed for pow_base.\n");
        exit(EXIT_FAILURE);
    }
    pow_base[0] = 1;
    for(int i = 0; i < n; i++) {
        pow_base[i + 1] = (pow_base[i] * (unsigned long long)base) % MOD;
    }
    
    // Step 5: Compute prefix hashes for the target string
    unsigned long long* pre_hash = malloc((n + 1) * sizeof(unsigned long long));
    if (!pre_hash) {
        fprintf(stderr, "Memory allocation failed for pre_hash.\n");
        free(pow_base);
        exit(EXIT_FAILURE);
    }
    pre_hash[0] = 0;
    for(int i = 0; i < n; i++) {
        pre_hash[i + 1] = (pre_hash[i] * (unsigned long long)base + (unsigned char)target[i]) % MOD;
    }
    
    // Step 6: Initialize HashSets for each possible prefix length
    HashSet* sets = malloc(max_len * sizeof(HashSet));
    if (!sets) {
        fprintf(stderr, "Memory allocation failed for sets.\n");
        free(pow_base);
        free(pre_hash);
        exit(EXIT_FAILURE);
    }
    for(int j = 0; j < max_len; j++) {
        initHashSet(&sets[j]);
    }
    
    // Step 7: Populate the HashSets with prefix hashes from the words
    for(int i = 0; i < wordsSize; i++) {
        int len = 0;
        while(words[i][len] != '\0' && len < max_len) len++;
        unsigned long long h = 0;
        for(int j = 0; j < len; j++) {
            h = (h * (unsigned long long)base + (unsigned char)words[i][j]) % MOD;
            addHash(&sets[j], (int)h);
        }
    }
    
    // Step 8: Sort and remove duplicates in each HashSet
    for(int j = 0; j < max_len; j++) {
        sortAndUniqueHashSet(&sets[j]);
    }
    
    // Step 9: Implement the two-pointer approach to find the minimum number of strings
    int ans = 0;
    int cur_r = 0;
    int nxt_r = 0;
    
    for(int i = 0; i < n; i++) {
        // Attempt to extend nxt_r as far as possible from the current position i
        while(nxt_r < n && (nxt_r - i) < max_len) {
            // Compute the hash of the substring target[i..nxt_r+1)
            if (nxt_r + 1 > n) break; // Safety check
            unsigned long long current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            int sh = (int)current_hash;
            
            // Determine the prefix length (nxt_r - i)
            int prefix_len = nxt_r - i;
            if(prefix_len < 0 || prefix_len >= max_len) break; // Safety check
            
            // Binary search in the sorted HashSet for this prefix length
            HashSet* set = &sets[prefix_len];
            int left = 0, right = set->size - 1;
            int found = 0;
            while(left <= right) {
                int mid = left + (right - left) / 2;
                if(set->data[mid] == sh) {
                    found = 1;
                    break;
                }
                else if(set->data[mid] < sh) {
                    left = mid + 1;
                }
                else {
                    right = mid - 1;
                }
            }
            
            if(found) {
                nxt_r++;
            }
            else {
                break;
            }
        }
        
        // If we've reached the end of the current bridge, update the bridge
        if(i == cur_r) {
            if(i == nxt_r) {
                // Free all allocated memory before returning
                for(int j = 0; j < max_len; j++) {
                    free(sets[j].data);
                }
                free(sets);
                free(pow_base);
                free(pre_hash);
                return -1;
            }
            cur_r = nxt_r;
            ans++;
        }
    }
    
    // Free all allocated memory
    for(int j = 0; j < max_len; j++) {
        free(sets[j].data);
    }
    free(sets);
    free(pow_base);
    free(pre_hash);
    
    return ans;
}

// 1 <= words.length <= 100, 1 <= words[i].length <= 5 * 10^3. The input is generated such that sum(words[i].length) <= 10^5. words[i] consists only of lowercase English letters. 1 <= target.length <= 5 * 10^3. target consists only of lowercase English letters.
int main() {
    int wordsSize;
    scanf("%d", &wordsSize);
    char** words = (char**)malloc(sizeof(char*) * wordsSize);
    int wordsColSize;
    for (int i = 0; i < wordsSize; i++) {
        scanf("%d", &wordsColSize);
        words[i] = (char*)malloc(sizeof(char) * (wordsColSize + 1));
        scanf("%s", words[i]);
    }
    int targetSize;
    scanf("%d", &targetSize);
    char* target = (char*)malloc(sizeof(char) * (targetSize + 1));
    scanf("%s", target);
    int res = minValidStrings(words, wordsSize, target);
    printf("%d\n", res);
    for (int i = 0; i < wordsSize; i++) {
        free(words[i]);
    } 
    free(words);
    free(target);
    return 0;
}
