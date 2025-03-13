// Problem: Weekly Contest 421 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int lengthAfterTransformations(string s, int t) {
        const int MOD = 1e9 + 7;
        long long cnt[26] = {0};
        for (char c : s) cnt[c - 'a']++;
        for (int i = 1; i <= t; i++) {
            int nxt[26] = {0};
            for (int j = 0; j < 25; j++) nxt[j + 1] = cnt[j];
            nxt[0] = (nxt[0] + cnt[25]) % MOD;
            nxt[1] = (nxt[1] + cnt[25]) % MOD;
            for (int j = 0; j < 26; j++) cnt[j] = nxt[j];
        }
        long long ans = 0;
        for (int i = 0; i < 26; i++) ans = (ans + cnt[i]) % MOD;
        return ans;
    }
};

int main() {
    string s;
    int t;
    cin >> s >> t;
    Solution solution;
    cout << solution.lengthAfterTransformations(s, t) << endl;
    return 0;
}
