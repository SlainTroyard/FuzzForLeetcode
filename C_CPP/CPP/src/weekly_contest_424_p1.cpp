// Problem: Weekly Contest 424 Problem 1
#include <iostream>
#include <vector>
#include <cstdlib> // For abs()

using namespace std;

class Solution {
public:
    int countValidSelections(vector<int>& nums) {
        int n = nums.size();
        int res = 0;
        vector<int> left(n, 0);
        vector<int> right(n, 0);

        // Compute prefix sums from the left
        for (int i = 1; i < n; ++i) {
            left[i] = left[i - 1] + nums[i - 1];
        }

        // Compute prefix sums from the right
        for (int i = 1; i < n; ++i) {
            right[n - i - 1] = right[n - i] + nums[n - i];
        }

        // Iterate through each element to count valid selections
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

/**
 * Main Function
 * 
 * Reads input, invokes the countValidSelections method, and prints the result.
 */
int main(){
    int n;
    // Read the number of elements in the nums array
    cin >> n;

    vector<int> nums(n);
    // Read the elements of the nums array
    for(int i = 0; i < n; i++) {
        cin >> nums[i];
    }

    // Create an instance of the Solution class
    Solution solution;

    // Call the countValidSelections method and store the result
    int result = solution.countValidSelections(nums);

    // Print the result
    cout << result << endl;

    return 0;
}