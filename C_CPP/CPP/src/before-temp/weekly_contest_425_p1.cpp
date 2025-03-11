// Problem: Weekly Contest 425 Problem 1
#include <iostream>
#include <vector>
#include <climits>
using namespace std;

class Solution {
public:
    int minimumSumSubarray(vector<int>& nums, int l, int r) {
        int n = nums.size();
        int mini = INT_MAX;
        for (int i = 0; i < n; i++) {
            int currsum = 0;
            for (int j = i; j < n; j++) {
                currsum += nums[j];
                int length = j - i + 1;
                if (length >= l && length <= r && currsum > 0) {
                    mini = min(mini, currsum);
                }
            }
        }
        return mini == INT_MAX ? -1 : mini;
    }
};

int main() {
    Solution solution;

    // Input the size of the array
    int n;
    cin >> n;

    // Input the array elements
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }

    // Input the range [l, r]
    int l, r;
    cin >> l >> r;

    // Compute the minimum sum subarray
    int result = solution.minimumSumSubarray(nums, l, r);

    // Output the result
    cout << result << endl;

    return 0;
}
