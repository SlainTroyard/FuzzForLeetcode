// Problem: Weekly Contest 433 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cstring>
#include <climits>
using namespace std;

class Solution {
public:
    long long minCost(int n, vector<vector<int>>& cost) {
        vector<vector<vector<long long>>> memo(n / 2, vector<vector<long long>>(4, vector<long long>(4, -1)));

        struct DFS {
            const vector<vector<vector<long long>>>& memo;  // 引用 memo
            const vector<vector<int>>& cost;                // 引用 cost
            int n;                                          // 保存 n

            DFS(const vector<vector<vector<long long>>>& m, const vector<vector<int>>& c, int nn)
                : memo(m), cost(c), n(nn) {}

            long long operator()(int i, int pre_j, int pre_k) const {
                if (i < 0) {
                    return 0;
                }

                const auto& res = memo[i][pre_j][pre_k];
                if (res != -1) {
                    return res;
                }

                long long min_res = LLONG_MAX;
                for (int j = 0; j < 3; j++) {
                    if (j == pre_j) {
                        continue;
                    }
                    for (int k = 0; k < 3; k++) {
                        if (k != pre_k && k != j) {
                            long long temp = (*this)(i - 1, j, k) + cost[i][j] + cost[n - 1 - i][k];
                            min_res = min(min_res, temp);
                        }
                    }
                }

                const_cast<vector<vector<vector<long long>>>&>(memo)[i][pre_j][pre_k] = min_res;
                return min_res;
            }
        };

        DFS dfs(memo, cost, n);
        return dfs(n / 2 - 1, 3, 3);
    }
};


int main() {
    // TODO: Add the base I/O interface here
    int n;
    cin >> n;
    vector<vector<int>> cost(n, vector<int>(3));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < 3; j++) {
            cin >> cost[i][j];
        }
    }
    Solution solution;
    cout << solution.minCost(n, cost) << endl;
    return 0;
}
