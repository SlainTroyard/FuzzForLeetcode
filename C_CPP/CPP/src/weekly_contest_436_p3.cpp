// Problem: Weekly Contest 436 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <array>

using namespace std;

class Solution {
public:
    long long countSubstrings(string s) {
        long long ans = 0;
        array<array<int, 9>, 10> f{};
        for (char c : s) {
            int d = c - '0';
            for (int m = 1; m < 10; ++m) {
                array<int, 9> nf{};
                nf[d % m] = 1;
                for (int rem = 0; rem < m; ++rem) {
                    nf[(rem * 10 + d) % m] += f[m][rem];
                }
                f[m] = nf;
            }
            ans += f[d][0];
        }
        return ans;
    }
};

int main() {
    string s;
    cin >> s;
    Solution sol;
    cout << sol.countSubstrings(s) << endl;
    return 0;
}
