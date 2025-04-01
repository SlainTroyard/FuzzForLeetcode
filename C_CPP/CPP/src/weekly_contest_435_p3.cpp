// Problem: Weekly Contest 435 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
#include <cmath>
#include <climits>
#include <functional>
using namespace std;

class Solution {
public:
    int minimumIncrements(vector<int>& nums, vector<int>& target) {
        int n = nums.size(), m = target.size();

        long long g[1 << m];
        for (int i = 0; i < (1 << m); i++) {
            g[i] = 1;
            for (int j = 0; j < m; j++) if (i >> j & 1)
                g[i] = g[i] / gcd(g[i], target[j]) * target[j];
        }

        const long long INF = 1e18;
        long long f[2][1 << m];
        for (int j = 0; j < (1 << m); j++) f[0][j] = INF;
        f[0][0] = 0;

        for (int i = 1; i <= n; i++) {
            for (int j = 0; j < (1 << m); j++) f[i & 1][j] = f[i & 1 ^ 1][j];
            for (int j = 0; j < (1 << m); j++) for (int k = j; k > 0; k = (k - 1) & j) {
                long long v = (nums[i - 1] + g[k] - 1) / g[k] * g[k] - nums[i - 1];
                f[i & 1][j] = min(f[i & 1][j], f[i & 1 ^ 1][j ^ k] + v);
            }
        }

        return f[n & 1][(1 << m) - 1];
    }
};

int main() {
    
    int n, m;
    cin >> n >> m;
    vector<int> nums(n), target(m);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    for (int i = 0; i < m; i++) {
        cin >> target[i];
    }
    Solution solution;
    cout << solution.minimumIncrements(nums, target) << endl;
    return 0;
}
