// Problem: Weekly Contest 434 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

class Solution {
public:
    int countPartitions(vector<int>& nums) {
        int s = reduce(nums.begin(), nums.end());
        return s % 2 ? 0 : nums.size() - 1;
    }
};

int main() {
    int n;
    cin >> n;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution sol;
    cout << sol.countPartitions(nums) << endl;
    return 0;
}
