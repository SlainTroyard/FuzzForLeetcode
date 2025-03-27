// Problem: Weekly Contest 437 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

int min(int a, int b) {
    return a < b ? a : b;
}

int max(int a, int b) {
    return a > b ? a : b;
}

typedef struct {
    int right;  
    int left;   
} Interval;

int compare_intervals(const void *a, const void *b) {
    return ((Interval *)a)->right - ((Interval *)b)->right;
}

int lower_bound(int *arr, int size, int val) {
    int low = 0, high = size;
    while (low < high) {
        int mid = low + (high - low) / 2;
        if (arr[mid] < val)
            low = mid + 1;
        else
            high = mid;
    }
    return low;
}

int upper_bound(int *arr, int size, int val) {
    int low = 0, high = size;
    while (low < high) {
        int mid = low + (high - low) / 2;
        if (arr[mid] <= val)
            low = mid + 1;
        else
            high = mid;
    }
    return low;
}

bool maxSubstringLength(char* s, int k) {
    int n = strlen(s);
    
    int* pos[26];
    int pos_sizes[26] = {0};
    
    for (int i = 0; i < 26; i++) {
        pos[i] = (int*)malloc(n * sizeof(int));
        if (!pos[i]) {
            for (int j = 0; j < i; j++) {
                free(pos[j]);
            }
            return false; 
        }
    }
    
    for (int i = 0; i < n; i++) {
        int c = s[i] - 'a';
        pos[c][pos_sizes[c]++] = i;
    }
    
    Interval* intervals = (Interval*)malloc(26 * sizeof(Interval));
    if (!intervals) {
        for (int i = 0; i < 26; i++) {
            free(pos[i]);
        }
        return false; 
    }
    
    int interval_count = 0;
    
    for (int i = 0; i < 26; i++) {
        if (pos_sizes[i] > 0) {
            int l = pos[i][0], r = pos[i][pos_sizes[i] - 1];
            bool flag = true;
            
            while (flag) {
                flag = false;
                for (int j = 0; j < 26; j++) {
                    if (pos_sizes[j] > 0) {
                        int low_idx = lower_bound(pos[j], pos_sizes[j], l);
                        int up_idx = upper_bound(pos[j], pos_sizes[j], r);
                        int cnt = up_idx - low_idx;
                        
                        if (cnt > 0 && cnt < pos_sizes[j]) {
                            l = min(l, pos[j][0]);
                            r = max(r, pos[j][pos_sizes[j] - 1]);
                            flag = true;
                        }
                    }
                }
            }
            
            if (l > 0 || r < n - 1) {
                intervals[interval_count].right = r;
                intervals[interval_count].left = l;
                interval_count++;
            }
        }
    }
    
    qsort(intervals, interval_count, sizeof(Interval), compare_intervals);
    
    int R = -1, cnt = 0;
    for (int i = 0; i < interval_count; i++) {
        if (intervals[i].left > R) {
            R = intervals[i].right;
            cnt++;
        }
    }
    
    for (int i = 0; i < 26; i++) {
        free(pos[i]);
    }
    free(intervals);
    
    return cnt >= k;
}

int main() {
    char s[100001]; 
    int k;
    
    if (scanf("%s %d", s, &k) != 2) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    bool result = maxSubstringLength(s, k);
    
    printf("%s\n", result ? "true" : "false");
    
    return 0;
}
