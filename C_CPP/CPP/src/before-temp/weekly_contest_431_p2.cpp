// Problem: Weekly Contest 431 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <stack>
using namespace std;

class Solution {
public:
    long long calculateScore(string s) {
        stack<int> stk[26];
        long long ans = 0;
        for (int i = 0; i < s.size(); i++) {
            int c = s[i] - 'a';
            if (!stk[25 - c].empty()) {
                ans += i - stk[25 - c].top();
                stk[25 - c].pop();
            } else {
                stk[c].push(i);                
            }
        }
        return ans;
    }
};

int main() {
    string s;
    cin >> s;
    Solution sol;
    cout << sol.calculateScore(s) << endl;
    return 0;
}
