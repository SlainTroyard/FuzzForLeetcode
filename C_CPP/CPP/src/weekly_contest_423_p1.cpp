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
    
    // Read the size of the array and the subarray length
    cin >> n;
    
    vector<int> nums(n);
    
    // Read the elements of the array
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }

    // Read the length of the subarray
    cin >> k;

    // Create a Solution object and call the function
    Solution sol;
    if (sol.hasIncreasingSubarrays(nums, k)) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }

    return 0;
}
