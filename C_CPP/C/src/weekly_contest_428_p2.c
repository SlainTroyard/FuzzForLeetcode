// Problem: Weekly Contest 428 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

double maxAmount(char* initialCurrency, char*** pairs1, int pairs1Size, int* pairs1ColSize, double* rates1, int rates1Size, char*** pairs2, int pairs2Size, int* pairs2ColSize, double* rates2, int rates2Size) {
    const int MAX_CURRENCIES = 20;
    double graph1[MAX_CURRENCIES][MAX_CURRENCIES];
    double graph2[MAX_CURRENCIES][MAX_CURRENCIES];
    char* currencies[MAX_CURRENCIES];
    int currencyCount = 0;

    int getCurrencyIndex(char* currency) {
        for (int i = 0; i < currencyCount; i++) {
            if (strcmp(currencies[i], currency) == 0) {
                return i;
            }
        }
        currencies[currencyCount] = currency;
        return currencyCount++;
    }

    for (int i = 0; i < MAX_CURRENCIES; i++) {
        for (int j = 0; j < MAX_CURRENCIES; j++) {
            graph1[i][j] = (i == j) ? 1.0 : 0.0;
            graph2[i][j] = (i == j) ? 1.0 : 0.0;
        }
    }

    for (int i = 0; i < pairs1Size; i++) {
        int from = getCurrencyIndex(pairs1[i][0]);
        int to = getCurrencyIndex(pairs1[i][1]);
        graph1[from][to] = rates1[i];
        graph1[to][from] = 1.0 / rates1[i];
    }

    for (int i = 0; i < pairs2Size; i++) {
        int from = getCurrencyIndex(pairs2[i][0]);
        int to = getCurrencyIndex(pairs2[i][1]);
        graph2[from][to] = rates2[i];
        graph2[to][from] = 1.0 / rates2[i];
    }

    for (int k = 0; k < currencyCount; k++) {
        for (int i = 0; i < currencyCount; i++) {
            for (int j = 0; j < currencyCount; j++) {
                graph1[i][j] = fmax(graph1[i][j], graph1[i][k] * graph1[k][j]);
                graph2[i][j] = fmax(graph2[i][j], graph2[i][k] * graph2[k][j]);
            }
        }
    }

    int startIndex = getCurrencyIndex(initialCurrency);

    double maxAmount = 1.0;
    for (int i = 0; i < currencyCount; i++) {
        double amountDay1 = graph1[startIndex][i];
        double amountDay2 = amountDay1 * graph2[i][startIndex];
        maxAmount = fmax(maxAmount, amountDay2);
    }

    return maxAmount;
}

int main() {
    char initialCurrency[4];
    scanf("%s", initialCurrency);

    int pairs1Size;
    scanf("%d", &pairs1Size);

    char*** pairs1 = (char***)malloc(pairs1Size * sizeof(char**));
    double* rates1 = (double*)malloc(pairs1Size * sizeof(double));
    int* pairs1ColSize = (int*)malloc(pairs1Size * sizeof(int));

    for (int i = 0; i < pairs1Size; i++) {
        pairs1[i] = (char**)malloc(2 * sizeof(char*));
        pairs1[i][0] = (char*)malloc(4 * sizeof(char));
        pairs1[i][1] = (char*)malloc(4 * sizeof(char));
        scanf("%s %s %lf", pairs1[i][0], pairs1[i][1], &rates1[i]);
        pairs1ColSize[i] = 2; 
    }

    int pairs2Size;
    scanf("%d", &pairs2Size);

    char*** pairs2 = (char***)malloc(pairs2Size * sizeof(char**));
    double* rates2 = (double*)malloc(pairs2Size * sizeof(double));
    int* pairs2ColSize = (int*)malloc(pairs2Size * sizeof(int));

    for (int i = 0; i < pairs2Size; i++) {
        pairs2[i] = (char**)malloc(2 * sizeof(char*));
        pairs2[i][0] = (char*)malloc(4 * sizeof(char));
        pairs2[i][1] = (char*)malloc(4 * sizeof(char));
        scanf("%s %s %lf", pairs2[i][0], pairs2[i][1], &rates2[i]);
        pairs2ColSize[i] = 2; 
    }

    double result = maxAmount(initialCurrency, pairs1, pairs1Size, pairs1ColSize, rates1, pairs1Size, pairs2, pairs2Size, pairs2ColSize, rates2, pairs2Size);

    printf("%.5lf\n", result);

    for (int i = 0; i < pairs1Size; i++) {
        free(pairs1[i][0]);
        free(pairs1[i][1]);
        free(pairs1[i]);
    }
    free(pairs1);
    free(rates1);
    free(pairs1ColSize);

    for (int i = 0; i < pairs2Size; i++) {
        free(pairs2[i][0]);
        free(pairs2[i][1]);
        free(pairs2[i]);
    }
    free(pairs2);
    free(rates2);
    free(pairs2ColSize);

    return 0;
}
