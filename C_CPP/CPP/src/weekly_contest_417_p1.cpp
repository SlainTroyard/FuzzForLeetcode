// Problem: Weekly Contest 417 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    char kthCharacter(long long k) {
        return 'a' + __builtin_popcount(k - 1);
    }
};

int main() {
    int k = 0;
    cin >> k;
    Solution s;
    cout << s.kthCharacter(k) << endl;
    return 0;
}
