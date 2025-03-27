// Problem: Weekly Contest 429 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>
using namespace std;

class Solution {
public:
    int minimumOperations(vector<int>& nums) {
        unordered_set<int> seen;
        for (int i = nums.size() - 1; i >= 0; i--) {
            if (!seen.insert(nums[i]).second) { 
                return i / 3 + 1;
            }
        }
        return 0;
    }
};

int main() {
    Solution solution;
    int n;

    cin >> n;

    vector<int> nums(n);
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }

    int result = solution.minimumOperations(nums);
    cout << result << endl;

    return 0;
}
