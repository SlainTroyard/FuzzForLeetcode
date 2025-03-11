// Problem: Weekly Contest 430 Problem 3
#include <iostream>
#include <vector>
#include <unordered_map>
#include <numeric> // For gcd
using namespace std;

class Solution {
public:
    long long numberOfSubsequences(vector<int>& nums) {
        int n = nums.size();
        unordered_map<int, int> suf;
        for (int i = 4; i < n - 2; i++) {
            int c = nums[i];
            for (int j = i + 2; j < n; j++) {
                int d = nums[j];
                int g = gcd(c, d);
                suf[(d / g) << 16 | (c / g)]++;
            }
        }

        long long ans = 0;
        for (int i = 2; i < n - 4; i++) {
            int b = nums[i];
            for (int j = 0; j < i - 1; j++) {
                int a = nums[j];
                int g = gcd(a, b);
                ans += suf[(a / g) << 16 | (b / g)];
            }
            int c = nums[i + 2];
            for (int j = i + 4; j < n; j++) {
                int d = nums[j];
                int g = gcd(c, d);
                suf[(d / g) << 16 | (c / g)]--;
            }
        }
        return ans;
    }
};

int main() {
    Solution solution;
    int n;
    // Input the size of the array
    cin >> n;
    // Input the elements of the array
    vector<int> nums(n);
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }
    // Call the numberOfSubsequences function
    long long result = solution.numberOfSubsequences(nums);
    // Output the result
    cout << result << endl;
    return 0;
}

