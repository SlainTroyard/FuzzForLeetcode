// Problem: Weekly Contest 437 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <utility>
using namespace std;

class Solution {
public:
    bool maxSubstringLength(string s, int K) {
        int n = s.size();

        vector<int> pos[26];
        for (int i = 0; i < n; i++) {
            int c = s[i] - 'a';
            pos[c].push_back(i);
        }

        typedef pair<int, int> pii;
        vector<pii> vec;
        for (int i = 0; i < 26; i++) if (!pos[i].empty()) {
            int l = pos[i][0], r = pos[i].back();
            bool flag = true;
            while (flag) {
                flag = false;
                for (int j = 0; j < 26; j++) {
                    int cnt = upper_bound(pos[j].begin(), pos[j].end(), r) - lower_bound(pos[j].begin(), pos[j].end(), l);
                    if (cnt > 0 && cnt < pos[j].size()) {
                        l = min(l, pos[j][0]);
                        r = max(r, pos[j].back());
                        flag = true;
                    }
                }
            }
            if (l > 0 || r < n - 1) vec.push_back({r, l});
        }

        sort(vec.begin(), vec.end());
        int R = -1, cnt = 0;
        for (pii p : vec) if (p.second > R) {
            R = p.first;
            cnt++;
        }
        return cnt >= K;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    string s;
    int K;
    cin >> s >> K;
    Solution sol;
    cout << sol.maxSubstringLength(s, K) << endl;
    return 0;
}
