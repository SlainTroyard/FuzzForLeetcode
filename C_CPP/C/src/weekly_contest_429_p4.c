// Problem: Weekly Contest 429 Problem 4
#include <stdio.h>
#include <string.h>
#include <limits.h>
#include <stdbool.h>

#define MAX_LEN 100000

int merge(int intervals[][2], int size, int len, int merged[][2]) {
    if (size == 0) return 0;

    int mergedSize = 0;
    int currentStart = intervals[0][0];
    int currentEnd = intervals[0][1];

    for (int i = 1; i < size; i++) {
        int start = intervals[i][0];
        int end = intervals[i][1];

        if (start <= currentEnd && (start - currentStart + 1) <= len) {
            currentEnd = currentEnd > end ? currentEnd : end;
        } else {
            merged[mergedSize][0] = currentStart;
            merged[mergedSize][1] = currentEnd;
            mergedSize++;
            currentStart = start;
            currentEnd = end;
        }
    }
    merged[mergedSize][0] = currentStart;
    merged[mergedSize][1] = currentEnd;
    return mergedSize + 1;
}

bool isPoss(char *s, int n, int op, int mid) {
    int i = 0, j = 0;
    int zero = 0, one = 0;
    int intervals[MAX_LEN][2];
    int size = 0;

    while (j < n) {
        if (s[j] == '0') zero++;
        else one++;

        while ((j - i + 1) > mid) {
            if (s[i] == '0') zero--;
            else one--;
            i++;
        }

        if ((j - i + 1) == mid) {
            if (zero == 0 || one == 0) {
                intervals[size][0] = i;
                intervals[size][1] = j;
                size++;
            }
        }
        j++;
    }

    int merged[MAX_LEN][2];
    int mergedSize = merge(intervals, size, mid, merged);

    return mergedSize <= op;
}

int getMini(char *s, int n, char even, char odd) {
    int op = 0;
    for (int i = 0; i < n; i++) {
        if (i % 2 == 0 && s[i] != even) op++;
        else if (i % 2 == 1 && s[i] != odd) op++;
    }
    return op;
}

int minLength(char *s, int numOps) {
    int n = strlen(s);
    int start = 3, end = n;
    int res = 2;

    int op = INT_MAX;
    int opEvenOdd = getMini(s, n, '0', '1');
    int opOddEven = getMini(s, n, '1', '0');
    op = opEvenOdd < op ? opEvenOdd : op;
    op = opOddEven < op ? opOddEven : op;

    if (op <= numOps) return 1;

    while (start <= end) {
        int mid = start + (end - start) / 2;
        if (isPoss(s, n, numOps, mid)) {
            end = mid - 1;
        } else {
            res = mid;
            start = mid + 1;
        }
    }
    return res;
}

int main() {
    char s[MAX_LEN + 1];
    int numOps;

    scanf("%s", s);

    scanf("%d", &numOps);

    int result = minLength(s, numOps);

    printf("%d\n", result);

    return 0;
}
