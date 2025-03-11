// Problem: Weekly Contest 420 Problem 4
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    vector<bool> findAnswer(vector<int>& parent, string s) {
        int n = parent.size();
        vector<vector<int>> g(n);
        for (int i = 1; i < n; i++) {
            int p = parent[i];
            g[p].push_back(i);
        }

        string dfsStr(n, 0);
        vector<pair<int, int>> nodes(n);
        int time = 0;
        auto dfs = [&](auto&& dfs, int x) -> void {
            nodes[x].first = time;
            for (int y : g[x]) {
                dfs(dfs, y);
            }
            dfsStr[time++] = s[x];
            nodes[x].second = time;
        };
        dfs(dfs, 0);

        string t = "^";
        for (char c : dfsStr) {
            t += '#';
            t += c;
        }
        t += "#$";

        vector<int> halfLen(t.length() - 2);
        halfLen[1] = 1;
        int boxM = 0, boxR = 0;
        for (int i = 2; i < halfLen.size(); i++) { 
            int hl = 1;
            if (i < boxR) {
                hl = min(halfLen[boxM * 2 - i], boxR - i);
            }
            while (t[i - hl] == t[i + hl]) {
                hl++;
                boxM = i;
                boxR = i + hl;
            }
            halfLen[i] = hl;
        }

        auto isPalindrome = [&](int l, int r) -> bool {
            return halfLen[l + r + 1] > r - l;
        };

        vector<bool> ans(n);
        for (int i = 0; i < n; i++) {
            ans[i] = isPalindrome(nodes[i].first, nodes[i].second);
        }
        return ans;
    }
};


int main() {
    int n;
    cin >> n;
    vector<int> parent(n);
    for (int i = 0; i < n; i++) {
        cin >> parent[i];
    }
    string s;
    cin >> s;
    Solution solution;
    vector<bool> ans = solution.findAnswer(parent, s);
    for (bool b : ans) {
        if (b) {
            cout << "true ";
        } else {
            cout << "false ";
        }
    }
    cout << endl;
    return 0;
}
