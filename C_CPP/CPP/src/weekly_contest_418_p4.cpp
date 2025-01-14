// Problem: Weekly Contest 418 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
using namespace std;

class Solution {
public:
    vector<int> gcdValues(vector<int>& nums, vector<long long>& queries) {
        int mx = *std::max_element(nums.begin(), nums.end());
        vector<int> cnt_x(mx + 1);
        for (int x : nums) {
            cnt_x[x]++;
        }

        vector<long long> cnt_gcd(mx + 1);
        for (int i = mx; i > 0; i--) {
            int c = 0;
            for (int j = i; j <= mx; j += i) {
                c += cnt_x[j];
                cnt_gcd[i] -= cnt_gcd[j];
            }
            cnt_gcd[i] += (long long) c * (c - 1) / 2;
        }
        partial_sum(cnt_gcd.begin(), cnt_gcd.end(), cnt_gcd.begin());

        vector<int> ans(queries.size());
        for (int i = 0; i < queries.size(); i++) {
            ans[i] = std::upper_bound(cnt_gcd.begin(), cnt_gcd.end(), queries[i]) - cnt_gcd.begin();
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
    int q;
    cin >> q;
    vector<long long> queries(q);
    for (int i = 0; i < q; i++) {
        cin >> queries[i];
    }
    Solution solution;
    vector<int> ans = solution.gcdValues(nums, queries);
    for (int x : ans) {
        cout << x << " ";
    }
    cout << endl;
    return 0;
}
