// Problem: Weekly Contest 433 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <numeric>
#include <algorithm>
using namespace std;

class Solution {
public:
    int subarraySum(vector<int>& nums) {
        int n = nums.size();
        vector<int> s(n + 1);
        partial_sum(nums.begin(), nums.end(), s.begin() + 1);

        int ans = 0;
        for (int i = 0; i < n; i++) {
            ans += s[i + 1] - s[max(i - nums[i], 0)];
        }
        return ans;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n;
    cin >> n;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution solution;
    cout << solution.subarraySum(nums) << endl;
    return 0;
}
