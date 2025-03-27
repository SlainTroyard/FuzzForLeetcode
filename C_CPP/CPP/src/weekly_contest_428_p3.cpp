// Problem: Weekly Contest 428 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int beautifulSplits(vector<int>& nums) {
        int n = nums.size();
        vector<vector<int>> lcp(n + 1, vector<int>(n + 1));
        for (int i = n - 1; i >= 0; i--) {
            for (int j = n - 1; j >= i; j--) {
                if (nums[i] == nums[j]) {
                    lcp[i][j] = lcp[i + 1][j + 1] + 1;
                }
            }
        }

        int ans = 0;
        for (int i = 1; i < n - 1; i++) {
            for (int j = i + 1; j < n; j++) {
                if (i <= j - i && lcp[0][i] >= i || lcp[i][j] >= j - i) {
                    ans++;
                }
            }
        }
        return ans;
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

    int result = solution.beautifulSplits(nums);
    cout << result << endl;

    return 0;
}

