// Problem: Weekly Contest 431 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <array>
using namespace std;

class Solution {
public:
    vector<int> maximumWeight(vector<vector<int>>& intervals) {
        int n = intervals.size();

        typedef pair<int, int> pii;
        vector<pii> vec;
        vec.push_back({-1, -1});
        for (int i = 0; i < n; i++) {
            auto &seg = intervals[i];
            vec.push_back({seg[0], -1});
            vec.push_back({seg[1], i});
        }
        sort(vec.begin(), vec.end());

        n = vec.size();
        const long long INF = 1e18;
        // 使用类型别名和 std::vector 替换 VLA
        using LLArray = std::array<long long, 5>;
        std::vector<std::array<LLArray, 5>> f(n);

        // 初始化 f[0]
        for (int j = 1; j <= 4; j++) f[0][j] = {INF, INF, INF, INF, INF};
        f[0][0] = {0, INF, INF, INF, INF};

        // 动态规划
        for (int i = 1; i < n; i++) {
            for (int j = 0; j <= 4; j++) f[i][j] = f[i - 1][j];
            int idx = vec[i].second;
            if (idx >= 0) {
                int l = intervals[idx][0];
                int head = 0, tail = i - 1;
                while (head < tail) {
                    int mid = (head + tail + 1) >> 1;
                    if (vec[mid].first < l) head = mid;
                    else tail = mid - 1;
                }

                for (int j = 1; j <= 4; j++) {
                    auto tmp = f[head][j - 1];
                    tmp[0] -= intervals[idx][2];
                    tmp[j] = idx;
                    sort(tmp.begin(), tmp.end());
                    f[i][j] = min(f[i][j], tmp); // std::array 支持 lexicographical 比较
                }
            }
        }

        // 计算答案
        array<long long, 5> ans = {INF, INF, INF, INF, INF};
        for (int j = 1; j <= 4; j++) ans = min(ans, f[n - 1][j]);
        vector<int> ret;
        for (int j = 1; j <= 4; j++) if (ans[j] < INF) ret.push_back(ans[j]);
        return ret;
    }
};

int main() {
    int n;
    cin >> n;
    vector<vector<int>> vec(n, vector<int>(3));
    for (int i = 0; i < n; i++) {
        cin >> vec[i][0] >> vec[i][1] >> vec[i][2];
    }
    Solution sol;
    vector<int> ans = sol.maximumWeight(vec);
    for (int i = 0; i < ans.size(); i++) {
        cout << ans[i] << " ";
    }
    cout << endl;
    return 0;
}
