// Problem: Weekly Contest 437 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

// 返回两个数中的较小值
int min(int a, int b) {
    return a < b ? a : b;
}

// 返回两个数中的较大值
int max(int a, int b) {
    return a > b ? a : b;
}

// 表示区间的结构体 (相当于C++中的pair<int, int>)
typedef struct {
    int right;  // 右端点 (用于排序)
    int left;   // 左端点
} Interval;

// 用于qsort的比较函数 - 按照右端点排序
int compare_intervals(const void *a, const void *b) {
    return ((Interval *)a)->right - ((Interval *)b)->right;
}

// 二分查找lower_bound (找到第一个大于等于val的元素位置)
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

// 二分查找upper_bound (找到第一个大于val的元素位置)
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

// 主函数实现
bool maxSubstringLength(char* s, int k) {
    int n = strlen(s);
    
    // 为每个字符存储位置
    int* pos[26];
    int pos_sizes[26] = {0};
    
    // 为每个字符分配足够大的数组 (最大n个位置)
    for (int i = 0; i < 26; i++) {
        pos[i] = (int*)malloc(n * sizeof(int));
        if (!pos[i]) {
            // 内存分配失败，释放已分配的内存
            for (int j = 0; j < i; j++) {
                free(pos[j]);
            }
            return false; // 错误处理
        }
    }
    
    // 将每个字符的位置存储到对应数组
    for (int i = 0; i < n; i++) {
        int c = s[i] - 'a';
        pos[c][pos_sizes[c]++] = i;
    }
    
    // 存储候选区间
    Interval* intervals = (Interval*)malloc(26 * sizeof(Interval));
    if (!intervals) {
        // 内存分配失败，释放已分配的内存
        for (int i = 0; i < 26; i++) {
            free(pos[i]);
        }
        return false; // 错误处理
    }
    
    int interval_count = 0;
    
    // 查找满足条件的区间
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
    
    // 按照右端点排序区间
    qsort(intervals, interval_count, sizeof(Interval), compare_intervals);
    
    // 贪心选择区间
    int R = -1, cnt = 0;
    for (int i = 0; i < interval_count; i++) {
        if (intervals[i].left > R) {
            R = intervals[i].right;
            cnt++;
        }
    }
    
    // 释放内存
    for (int i = 0; i < 26; i++) {
        free(pos[i]);
    }
    free(intervals);
    
    return cnt >= k;
}

int main() {
    // 读取输入
    char s[100001]; // 假设字符串最大长度为100000
    int k;
    
    if (scanf("%s %d", s, &k) != 2) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    // 调用函数计算结果
    bool result = maxSubstringLength(s, k);
    
    // 输出结果
    printf("%s\n", result ? "true" : "false");
    
    return 0;
}
