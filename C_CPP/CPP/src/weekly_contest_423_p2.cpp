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

        // Traverse through the nums array
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i - 1] < nums[i]) {
                curr++;  // Increase the length of the current increasing subarray
            } else {
                prev = curr;  // Update the previous subarray length
                curr = 1;     // Reset current subarray length
            }
            // Update the answer by considering both the current and previous subarray lengths
            ans = max(ans, max(curr / 2, min(prev, curr)));
        }
        return ans;  // Return the maximum length of increasing subarrays
    }
};

int main() {
    int n;
    
    // Input the size of the array
    cin >> n;
    
    vector<int> nums(n);
    
    // Input the array elements
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    
    // Create a Solution object and call the function to get the result
    Solution sol;
    int result = sol.maxIncreasingSubarrays(nums);
    
    // Output the result
    cout << result << endl;
    
    return 0;
}
