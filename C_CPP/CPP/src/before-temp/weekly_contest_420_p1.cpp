// Problem: Weekly Contest 420 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    vector<string> stringSequence(string target) {
        vector<string> ans;
        string s;
        for (int c : target) {
            s += 'a'; // 占位
            for (char j = 'a'; j <= c; j++) {
                s.back() = j;
                ans.push_back(s);
            }
        }
        return ans;
    }
};

int main() {
    string target;
    cin >> target;
    Solution solution;
    vector<string> ans = solution.stringSequence(target);
    for (string s : ans) {
        cout << s << " ";
    }
    cout << endl;
    return 0;
}
