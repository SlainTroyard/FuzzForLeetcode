// Problem: Weekly Contest 417 Problem 4
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    char kthCharacter(long long k, vector<int>& operations) {
        k--;
        int inc = 0;
        for (int i = __lg(k); i >= 0; i--) {
            if (k >> i & 1) {
                inc += operations[i];
            }
        }
        return 'a' + inc % 26;
    }
};

int main() {
    long long k;
    int operationSize;
    cin >> k >> operationSize;
    vector<int> operations(operationSize);
    for (int i = 0; i < operationSize; i++) {
        cin >> operations[i];
    }
    Solution s;
    cout << s.kthCharacter(k, operations) << endl;
    return 0;
}
