// Problem: Weekly Contest 433 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <stack>
#include <climits>
using namespace std;

class Solution {
public:
    long long minMaxSubarraySum(vector<int>& nums, int k) {
        auto count = [&](int m) -> long long {
            return m > k ? 1LL * (m * 2 - k + 1) * k / 2 : 1LL * (m + 1) * m / 2;
        };

        auto sumSubarrayMins = [&]() -> long long {
            long long res = 0;
            stack<int> st;
            st.push(-1);
            for (int r = 0; r < nums.size(); r++) {
                while (st.size() > 1 && nums[st.top()] >= nums[r]) {
                    int i = st.top();
                    st.pop();
                    int l = st.top();
                    long long cnt = count(r - l - 1) - count(i - l - 1) - count(r - i - 1);
                    res += nums[i] * cnt;
                }
                st.push(r);
            }
            return res;
        };

        nums.push_back(INT_MIN / 2);
        long long ans = sumSubarrayMins();
        for (int& x : nums) {
            x = -x;
        }
        nums.back() *= -1;
        ans -= sumSubarrayMins();
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
    Solution sol;
    cout << sol.minMaxSubarraySum(nums, k) << endl;
    return 0;
}
