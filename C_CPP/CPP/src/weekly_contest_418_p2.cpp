// Problem: Weekly Contest 418 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <numeric>
using namespace std;

class Solution {
public:
    vector<int> remainingMethods(int n, int k, vector<vector<int>>& invocations) {
        vector<vector<int>> g(n);
        for (auto& e : invocations) {
            g[e[0]].push_back(e[1]);
        }

        // 标记所有可疑方法
        vector<int> is_suspicious(n);
        auto dfs = [&](auto&& dfs, int x) -> void {
            is_suspicious[x] = true;
            for (int y : g[x]) {
                if (!is_suspicious[y]) { // 避免无限递归
                    dfs(dfs, y);
                }
            }
        };
        dfs(dfs, k);

        // 检查是否有【非可疑方法】->【可疑方法】的边
        for (auto& e : invocations) {
            if (!is_suspicious[e[0]] && is_suspicious[e[1]]) {
                // 无法移除可疑方法
                vector<int> ans(n);
                iota(ans.begin(), ans.end(), 0);
                return ans;
            }
        }

        // 移除所有可疑方法
        vector<int> ans;
        for (int i = 0; i < n; i++) {
            if (!is_suspicious[i]) {
                ans.push_back(i);
            }
        }
        return ans;
    }
};

int main() {
    int n, k, invocationsSize;
    cin >> n >> k >> invocationsSize;
    vector<vector<int>> invocations(invocationsSize, vector<int>(2));
    for (int i = 0; i < invocationsSize; i++) {
        cin >> invocations[i][0] >> invocations[i][1];
    }
    Solution s;
    vector<int> ans = s.remainingMethods(n, k, invocations);
    for (int x : ans) {
        cout << x << ' ';
    }
    cout << endl;
    return 0;
}
