// Problem: Weekly Contest 435 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <climits>
using namespace std;

class Solution {
public:
    int maxDifference(string s) {
        int cnt[26]{};
        for (char b : s) {
            cnt[b - 'a']++;
        }

        int max1 = 0, min0 = INT_MAX;
        for (int c : cnt) {
            if (c % 2) {
                max1 = max(max1, c);
            } else if (c) {
                min0 = min(min0, c);
            }
        }
        return max1 - min0;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    string s;
    cin >> s;
    Solution sol;
    cout << sol.maxDifference(s) << endl;
    return 0;
}
