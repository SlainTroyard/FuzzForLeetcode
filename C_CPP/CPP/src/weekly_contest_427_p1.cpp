// Problem: Weekly Contest 427 Problem 1
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> constructTransformedArray(vector<int>& A) {
        int n = A.size();
        vector<int> res(n);
        for (int i = 0; i < n; ++i) {
            res[i] = A[(i + A[i] % n + n) % n];
        }
        return res;
    }
};

int main() {
    Solution solution;

    int n;
    cin >> n;

    vector<int> A(n);
    for (int i = 0; i < n; ++i) {
        cin >> A[i];
    }

    vector<int> transformedArray = solution.constructTransformedArray(A);

    for (int num : transformedArray) {
        cout << num << " ";
    }
    cout << endl;

    return 0;
}
