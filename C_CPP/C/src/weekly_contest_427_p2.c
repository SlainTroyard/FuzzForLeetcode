// Problem: Weekly Contest 427 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MAX_POINTS 100

int get(int points[][2], int size) {
    int maxArea = -1;
    for (int i = 0; i < size - 3; i++) {
        if (points[i][0] == points[i + 1][0] &&
            points[i + 2][0] == points[i + 3][0] &&
            points[i][1] == points[i + 2][1] &&
            points[i + 1][1] == points[i + 3][1]) {
            int area = (points[i + 2][0] - points[i][0]) * 
                       (points[i + 1][1] - points[i][1]);
            if (area > maxArea) {
                maxArea = area;
            }
        }
    }
    return maxArea;
}

int comparePoints(const void *a, const void *b) {
    int *pointA = (int *)a;
    int *pointB = (int *)b;

    if (pointA[0] != pointB[0]) {
        return pointA[0] - pointB[0];
    }
    return pointA[1] - pointB[1];
}

int maxRectangleArea(int points[][2], int n) {
    int maxArea = -1;

    qsort(points, n, sizeof(points[0]), comparePoints);

    for (int i = 0; i < n - 3; i++) {
        int rectanglePoints[4][2];

        rectanglePoints[0][0] = points[i][0];
        rectanglePoints[0][1] = points[i][1];
        rectanglePoints[1][0] = points[i + 1][0];
        rectanglePoints[1][1] = points[i + 1][1];

        int j = i + 2;
        while (j < n - 2) {
            if (points[j][1] > points[i + 1][1] || points[j][1] < points[i][1]) {
                j++;
            } else {
                break;
            }
        }

        if (j < n - 1) {
            rectanglePoints[2][0] = points[j][0];
            rectanglePoints[2][1] = points[j][1];
            rectanglePoints[3][0] = points[j + 1][0];
            rectanglePoints[3][1] = points[j + 1][1];

            int area = get(rectanglePoints, 4);
            if (area > maxArea) {
                maxArea = area;
            }
        }
    }

    return maxArea;
}

int main() {
    int n;
    scanf("%d", &n);

    if (n < 4) {
        printf("-1\n");
        return 0;
    }

    int points[MAX_POINTS][2];
    for (int i = 0; i < n; i++) {
        scanf("%d %d", &points[i][0], &points[i][1]);
    }

    int result = maxRectangleArea(points, n);
    printf("%d\n", result);

    return 0;
}
