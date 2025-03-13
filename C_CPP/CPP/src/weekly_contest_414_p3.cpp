// Problem: Weekly Contest 414 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    long long findMaximumScore(vector<int>& nums) {
        long long ans = 0;
        int mx = 0;
        for (int i = 0; i + 1 < nums.size(); i++) {
            mx = max(mx, nums[i]);
            ans += mx;
        }
        return ans;
    }
};
int main() {
    int numsSize;
    cin >> numsSize;
    vector<int> nums(numsSize);
    for (int i = 0; i < numsSize; i++) {
        cin >> nums[i];
    }
    Solution sol;
    cout << sol.findMaximumScore(nums) << endl;
    return 0;
}
