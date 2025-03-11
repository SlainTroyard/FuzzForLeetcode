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

    // Input the size of the array
    int n;
    cin >> n;

    // Input the elements of the array
    vector<int> A(n);
    for (int i = 0; i < n; ++i) {
        cin >> A[i];
    }

    // Call the constructTransformedArray function
    vector<int> transformedArray = solution.constructTransformedArray(A);

    // Output the transformed array
    for (int num : transformedArray) {
        cout << num << " ";
    }
    cout << endl;

    return 0;
}
