// Problem: Weekly Contest 424 Problem 4
#include <string>
#include <iostream>
#include <vector>
#include <climits>
#include <algorithm>

using namespace std;

class Solution {
public:
    int minDifference(vector<int>& nums) {
        int n = nums.size();
        int min_l = INT_MAX, max_r = 0;
        for (int i = 0; i < n; i++) {
            if (nums[i] != -1 && (i > 0 && nums[i - 1] == -1 || i < n - 1 && nums[i + 1] == -1)) {
                min_l = min(min_l, nums[i]);
                max_r = max(max_r, nums[i]);
            }
        }

        int ans = 0;
        auto update_ans = [&](int l, int r, bool big) {
            int d = (min(r - min_l, max_r - l) + 1) / 2;
            if (big) {
                d = min(d, (max_r - min_l + 2) / 3); 
            }
            ans = max(ans, d);
        };

        int pre_i = -1;
        for (int i = 0; i < n; i++) {
            if (nums[i] == -1) {
                continue;
            }
            if (pre_i >= 0) {
                if (i - pre_i == 1) {
                    ans = max(ans, abs(nums[i] - nums[pre_i]));
                } else {
                    update_ans(min(nums[pre_i], nums[i]), max(nums[pre_i], nums[i]), i - pre_i > 2);
                }
            } else if (i > 0) {
                update_ans(nums[i], nums[i], false);
            }
            pre_i = i;
        }
        if (0 <= pre_i && pre_i < n - 1) {
            update_ans(nums[pre_i], nums[pre_i], false);
        }
        return ans;
    }
};

int main() {
    int n;
    cin >> n;

    vector<int> nums(n);
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }

    Solution sol;
    int result = sol.minDifference(nums);

    cout << result << endl;

    return 0;
}