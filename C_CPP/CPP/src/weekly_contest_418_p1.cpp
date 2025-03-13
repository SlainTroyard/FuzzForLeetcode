// Problem: Weekly Contest 418 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    int maxGoodNumber(vector<int>& nums) {
        std::sort(nums.begin(), nums.end(), [](int a, int b) {
            int len_a = __lg(a) + 1;
            int len_b = __lg(b) + 1;
            return (a << len_b | b) > (b << len_a | a);
        });

        int ans = 0;
        for (int x : nums) {
            ans = ans << (__lg(x) + 1) | x;
        }
        return ans;
    }
};

int main() {
    int numSize;
    cin >> numSize;
    vector<int> nums(numSize);
    for (int i = 0; i < numSize; i++) {
        cin >> nums[i];
    }
    Solution s;
    cout << s.maxGoodNumber(nums) << endl;
    return 0;
}
