// Problem: Weekly Contest 431 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <cstring>
using namespace std;

class Solution {
public:
    int maxLength(vector<int>& nums) {
        int n = nums.size(), m = 0;
        for (int x : nums) m = max(m, x);

        vector<int> fac[m + 1];
        for (int i = 2; i <= m; i++) if (fac[i].empty())
            for (int j = i; j <= m; j += i) fac[j].push_back(i);

        int ans = 2;
        bool vis[m + 1];
        memset(vis, 0, sizeof(vis));
        for (int i = 0, j = 0; i < n; i++) {
            while (j < n) {
                auto check = [&]() {
                    for (int p : fac[nums[j]]) if (vis[p]) return false;
                    return true;
                };
                if (check()) {
                    for (int p : fac[nums[j]]) vis[p] = true;
                    j++;
                } else {
                    break;
                }
            }
            ans = max(ans, j - i);
            for (int p : fac[nums[i]]) vis[p] = false;
        }
        return ans;
    }
};

int main() {
    int numSize;
    cin >> numSize;
    vector<int> nums(numSize);
    for (int i = 0; i < numSize; i++) {
        cin >> nums[i];
    }
    Solution solution;
    int result = solution.maxLength(nums);
    cout << result << endl;
    return 0;
}
