// Problem: Weekly Contest 418 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    vector<vector<int>> constructGridLayout(int n, vector<vector<int>>& edges) {
        vector<vector<int>> g(n);
        for (auto& e : edges) {
            int x = e[0], y = e[1];
            g[x].push_back(y);
            g[y].push_back(x);
        }

        int x = 0;
        for (int i = 0; i < g.size(); i++) {
            if (g[i].size() < g[x].size()) {
                x = i;
            }
        }

        vector<int> row = {x};
        vector<int> vis(n);
        vis[x] = true;
        int deg_st = g[x].size();
        do {
            int nxt = -1;
            for (int y : g[x]) {
                if (!vis[y] && (nxt < 0 || g[y].size() < g[nxt].size())) {
                    nxt = y;
                }
            }
            x = nxt;
            row.push_back(x);
            vis[x] = true;
        } while (g[x].size() > deg_st);

        vector<vector<int>> ans(n / row.size());
        ans[0] = move(row);
        for (int i = 1; i < ans.size(); i++) {
            for (int x : ans[i - 1]) {
                for (int y : g[x]) {
                    if (!vis[y]) {
                        vis[y] = true;
                        ans[i].push_back(y);
                        break;
                    }
                }
            }
        }
        return ans;
    }
};

int main() {
    int n, edgesSize;
    cin >> n >> edgesSize;
    vector<vector<int>> edges(edgesSize, vector<int>(2));
    for (int i = 0; i < edgesSize; i++) {
        cin >> edges[i][0] >> edges[i][1];
    }
    Solution sol;
    vector<vector<int>> res = sol.constructGridLayout(n, edges);
    for (auto& row : res) {
        for (int x : row) {
            cout << x << " ";
        }
        cout << endl;
    }
    return 0;
}
