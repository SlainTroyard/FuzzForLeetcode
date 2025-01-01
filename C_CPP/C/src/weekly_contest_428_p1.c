// Problem: Weekly Contest 428 Problem 1
#include <stdio.h>
#include <stdlib.h>

// Function to calculate the button with the longest press time
int buttonWithLongestTime(int** events, int eventsSize, int* eventsColSize) {
    int longest_time = events[0][1];
    int prev = events[0][1];
    int longest_index = events[0][0];

    for (int i = 1; i != eventsSize; ++i) {
        const int current_press = events[i][1];
        const int current_time = current_press - prev;

        if (current_time > longest_time ||
            (current_time == longest_time && events[i][0] < longest_index)) {
            longest_time = current_time;
            longest_index = events[i][0];
        }
        prev = current_press;
    }
    return longest_index;
}

int main() {
    int n;
    scanf("%d", &n);

    int** events = (int**)malloc(n * sizeof(int*));
    for (int i = 0; i < n; i++) {
        events[i] = (int*)malloc(2 * sizeof(int));
        scanf("%d %d", &events[i][0], &events[i][1]);
    }

    int* eventsColSize = (int*)malloc(n * sizeof(int));
    for (int i = 0; i < n; i++) {
        eventsColSize[i] = 2;
    }

    int result = buttonWithLongestTime(events, n, eventsColSize);
    printf("%d\n", result);

    for (int i = 0; i < n; i++) {
        free(events[i]);
    }
    free(events);
    free(eventsColSize);

    return 0;
}
