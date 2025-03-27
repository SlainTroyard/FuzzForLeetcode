// Problem: Weekly Contest 431 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int compare(const void* a, const void* b) {
    int* coin1 = *(int**)a;
    int* coin2 = *(int**)b;
    return coin1[0] - coin2[0]; 
}

long long max_ll(long long a, long long b) {
    return (a > b) ? a : b;
}

long long maximumCoins(int** coins, int coinsSize, int* coinsColSize, int k) {
    qsort(coins, coinsSize, sizeof(int*), compare);
    
    long long* presum = (long long*)malloc((coinsSize + 1) * sizeof(long long));
    presum[0] = 0;
    for (int i = 1; i <= coinsSize; i++) {
        presum[i] = presum[i - 1] + (long long)(coins[i - 1][1] - coins[i - 1][0] + 1) * coins[i - 1][2];
    }
    
    long long ans = 0;
    int left = 0, right = 0;
    
    while (right < coinsSize && left < coinsSize) {
        while (left < coinsSize && coins[right][1] - coins[left][0] + 1 > k) {
            long long tamp = k - (coins[right][0] - coins[left][0]);
            ans = max_ll(ans, tamp * coins[right][2] + presum[right] - presum[left]);
            left += 1;
        }
        if (left >= coinsSize) break;
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        right += 1;
    }
    
    left = coinsSize - 1;
    right = coinsSize - 1;
    while (right >= 0 && left >= 0) {
        while (right >= 0 && coins[right][1] - coins[left][0] + 1 > k) {
            long long tamp = k - (coins[right][1] - coins[left][1]);
            ans = max_ll(ans, tamp * coins[left][2] + presum[right + 1] - presum[left + 1]);
            right -= 1;
        }
        if (right < 0) break;
        ans = max_ll(ans, presum[right + 1] - presum[left]);
        left -= 1;
    }
    
    free(presum);
    return ans;
}

int main() {
    int n, k;
    scanf("%d %d", &n, &k);
    
    int** coins = (int**)malloc(n * sizeof(int*));
    int* coinsColSize = (int*)malloc(n * sizeof(int));
    
    for (int i = 0; i < n; i++) {
        coins[i] = (int*)malloc(3 * sizeof(int));
        coinsColSize[i] = 3;
        scanf("%d %d %d", &coins[i][0], &coins[i][1], &coins[i][2]);
    }
    
    printf("%lld\n", maximumCoins(coins, n, coinsColSize, k));
    
    for (int i = 0; i < n; i++) {
        free(coins[i]);
    }
    free(coins);
    free(coinsColSize);
    
    return 0;
}
