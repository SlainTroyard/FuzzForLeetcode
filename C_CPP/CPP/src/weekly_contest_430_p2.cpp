// Problem: Weekly Contest 430 Problem 2
#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

class Solution {
public:
    string answerString(string s, int k) {
        if (k == 1) {
            return s;
        }
        int n = s.length();
        string ans;
        for (int i = 0; i < n; i++) {
            ans = max(ans, s.substr(i, n - max(k - 1, i)));
        }
        return ans;
    }
};

int main() {
    Solution solution;
    string s;
    int k;

    // Input string and k
    cin >> s;
    cin >> k;

    // Call answerString function
    string result = solution.answerString(s, k);

    // Print the result
    cout << result << endl;

    return 0;
}
