// Problem: Weekly Contest 431 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long maximumCoins(vector<vector<int>>& nums, int k) {
        auto cmp = [&](vector<int>& a, vector<int>& b) -> bool {
            return a[0] < b[0];
        };
        sort(nums.begin(), nums.end(), cmp);
        long long ans = 0;
        vector<long long> presum(nums.size() + 1, 0);
        for (int i = 1; i < nums.size() + 1; i++) {
            presum[i] = presum[i - 1] + (1LL * (nums[i - 1][1] - nums[i - 1][0] + 1) * nums[i - 1][2]);
        }
        int left = 0; int right = 0;
        while (right < nums.size() && left < nums.size()) {
            while (left < nums.size() && nums[right][1] - nums[left][0] + 1 > k) {
                long long tamp = k - (nums[right][0] - nums[left][0]);
                ans = max(ans, tamp * nums[right][2] + presum[right] - presum[left]);
                left += 1;
            }
            if (left > nums.size()) break;
            ans = max(ans, presum[right + 1] - presum[left]);
            right += 1;
        }
        left = nums.size() - 1; right = nums.size() - 1;
        while (right >= 0 && left >= 0) {
            while (right >= 0 && nums[right][1] - nums[left][0] + 1 > k) {
                long long tamp = k - (nums[right][1] - nums[left][1]);
                ans = max(ans, tamp * nums[left][2] + presum[right + 1] - presum[left + 1]);
                right -= 1;
            }
            if (right < 0) break;
            ans = max(ans, presum[right + 1] - presum[left]);
            left -= 1;
        }
        return ans;
    }
};
    

int main() {
    
    int n, K;
    cin >> n >> K;
    vector<vector<int>> vec(n, vector<int>(3));
    for (int i = 0; i < n; i++) {
        cin >> vec[i][0] >> vec[i][1] >> vec[i][2];
    }
    Solution sol;
    cout << sol.maximumCoins(vec, K) << endl;
    return 0;
}
