// Problem: Weekly Contest 420 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

const int MX = 1'000'001;
int LPF[MX];

auto init = [] {
    for (int i = 2; i < MX; i++) {
        if (LPF[i] == 0) {
            for (int j = i; j < MX; j += i) {
                if (LPF[j] == 0) {
                    LPF[j] = i;
                }
            }
        }
    }
    return 0;
}();

class Solution {
public:
    int minOperations(vector<int>& nums) {
        int ans = 0;
        for (int i = nums.size() - 2; i >= 0; i--) {
            if (nums[i] > nums[i + 1]) {
                nums[i] = LPF[nums[i]];
                if (nums[i] > nums[i + 1]) {
                    return -1;
                }
                ans++;
            }
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
    Solution sol;
    cout << sol.minOperations(nums) << endl;
    return 0;
}
