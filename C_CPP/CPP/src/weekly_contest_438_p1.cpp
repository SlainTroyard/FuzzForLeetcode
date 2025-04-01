// Problem: Weekly Contest 438 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool hasSameDigits(string s) {
        while (s.size() > 2) {
            string t;
            for (int i = 0; i + 1 < s.size(); i++) {
                int x = s[i] - '0', y = s[i + 1] - '0';
                t.push_back((x + y) % 10 + '0');
            }
            s = t;
        }
        return s[0] == s[1];
    }
};
    

int main() {
    
    string s;
    cin >> s;
    Solution sol;
    cout << sol.hasSameDigits(s) << endl;
    return 0;
}
