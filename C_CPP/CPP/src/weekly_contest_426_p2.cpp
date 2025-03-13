// Problem: Weekly Contest 426 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <algorithm>
using namespace std;

class Solution {
public:
    int getLargestOutlier(vector<int>& nums) {
        unordered_map<int, int> ctr;
        int sm = 0;

        for (int num : nums) {
            ctr[num]++;
            sm += num;
        }

        vector<int> cands;
        for (const auto& [num, count] : ctr) {
            cands.push_back(num);
        }

        sort(cands.rbegin(), cands.rend());

        for (int n : cands) {
            int d = (sm - n) / 2;
            int m = (sm - n) % 2;
            if (m == 0 && ctr.count(d) && (d != n || ctr[d] > 1)) {
                return n;
            }
        }
        return -1;
    }
};

int main() {
    Solution solution;
    int n;

    // Input the number of elements
    cin >> n;

    vector<int> nums(n);

    // Input the elements of the array
    for (int i = 0; i < n; ++i) {
        cin >> nums[i];
    }

    // Call the method and output the result
    int result = solution.getLargestOutlier(nums);
    cout << result << endl;

    return 0;
}
