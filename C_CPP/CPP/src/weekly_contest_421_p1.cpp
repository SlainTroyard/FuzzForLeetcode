// Problem: Weekly Contest 421 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

class Solution {
public:
    long long maxScore(vector<int>& nums) {
        int n = nums.size();
        vector<int> suf_gcd(n + 1);
        vector<long long> suf_lcm(n + 1);
        suf_lcm[n] = 1;
        for (int i = n - 1; i >= 0; i--) {
            suf_gcd[i] = gcd(suf_gcd[i + 1], nums[i]);
            suf_lcm[i] = lcm(suf_lcm[i + 1], nums[i]);
        }

        long long ans = suf_gcd[0] * suf_lcm[0]; // 不移除元素
        int pre_gcd = 0;
        long long pre_lcm = 1;
        for (int i = 0; i < n; i++) { // 枚举移除 nums[i]
            ans = max(ans, gcd(pre_gcd, suf_gcd[i + 1]) * lcm(pre_lcm, suf_lcm[i + 1]));
            pre_gcd = gcd(pre_gcd, nums[i]);
            pre_lcm = lcm(pre_lcm, nums[i]);
        }
        return ans;
    }
};

int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution solution;
    cout << solution.maxScore(nums) << endl;
    return 0;
}
