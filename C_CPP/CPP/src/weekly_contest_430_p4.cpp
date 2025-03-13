#include <iostream>
#include <vector>
using namespace std;

const int MOD = 1000000007;

class Solution {
public:
    long long exponent(long long base, long long exp, long long mod) {
        long long result = 1;
        while (exp > 0) {
            if (exp % 2 == 1) {
                result = (result * base) % mod;
            }
            base = (base * base) % mod;
            exp /= 2;
        }
        return result;
    }

    long long modInverse(long long x, long long mod) {
        return exponent(x, mod - 2, mod);
    }

    long long nCr(int n, int r, vector<long long>& fact) {
        return fact[n] * modInverse(fact[r], MOD) % MOD * modInverse(fact[n - r], MOD) % MOD;
    }

    int countGoodArrays(int n, int m, int k) {
        vector<long long> fact(n + 1, 1);
        for (int i = 2; i <= n; ++i) 
            fact[i] = fact[i - 1] * i % MOD;

        long long result = nCr(n - 1, k, fact);
        result = result * m % MOD;
        result = result * exponent(m - 1, n - k - 1, MOD) % MOD;
        return result;
    }
};

int main() {
    int n, m, k;
    cin >> n >> m >> k;

    Solution solution;
    int result = solution.countGoodArrays(n, m, k);

    cout << result << endl;

    return 0;
}
