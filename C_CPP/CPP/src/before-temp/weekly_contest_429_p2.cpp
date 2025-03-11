// Problem: Weekly Contest 429 Problem 2
#include <iostream>
#include <vector>
#include <set>
#include <algorithm>
#include <climits>
using namespace std;

class Solution {
public:
    int maxDistinctElements(vector<int>& arr, int diff) {
        int prev = INT_MIN;
        set<int> c;
        sort(arr.begin(), arr.end());
        for (int i = 0; i < arr.size(); i++) {
            int x = max(prev + 1, arr[i] - diff);

            if (x <= arr[i] + diff) {
                c.insert(x);
                prev = x;
            }
        }
        return c.size();
    }
};

int main() {
    Solution solution;
    int n, diff;

    // Input array size and difference
    cin >> n;
    cin >> diff;

    // Input array elements
    vector<int> arr(n);
    for (int i = 0; i < n; i++) {
        cin >> arr[i];
    }

    // Compute the result
    int result = solution.maxDistinctElements(arr, diff);

    // Output the result
    cout << result << endl;

    return 0;
}
