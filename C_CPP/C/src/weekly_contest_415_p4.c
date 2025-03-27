// Problem: Weekly Contest 415 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <time.h>

#define MOD 1070777777

typedef struct {
    int* data;      
    int size;       
    int capacity;   
} HashSet;

void initHashSet(HashSet* set) {
    set->data = NULL;
    set->size = 0;
    set->capacity = 0;
}

void addHash(HashSet* set, int hash) {
    if (set->size == set->capacity) {
        set->capacity = set->capacity ? set->capacity * 2 : 16;
        set->data = realloc(set->data, set->capacity * sizeof(int));
        if (!set->data) {
            fprintf(stderr, "Memory allocation failed.\n");
            exit(EXIT_FAILURE);
        }
    }
    set->data[set->size++] = hash;
}

int cmp_int(const void* a, const void* b) {
    int x = *(const int*)a;
    int y = *(const int*)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

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

unsigned int getRandomBase() {
    return 800000000 + (rand() % 100000000);
}

int minValidStrings(char** words, int wordsSize, char* target) {
    srand((unsigned int)time(NULL));
    
    int n = 0;
    while(target[n] != '\0') n++;
    
    int max_len = 0;
    for(int i = 0; i < wordsSize; i++) {
        int len = 0;
        while(words[i][len] != '\0') len++;
        if(len > max_len) max_len = len;
    }
    
    unsigned int base = getRandomBase();
    
    unsigned long long* pow_base = malloc((n + 1) * sizeof(unsigned long long));
    if (!pow_base) {
        fprintf(stderr, "Memory allocation failed for pow_base.\n");
        exit(EXIT_FAILURE);
    }
    pow_base[0] = 1;
    for(int i = 0; i < n; i++) {
        pow_base[i + 1] = (pow_base[i] * (unsigned long long)base) % MOD;
    }
    
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
    
    for(int i = 0; i < wordsSize; i++) {
        int len = 0;
        while(words[i][len] != '\0' && len < max_len) len++;
        unsigned long long h = 0;
        for(int j = 0; j < len; j++) {
            h = (h * (unsigned long long)base + (unsigned char)words[i][j]) % MOD;
            addHash(&sets[j], (int)h);
        }
    }
    
    for(int j = 0; j < max_len; j++) {
        sortAndUniqueHashSet(&sets[j]);
    }
    
    int ans = 0;
    int cur_r = 0;
    int nxt_r = 0;
    
    for(int i = 0; i < n; i++) {
        while(nxt_r < n && (nxt_r - i) < max_len) {
            if (nxt_r + 1 > n) break; 
            unsigned long long current_hash = (pre_hash[nxt_r + 1] + MOD - (pre_hash[i] * pow_base[nxt_r + 1 - i]) % MOD) % MOD;
            int sh = (int)current_hash;
            
            int prefix_len = nxt_r - i;
            if(prefix_len < 0 || prefix_len >= max_len) break; 
            
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
        
        if(i == cur_r) {
            if(i == nxt_r) {
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
    
    for(int j = 0; j < max_len; j++) {
        free(sets[j].data);
    }
    free(sets);
    free(pow_base);
    free(pre_hash);
    
    return ans;
}

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
