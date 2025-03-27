// Problem: Weekly Contest 413 Problem 1
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool checkTwoChessboards(char* coordinate1, char* coordinate2) {
    return (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0;
}

int main() {
    char coordinate1[3], coordinate2[3];
    scanf("%s %s", coordinate1, coordinate2);

    if (checkTwoChessboards(coordinate1, coordinate2)) {
        printf("true\n");
    } else {
        printf("false\n");
    }

    return 0;
}
