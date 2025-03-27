// Problem: Weekly Contest 423 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int maxIncreasingSubarrays(vector<int>& nums) {
        int flag = 0;
        int prev = 0;
        int curr = 1;
        int ans = 0;

        for (int i = 1; i < nums.size(); i++) {
            if (nums[i - 1] < nums[i]) {
                curr++;  
            } else {
                prev = curr;  
                curr = 1;     
            }
            ans = max(ans, max(curr / 2, min(prev, curr)));
        }
        return ans;  
    }
};

int main() {
    int n;
    
    cin >> n;
    
    vector<int> nums(n);
    
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    
    Solution sol;
    int result = sol.maxIncreasingSubarrays(nums);
    
    cout << result << endl;
    
    return 0;
}
