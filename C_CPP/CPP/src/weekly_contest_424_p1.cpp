// Problem: Weekly Contest 424 Problem 1
#include <iostream>
#include <vector>
#include <cstdlib> 

using namespace std;

class Solution {
public:
    int countValidSelections(vector<int>& nums) {
        int n = nums.size();
        int res = 0;
        vector<int> left(n, 0);
        vector<int> right(n, 0);

        for (int i = 1; i < n; ++i) {
            left[i] = left[i - 1] + nums[i - 1];
        }

        for (int i = 1; i < n; ++i) {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        for (int i = 0; i < n; ++i) {
            if (nums[i] != 0) {
                continue;
            }
            if (left[i] == right[i]) {
                res += 2;
            }
            if (abs(left[i] - right[i]) == 1) {
                res += 1;
            }
        }
        return res;
    }
};


int main(){
    int n;
    cin >> n;

    vector<int> nums(n);
    for(int i = 0; i < n; i++) {
        cin >> nums[i];
    }

    Solution solution;

    int result = solution.countValidSelections(nums);

    cout << result << endl;

    return 0;
}
