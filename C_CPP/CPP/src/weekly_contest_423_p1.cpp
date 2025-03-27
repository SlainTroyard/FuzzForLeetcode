// Problem: Weekly Contest 423 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool hasIncreasingSubarrays(const vector<int>& nums, int k) {
        auto mono = [&](int idx) -> bool {
            for (int i = idx; i < idx + k - 1; ++i) {
                if (nums[i] >= nums[i + 1]) return false;
            }
            return true;
        };

        for (int idx = 0; idx <= nums.size() - 2 * k; ++idx) {
            if (mono(idx) && mono(idx + k)) {
                return true;
            }
        }
        return false;
    }
};

int main() {
    int n, k;
    
    cin >> n;
    
    vector<int> nums(n);
    
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }

    cin >> k;

    Solution sol;
    if (sol.hasIncreasingSubarrays(nums, k)) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }

    return 0;
}
