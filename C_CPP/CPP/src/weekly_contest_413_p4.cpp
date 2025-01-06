// Problem: Weekly Contest 413 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cstring>
using namespace std;

class Solution {
public:
    vector<int> maximumSubarrayXor(vector<int>& f, vector<vector<int>>& queries) {
        int n = f.size();
        vector<vector<int>> mx(n, vector<int>(n));
        for (int i = n - 1; i >= 0; i--) {
            mx[i][i] = f[i];
            for (int j = i + 1; j < n; j++) {
                f[j] ^= f[j - 1];
                mx[i][j] = max({f[j], mx[i + 1][j], mx[i][j - 1]});
            }
        }

        vector<int> ans;
        for (auto& q : queries) {
            ans.push_back(mx[q[0]][q[1]]);
        }
        return ans;
    }
};

int main() {
    int numsSize, queriesSize;
    cin >> numsSize;
    vector<int> nums(numsSize);
    for (int i = 0; i < numsSize; i++) {
        cin >> nums[i];
    }
    cin >> queriesSize;
    vector<vector<int>> queries(queriesSize, vector<int>(2));
    for (int i = 0; i < queriesSize; i++) {
        cin >> queries[i][0] >> queries[i][1];
    }
    Solution solution;
    vector<int> result = solution.maximumSubarrayXor(nums, queries);
    for (int i = 0; i < result.size(); i++) {
        cout << result[i] << " ";
    }
    return 0;
}
