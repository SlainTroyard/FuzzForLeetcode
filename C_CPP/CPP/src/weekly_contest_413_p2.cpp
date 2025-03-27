// Problem: Weekly Contest 413 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    vector<int> resultsArray(vector<vector<int>>& queries, int k) {
        vector<int> ans(queries.size(), -1);
        priority_queue<int> pq;
        for (int i = 0; i < queries.size(); i++) {
            pq.push(abs(queries[i][0]) + abs(queries[i][1]));
            if (pq.size() > k) {
                pq.pop();
            }
            if (pq.size() == k) {
                ans[i] = pq.top();
            }
        }
        return ans;
    }
};
int main() {
    int queriesSize, k;
    cin >> queriesSize >> k;
    vector<vector<int>> queries(queriesSize, vector<int>(2));
    for (int i = 0; i < queriesSize; i++) {
        cin >> queries[i][0] >> queries[i][1];
    }
    Solution sol;
    vector<int> ans = sol.resultsArray(queries, k);
    for (int i = 0; i < ans.size(); i++) {
        cout << ans[i] << " ";
    }
    cout << endl;
    return 0;
}
