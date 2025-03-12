// Problem: Weekly Contest 435 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>
using namespace std;

class Solution {
public:
    int maxDistance(string s, int k) {
        int ans = 0, x = 0, y = 0;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] == 'N') y++;
            else if (s[i] == 'S') y--;
            else if (s[i] == 'E') x++;
            else x--;
            ans = max(ans, min(abs(x) + abs(y) + k * 2, i + 1));
        }
        return ans;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    string s;
    int k;
    cin >> s >> k;
    Solution sol;
    cout << sol.maxDistance(s, k) << endl;
    return 0;
}
