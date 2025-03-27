// Problem: Weekly Contest 437 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool hasSpecialSubstring(string s, int k) {
        int n = s.size();
        int cnt = 0;
        for (int i = 0; i < n; i++) {
            cnt++;
            if (i == n - 1 || s[i] != s[i + 1]) {
                if (cnt == k) {
                    return true;
                }
                cnt = 0;
            }
        }
        return false;
    }
};

int main() {
    string s;
    int k;
    cin >> s >> k;
    Solution sol;
    cout << sol.hasSpecialSubstring(s, k) << endl;
    return 0;
}
