// Problem: Weekly Contest 427 Problem 3
#include <iostream>
#include <vector>
#include <map>
#include <algorithm>
#include <climits>
using namespace std;

class Solution {
public:
    long long maxSubarraySum(vector<int>& v, int k) {
        map<int, long long> m;
        long long ans = LLONG_MIN;
        long long sm = 0;
        for (int i = 0; i < v.size(); i++) {
            sm += v[i];
            int curSZ = i + 1;
            if (curSZ % k == 0)
                ans = max(ans, sm);
            int y = (curSZ % k);
            if (m.find(y) != m.end()) {
                ans = max(ans, sm - m[y]);
                m[y] = min(m[y], sm);
            } else
                m[y] = sm;
        }
        return ans;
    }
};

int main() {
    Solution solution;
    int n, k;

    // Input array size and k value
    cin >> n;
    cin >> k;

    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }

    // Call the maxSubarraySum function and print the result
    long long result = solution.maxSubarraySum(nums, k);
    cout << result << endl;

    return 0;
}
