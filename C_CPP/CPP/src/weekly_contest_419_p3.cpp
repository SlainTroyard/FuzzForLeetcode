// Problem: Weekly Contest 419 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <array>
using namespace std;

class Solution {
public:
    int countWinningSequences(string s) {
        const int MOD = 1'000'000'007;
        int mp[128];
        mp['F'] = 0;
        mp['W'] = 1;
        mp['E'] = 2;

        int n = s.size();
        vector<vector<array<int, 3>>> f(n + 1, vector<array<int, 3>>(n * 2 + 1));
        for (int j = n + 1; j <= n * 2; j++) {
            f[0][j] = {1, 1, 1};
        }

        int pow2 = 1;
        for (int i = 0; i < n; i++) {
            int x = mp[s[i]];
            pow2 = pow2 * 2 % MOD;
            for (int j = -i; j < n - i; j++) {
                for (int ban = 0; ban < 3; ban++) {
                    if (j > i + 1) {
                        f[i + 1][j + n][ban] = pow2;
                        continue;
                    }
                    int& res = f[i + 1][j + n][ban]; 
                    for (int k = 0; k < 3; k++) {
                        if (i == n - 1 || k != ban) {
                            int score = (k - x + 3) % 3;
                            if (score == 2) {
                                score = -1;
                            }
                            res = (res + f[i][j + score + n][k]) % MOD;
                        }
                    }
                }
            }
        }
        return f[n][n][0];
    }
};

int main() {
    int count = 0;
    cin >> count;
    string s;
    cin >> s;
    Solution solution;
    cout << solution.countWinningSequences(s) << endl;
    return 0;
}
