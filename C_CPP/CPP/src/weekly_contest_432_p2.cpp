// Problem: Weekly Contest 432 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int maximumAmount(vector<vector<int>>& coins) {
        int n = coins.size(), m = coins[0].size();

        const long long INF = 1e18;
        long long f[n][m][3];
        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++)
            for (int k = 0; k < 3; k++) f[i][j][k] = -INF;
        f[0][0][0] = coins[0][0]; f[0][0][1] = 0;

        for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) {
            for (int k = 0; k < 3; k++) {
                if (i > 0) f[i][j][k] = max(f[i][j][k], f[i - 1][j][k] + coins[i][j]);
                if (j > 0) f[i][j][k] = max(f[i][j][k], f[i][j - 1][k] + coins[i][j]);
            }
            for (int k = 1; k < 3; k++) {
                if (i > 0) f[i][j][k] = max(f[i][j][k], f[i - 1][j][k - 1]);
                if (j > 0) f[i][j][k] = max(f[i][j][k], f[i][j - 1][k - 1]);
            }
        }

        long long ans = -INF;
        for (int k = 0; k < 3; k++) ans = max(ans, f[n - 1][m - 1][k]);
        return ans;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n, m;
    cin >> n >> m;
    vector<vector<int>> coins(n, vector<int>(m));
    for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) cin >> coins[i][j];
    Solution solution;
    cout << solution.maximumAmount(coins) << endl;
    return 0;
}
