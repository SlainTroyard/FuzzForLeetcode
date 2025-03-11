// Problem: Weekly Contest 413 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

// Function to check if the two squares are of the same color
bool checkTwoChessboards(char* coordinate1, char* coordinate2) {
    return (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0;
}

int main() {
    // Read the coordinates of the two squares
    char coordinate1[3], coordinate2[3];
    scanf("%s %s", coordinate1, coordinate2);

    // Check if the two squares are of the same color
    if (checkTwoChessboards(coordinate1, coordinate2)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    return 0;
}
