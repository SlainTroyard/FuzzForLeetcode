// Problem: Weekly Contest 415 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> getSneakyNumbers(vector<int>& nums) {
        int n = nums.size() - 2;
        int xor_all = n ^ (n + 1);
        for (int i = 0; i < nums.size(); i++) {
            xor_all ^= i ^ nums[i];
        }
        int shift = __builtin_ctz(xor_all);

        vector<int> ans(2);
        for (int i = 0; i < nums.size(); i++) {
            if (i < n) {
                ans[i >> shift & 1] ^= i;
            }
            ans[nums[i] >> shift & 1] ^= nums[i];
        }
        return ans;
    }
};

int main() {
    int numSize;
    cin >> numSize;
    numSize += 2;
    vector<int> nums(numSize);
    for (int i = 0; i < numSize; i++) {
        cin >> nums[i];
    }
    Solution solution;
    vector<int> result = solution.getSneakyNumbers(nums);
    for (int i = 0; i < 2; i++) {
        cout << result[i] << " ";
    }
    return 0;
}
