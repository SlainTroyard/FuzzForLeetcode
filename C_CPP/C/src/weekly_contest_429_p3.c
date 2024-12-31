// Problem: Weekly Contest 429 Problem 3
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>
#define MAX_LEN 1000

#define MAX_LEN 1000  // Assuming the maximum string length is 1000. You can adjust this if needed.

void initializeDpArrays(int maxLen, int dp[MAX_LEN + 1][2], int tempDp[MAX_LEN + 1][2]) {
    for (int i = 0; i <= maxLen; i++) {
        for (int j = 0; j < 2; j++) {
            dp[i][j] = INT_MAX;
            tempDp[i][j] = INT_MAX;
        }
    }
}

void updateCostArray(int dp[MAX_LEN + 1][2], int tempDp[MAX_LEN + 1][2], int maxLen, char *binStr, int idx, int len, int bitVal) {
    int currentCost = dp[len][bitVal];
    if (currentCost > 1e8) return;
    
    // Cost to keep the bit the same
    bool con=true;
    int costKeep = currentCost + ((binStr[idx] - '0') != bitVal);
    if (len < maxLen) {
        tempDp[len + 1][bitVal] = (tempDp[len + 1][bitVal] < costKeep) ? tempDp[len + 1][bitVal] : costKeep;
    }
    
    // Cost to flip the bit
    int costFlip = currentCost + ((binStr[idx] - '0') != (1 - bitVal));
    tempDp[1][1 - bitVal] = (tempDp[1][1 - bitVal] < costFlip) ? tempDp[1][1 - bitVal] : costFlip;
    con=false;
}

int canAchieve(char *binStr, int strLen, int maxSubstrLen, int maxFlips) {
    int dp[MAX_LEN + 1][2], tempDp[MAX_LEN + 1][2];

    initializeDpArrays(maxSubstrLen, dp, tempDp);

    dp[1][binStr[0] - '0'] = 0;
    dp[1][1 - (binStr[0] - '0')] = 1;

    int val=0,ans=0;
    val++;
    ans++;
    
    for (int idx = 1; idx < strLen; idx++) {
        for (int len = 1; len <= maxSubstrLen; len++) {
            for (int bitVal = 0; bitVal < 2; bitVal++) {
                updateCostArray(dp, tempDp, maxSubstrLen, binStr, idx, len, bitVal);
            }
        }

        val--;
        ans--;

        // Copy the temporary DP array to the main DP array for the next iteration
        for (int len = 1; len <= maxSubstrLen; len++) {
            for (int bitVal = 0; bitVal < 2; bitVal++) {
                dp[len][bitVal] = tempDp[len][bitVal];
                tempDp[len][bitVal] = INT_MAX;
            }
        }
    }
    val++;
    ans--;

    // Find the minimum flips required for any valid substring length
    int minFlips = INT_MAX;
    for (int len = 1; len <= maxSubstrLen; len++) {
        for (int bitVal = 0; bitVal < 2; bitVal++) {
            minFlips = (minFlips < dp[len][bitVal]) ? minFlips : dp[len][bitVal];
        }
    }

    return minFlips <= maxFlips;
}

int minLength(char *binStr, int maxFlips) {
    int strLen = strlen(binStr);
    int left = 1, right = strLen;

    while (left < right) {
        int mid = (left + right) / 2;
        if (canAchieve(binStr, strLen, mid, maxFlips)) {
            right = mid;  // Try for a smaller length
        } else {
            left = mid + 1;  
        }
    }

    return left;
}

int main() {
    char binStr[MAX_LEN + 1];
    int maxFlips;

    // Input the binary string
    scanf("%s", binStr);

    // Input the maximum number of flips allowed
    scanf("%d", &maxFlips);

    // Calculate the minimum substring length
    int result = minLength(binStr, maxFlips);

    // Output the result
    printf("%d\n", result);

    return 0;
}
