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

    int n;
    cin >> n;

    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }

    int l, r;
    cin >> l >> r;

    int result = solution.minimumSumSubarray(nums, l, r);

    cout << result << endl;

    return 0;
}
