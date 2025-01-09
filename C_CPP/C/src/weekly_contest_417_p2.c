// Problem: Weekly Contest 417 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int countOfSubstrings(char* word, int k) {
    int count = 0;
    int len = strlen(word);

    // Loop through all possible starting points of substrings
    for(int i = 0; i <= len - 5; i++) {
        int arr[5] = {0}; // Reset vowel counts for each starting point
        int nonVowelCount = 0;

        // Loop through the substrings starting at i
        for(int j = i; j < len; j++) {
            // Check for vowels and increment respective count
            if (word[j] == 'a') arr[0]++;
            else if (word[j] == 'e') arr[1]++;
            else if (word[j] == 'i') arr[2]++;
            else if (word[j] == 'o') arr[3]++;
            else if (word[j] == 'u') arr[4]++;
            else nonVowelCount++; // Increment non-vowel count for other characters

            // Check if all vowels are present and non-vowel count equals k
            if (arr[0] > 0 && arr[1] > 0 && arr[2] > 0 && arr[3] > 0 && arr[4] > 0 && nonVowelCount == k) {
                count++;
            }
        }
    }

    return count;
}

int main() {
    int wordSize, k;
    scanf("%d", &wordSize);
    char *word = (char *)malloc(wordSize + 1);
    scanf("%s", word);
    scanf("%d", &k);
    printf("%d\n", countOfSubstrings(word, k));
    free(word);
    return 0;
}
