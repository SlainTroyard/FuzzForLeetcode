// Problem: Weekly Contest 416 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long validSubstringCount(string word1, string word2) {
        vector<int> count(26, 0);
        for (auto c : word2) {
            count[c - 'a']++;
        }

        int n = word1.size();
        vector<vector<int>> pre_count(n + 1, vector<int>(26, 0));
        for (int i = 1; i <= n; i++) {
            pre_count[i].assign(pre_count[i - 1].begin(), pre_count[i - 1].end());
            pre_count[i][word1[i - 1] - 'a']++;
        }

        auto get = [&](int l, int r) {
            int border = l;
            while (l < r) {
                int m = l + r >> 1;
                bool f = true;
                for (int i = 0; i < 26; i++) {
                    if (pre_count[m][i] - pre_count[border - 1][i] < count[i]) {
                        f = false;
                        break;
                    }
                }
                if (f) {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            return l;
        };

        long long res = 0;
        for (int l = 1; l <= n; l++) {
            int r = get(l, n + 1);
            res += n - r + 1;
        }
        return res;
    }
};

int main() {
    int len1 = 0, len2 = 0;
    cin >> len1;
    string word1;
    cin >> word1;
    cin >> len2;
    string word2;
    cin >> word2;
    Solution s;
    cout << s.validSubstringCount(word1, word2) << endl;
    return 0;
}
