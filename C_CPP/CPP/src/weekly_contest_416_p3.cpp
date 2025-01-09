// Problem: Weekly Contest 416 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long validSubstringCount(string word1, string word2) {
        vector<int> diff(26, 0);
        for (auto c : word2) {
            diff[c - 'a']--;
        }

        long long res = 0;
        int cnt = count_if(diff.begin(), diff.end(), [](int c) { return c < 0; });
        auto update = [&](int c, int add) {
            diff[c] += add;
            if (add == 1 && diff[c] == 0) {
                // 表明 diff[c] 由 -1 变为 0
                cnt--;
            } else if (add == -1 && diff[c] == -1) {
                // 表明 diff[c] 由 0 变为 -1
                cnt++;
            }
        };

        for (int l = 0, r = 0; l < word1.size(); l++) {
            while (r < word1.size() && cnt > 0) {
                update(word1[r] - 'a', 1);
                r++;
            }
            if (cnt == 0) {
                res += word1.size() - r + 1;
            }
            update(word1[l] - 'a', -1);
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
