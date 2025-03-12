// Problem: Weekly Contest 434 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>
#include <algorithm>
#include <numeric>
#include <functional> // 添加此头文件以使用std::function

using namespace std;

class Solution {
public:
    vector<vector<int>> supersequences(vector<string>& words) {
        int all = 0, mask2 = 0;
        vector<int> g[26];
        for (auto& s : words) {
            int x = s[0] - 'a', y = s[1] - 'a';
            all |= 1 << x | 1 << y;
            if (x == y) {
                mask2 |= 1 << x;
            }
            g[x].push_back(y);
        }

        auto has_cycle = [&](int sub) -> bool {
            int color[26] = {0};
            function<bool(int)> dfs;
            dfs = [&](int x) -> bool { // 修改后的lambda定义
                color[x] = 1;
                for (int y : g[x]) {
                    if ((sub >> y & 1) == 0) {
                        continue;
                    }
                    if (color[y] == 1 || (color[y] == 0 && dfs(y))) {
                        return true;
                    }
                }
                color[x] = 2;
                return false;
            };
            for (int i = 0; i < 26; ++i) {
                if ((sub >> i & 1) && color[i] == 0 && dfs(i)) {
                    return true;
                }
            }
            return false;
        };

        unordered_set<int> st;
        int max_size = 0;
        int mask1 = all ^ mask2;
        int sub = mask1;
        do {
            int size = __builtin_popcount(sub); // 使用GCC内置函数
            if (size >= max_size && !has_cycle(sub)) {
                if (size > max_size) {
                    max_size = size;
                    st.clear();
                }
                st.insert(sub);
            }
            sub = (sub - 1) & mask1;
        } while (sub != mask1);

        vector<vector<int>> ans;
        for (int sub : st) {
            vector<int> cnt(26);
            for (int i = 0; i < 26; ++i) {
                cnt[i] = (all >> i & 1) + ((all ^ sub) >> i & 1);
            }
            ans.push_back(cnt);
        }
        return ans;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n;
    cin >> n;
    vector<string> words(n);
    for (int i = 0; i < n; i++) {
        cin >> words[i];
    }
    Solution sol;
    vector<vector<int>> ans = sol.supersequences(words);
    for (auto& cnt : ans) {
        for (int i = 0; i < 26; i++) {
            cout << cnt[i] << " ";
        }
        cout << endl;
    }
    return 0;
}
