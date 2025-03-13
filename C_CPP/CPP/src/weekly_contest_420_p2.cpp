// Problem: Weekly Contest 420 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    int numberOfSubstrings(string s, int k) {
        int ans = 0, left = 0, cnt[26]{};
        for (char c : s) {
            cnt[c - 'a']++;
            while (cnt[c - 'a'] >= k) {
                cnt[s[left] - 'a']--;
                left++;
            }
            ans += left;
        }
        return ans;
    }
};

int main() {
    string s;
    int k;
    cin >> s >> k;
    Solution sol;
    cout << sol.numberOfSubstrings(s, k) << endl;
    return 0;
}
