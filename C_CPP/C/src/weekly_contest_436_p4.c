// Problem: Weekly Contest 436 Problem 4
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <limits.h>

int max(int a, int b) {
    return a > b ? a : b;
}

int min_element(int* arr, int size) {
    int min_val = arr[0];
    for (int i = 1; i < size; i++) {
        if (arr[i] < min_val) {
            min_val = arr[i];
        }
    }
    return min_val;
}

bool check(int* points, int pointsSize, int m, long long low) {
    int n = pointsSize, rem = m, pre = 0;
    for (int i = 0; i < n; i++) {
        int k = (int)((low - 1) / points[i] + 1 - pre);
        if (i == n - 1 && k <= 0) {
            break;
        }
        k = max(k, 1);
        rem -= k * 2 - 1;
        if (rem < 0) {
            return false;
        }
        pre = k - 1;
    }
    return true;
}

long long maxScore(int* points, int pointsSize, int m) {
    long long left = 0;
    long long right = (long long)(m + 1) / 2 * min_element(points, pointsSize) + 1;
    
    while (left + 1 < right) {
        long long mid = left + (right - left) / 2;
        if (check(points, pointsSize, m, mid)) {
            left = mid;
        } else {
            right = mid;
        }
    }
    
    return left;
}

int main() {
    int n, m;
    if (scanf("%d %d", &n, &m) != 2) {
        fprintf(stderr, "Error reading input for n and m\n");
        return 1;
    }
    
    int* points = (int*)malloc(n * sizeof(int));
    if (!points) {
        fprintf(stderr, "Memory allocation failed\n");
        return 1;
    }
    
    for (int i = 0; i < n; i++) {
        if (scanf("%d", &points[i]) != 1) {
            fprintf(stderr, "Error reading input for points[%d]\n", i);
            free(points);
            return 1;
        }
    }
    
    long long result = maxScore(points, n, m);
    
    printf("%lld\n", result);
    
    free(points);
    
    return 0;
}
