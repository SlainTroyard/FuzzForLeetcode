// Problem: Weekly Contest 414 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>
#include <math.h>

int maxPossibleScore(int* start, int startSize, int d) {
    // Sort array
    for(int i = 0; i < startSize - 1; i++) {
        for(int j = 0; j < startSize - i - 1; j++) {
            if(start[j] > start[j+1]) {
                int temp = start[j];
                start[j] = start[j+1];
                start[j+1] = temp;
            }
        }
    }
    
    // Binary search
    int left = 0;
    int right = (start[startSize - 1] + d - start[0])/(startSize - 1) + 1;
    
    while(left + 1 < right) {
        int mid = left + (right - left) / 2;
        
        // Check function
        long long sum = LLONG_MIN;
        bool valid = true;
        for(int i = 0; i < startSize; i++) {
            sum = fmax(sum + mid, (long long)start[i]);
            if(sum > start[i] + d) {
                valid = false;
                break;
            }
        }
        
        if(valid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
}

int main() {
    int startSize, d;
    scanf("%d %d", &startSize, &d);
    int* start = (int*)malloc(sizeof(int) * startSize);
    for (int i = 0; i < startSize; i++) {
        scanf("%d", &start[i]);
    }
    printf("%d\n", maxPossibleScore(start, startSize, d));
    free(start);
    return 0;
}
