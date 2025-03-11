// Problem: Weekly Contest 415 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <array>
#include <algorithm>
#include <climits>
using namespace std;

class Solution {
public:
    long long maxScore(vector<int>& a, vector<int>& b) {
        long long f[5]{};
        fill(f + 1, f + 5, LLONG_MIN / 2);
        for (int y : b) {
            for (int j = 3; j >= 0; j--) {
                f[j + 1] = max(f[j + 1], f[j] + (long long) a[j] * y);
            }
        }
        return f[4];
    }
};

int main() {
    int aSize, bSize;
    cin >> aSize >> bSize;
    vector<int> a(aSize), b(bSize);
    for (int i = 0; i < aSize; i++) {
        cin >> a[i];
    }
    for (int i = 0; i < bSize; i++) {
        cin >> b[i];
    }
    Solution solution;
    cout << solution.maxScore(a, b) << endl;
    return 0;
}
