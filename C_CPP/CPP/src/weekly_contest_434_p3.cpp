// Problem: Weekly Contest 434 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int maxFrequency(vector<int>& nums, int k) {
        int f0 = 0, f1[51]{}, max_f1 = 0, f2 = 0;
        for (int x : nums) {
            f2 = max(f2, max_f1) + (x == k);
            f1[x] = max(f1[x], f0) + 1;
            f0 += (x == k);
            max_f1 = max(max_f1, f1[x]);
        }
        return max(max_f1, f2);
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n, k;
    cin >> n >> k;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution solution;
    cout << solution.maxFrequency(nums, k) << endl;
    return 0;
}
