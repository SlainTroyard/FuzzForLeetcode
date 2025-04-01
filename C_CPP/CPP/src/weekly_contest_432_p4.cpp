// Problem: Weekly Contest 432 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include <deque>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long countNonDecreasingSubarrays(vector<int>& nums, int k) {
        int n = nums.size();
        vector<vector<int>> g(n);
        vector<int> pos_r(n, n);
        stack<int> st;
        for (int i = 0; i < n; i++) {
            int x = nums[i];
            while (!st.empty() && x >= nums[st.top()]) {
                pos_r[st.top()] = i;
                st.pop();
            }
            if (!st.empty()) {
                g[st.top()].push_back(i);
            }
            st.push(i);
        }

        long long ans = 0;
        int cnt = 0, l = 0;
        deque<int> q;
        for (int r = 0; r < n; r++) {
            int x = nums[r];
            while (!q.empty() && nums[q.back()] <= x) {
                q.pop_back();
            }
            q.push_back(r);
            cnt += nums[q.front()] - x;
            while (cnt > k) {
                int out = nums[l];
                for (int i : g[l]) {
                    if (i > r) {
                        break;
                    }
                    cnt -= (out - nums[i]) * (min(pos_r[i], r + 1) - i);
                }
                l++;
                if (!q.empty() && q.front() < l) {
                    q.pop_front();
                }
            }
            ans += r - l + 1;
        }
        return ans;
    }
};

int main() {
    
    int n, k;
    cin >> n >> k;
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    Solution solution;
    cout << solution.countNonDecreasingSubarrays(nums, k) << endl;
    return 0;
}
