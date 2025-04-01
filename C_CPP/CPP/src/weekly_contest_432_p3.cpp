// Problem: Weekly Contest 432 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <queue>
#include <algorithm>
#include <climits>
#include <cstring>
using namespace std;

class Solution {
public:
    int minMaxWeight(int n, vector<vector<int>>& edges, int threshold) {
        auto check = [&](int lim) {
            vector<int> e[n];
            for (auto &edge : edges) if (edge[2] <= lim) e[edge[1]].push_back(edge[0]);

            bool vis[n];
            memset(vis, 0, sizeof(vis));
            queue<int> q;
            q.push(0); vis[0] = true;
            while (!q.empty()) {
                int sn = q.front(); q.pop();
                for (int fn : e[sn]) if (!vis[fn]) q.push(fn), vis[fn] = true;
            }

            for (int i = 0; i < n; i++) if (!vis[i]) return false;
            return true;
        };

        int mx = 0;
        for (auto &edge : edges) mx = max(mx, edge[2]);
        if (!check(mx)) return -1;

        int head = 0, tail = mx;
        while (head < tail) {
            int mid = (head + tail) >> 1;
            if (check(mid)) tail = mid;
            else head = mid + 1;
        }
        return head;
    }
};

int main() {
    
    int n;
    cin >> n;
    int threshold;
    cin >> threshold;
    vector<vector<int>> edges;
    for (int i = 0; i < n; i++) {
        vector<int> edge;
        for (int j = 0; j < 3; j++) {
            int x;
            cin >> x;
            edge.push_back(x);
        }
        edges.push_back(edge);
    }
    Solution solution;
    cout << solution.minMaxWeight(n, edges, threshold) << endl;
    return 0;
}
