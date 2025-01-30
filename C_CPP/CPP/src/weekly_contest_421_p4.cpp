// Problem: Weekly Contest 421 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <array>
#include <algorithm>
#include <numeric>
using namespace std;

class Solution {
    static constexpr int MOD = 1'000'000'007;
    static constexpr int SIZE = 26;

    using Matrix = array<array<int, SIZE>, SIZE>;

    // 返回矩阵 a 和矩阵 b 相乘的结果
    Matrix mul(Matrix& a, Matrix& b) {
        Matrix c{};
        for (int i = 0; i < SIZE; i++) {
            for (int k = 0; k < SIZE; k++) {
                if (a[i][k] == 0) {
                    continue;
                }
                for (int j = 0; j < SIZE; j++) {
                    c[i][j] = (c[i][j] + (long long) a[i][k] * b[k][j]) % MOD;
                }
            }
        }
        return c;
    }

    // 返回 n 个矩阵 a 相乘的结果
    Matrix pow(Matrix a, int n) {
        Matrix res = {};
        for (int i = 0; i < SIZE; i++) {
            res[i][i] = 1; // 单位矩阵
        }
        while (n) {
            if (n & 1) {
                res = mul(res, a);
            }
            a = mul(a, a);
            n >>= 1;
        }
        return res;
    }

public:
    int lengthAfterTransformations(string s, int t, vector<int>& nums) {
        Matrix m{};
        for (int i = 0; i < SIZE; i++) {
            for (int j = i + 1; j <= i + nums[i]; j++) {
                m[i][j % SIZE] = 1;
            }
        }
        m = pow(m, t);

        int cnt[SIZE]{};
        for (char c : s) {
            cnt[c - 'a']++;
        }

        long long ans = 0;
        for (int i = 0; i < SIZE; i++) {
            // m 第 i 行的和就是 f[t][i]
            ans += std::accumulate(m[i].begin(), m[i].end(), 0LL) * cnt[i];
        }
        return ans % MOD;
    }
};

int main() {
    int s_len;
    string s;
    int t;
    cin >> s_len >> s >> t;
    vector<int> nums(26);
    for (int i = 0; i < 26; i++) {
        cin >> nums[i];
    }
    Solution solution;
    cout << solution.lengthAfterTransformations(s, t, nums) << endl;
    return 0;
}
