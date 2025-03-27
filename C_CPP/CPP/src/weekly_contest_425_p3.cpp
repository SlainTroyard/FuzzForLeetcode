// Problem: Weekly Contest 425 Problem 3
#include <iostream>
#include <vector>
#include <algorithm>
#include <numeric>
#include <set>
#include <cstring>

using namespace std;

class Solution {
public:
    int minArraySum(vector<int>& nums, int k, int op1, int op2) {
        int n = nums.size();
        sort(nums.begin(), nums.end());

        auto m1 = lower_bound(nums.begin(), nums.end(), k) - nums.begin();
        auto m2 = lower_bound(nums.begin(), nums.end(), 2 * k - 1) - nums.begin();

        set<int> candidates; 
        int swapCnt = 0;

        int i = n - 1;
        while (i >= m2 && op1 > 0) {
            nums[i] = (nums[i] + 1) / 2;
            op1--;
            if (op2 > 0) {
                nums[i] -= k;
                op2--;
            }
            i--;
        }

        int j = m1;
        while (j <= i && op2 > 0) {
            if (k % 2 == 1 && nums[j] % 2 == 0) {
                candidates.insert(j);
            }
            nums[j] -= k;
            op2--;
            j++;
        }

        while (i >= j && op1 > 0) {
            if (k % 2 == 1 && nums[i] % 2 == 1) {
                swapCnt++;
            }
            nums[i] = (nums[i] + 1) / 2;
            op1--;
            i--;
        }

        vector<pair<int, int>> arr;
        for (int idx = 0; idx < j; idx++) {
            arr.emplace_back(nums[idx], idx);
        }
        sort(arr.begin(), arr.end()); 

        while (op1 > 0 && !arr.empty()) {
            auto [num, idx] = arr.back();
            arr.pop_back();
            nums[idx] = (num + 1) / 2;
            op1--;

            if (candidates.count(idx) && swapCnt > 0) {
                swapCnt--;
                nums[idx] -= 1;
            }
        }

        return accumulate(nums.begin(), nums.end(), 0);
    }
};

int main() {
    int n, k, op1, op2;
    cin >> n >> k >> op1 >> op2;
    
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    
    Solution solution;
    
    int result = solution.minArraySum(nums, k, op1, op2);
    cout << result << endl;

    return 0;
}
