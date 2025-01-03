// Problem: Weekly Contest 424 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int minZeroArray(vector<int>& nums, vector<vector<int>>& queries) {
        int n = nums.size();
        vector<int> d(n+1);
        
        d[0] = nums[0];
        for (int i=1; i<n; ++i) d[i] = nums[i] - nums[i-1];

        
        int acc = 0;
        int cur = -1;
        int ans = 0;
        while ((acc <= 0) && cur < n) {
            ++cur;
            acc += d[cur];
        }
        if (cur == n) return 0;

        int m = queries.size();
        for (int i=0; i<m; ++i) {
            d[queries[i][1]+1] += queries[i][2];
            d[queries[i][0]] -= queries[i][2];
            if ((cur >= queries[i][0]) && (cur <= queries[i][1])) {
                acc -= queries[i][2];
                while ((acc <= 0) && cur < n) {
                    ++cur;
                    acc += d[cur];
                }
                if (cur == n) return i+1;
            }
        }
        return -1;        
    }
};

int main() {
    Solution sol;
    
    // Read the size of the nums array
    int n;
    cin >> n;
    
    // Read the nums array
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    
    // Read the number of queries
    int m;
    cin >> m;
    
    // Read the queries
    vector<vector<int>> queries(m, vector<int>(3));
    for (int i = 0; i < m; i++) {
        cin >> queries[i][0] >> queries[i][1] >> queries[i][2];
    }
    
    // Call the solution function and print the result
    cout << sol.minZeroArray(nums, queries) << endl;
    
    return 0;
}
