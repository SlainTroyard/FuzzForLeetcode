// Problem: Weekly Contest 425 Problem 2
#include <iostream>
#include <string>
#include <unordered_map>
using namespace std;

class Solution {
public:
    bool isPossibleToRearrange(string s, string t, int k) {
        int n = s.size();
        unordered_map<string, int> mp;
        int size = n / k;
        for (int i = 0; i < n; i += size) mp[s.substr(i, size)]++;
        for (int i = 0; i < n; i += size) mp[t.substr(i, size)]--;
        
        for (auto &[key, value] : mp) if (value != 0) return false;
        
        return true;
    }
};

int main() {
    Solution solution;

    string s, t;
    int k;

    cin >> s;
    
    cin >> t;
    
    cin >> k;

    bool result = solution.isPossibleToRearrange(s, t, k);

    if (result) {
        cout << "true\n";
    } else {
        cout << "false\n";
    }

    return 0;
}
