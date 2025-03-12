// Problem: Weekly Contest 436 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<vector<int>> sortMatrix(vector<vector<int>>& grid) {
        int n = grid.size();
        for (int i = 0; i < n; i++) {
            vector<int> vec;
            for (int k = 0; i + k < n; k++) vec.push_back(grid[i + k][k]);
            sort(vec.begin(), vec.end(), greater<int>());
            for (int k = 0; i + k < n; k++) grid[i + k][k] = vec[k];
        }
        for (int j = 1; j < n; j++) {
            vector<int> vec;
            for (int k = 0; j + k < n; k++) vec.push_back(grid[k][j + k]);
            sort(vec.begin(), vec.end());
            for (int k = 0; j + k < n; k++) grid[k][j + k] = vec[k];
        }
        return grid;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n;
    cin >> n;
    vector<vector<int>> grid(n, vector<int>(n));
    for (int i = 0; i < n; i++) for (int j = 0; j < n; j++) cin >> grid[i][j];
    Solution solution;
    vector<vector<int>> result = solution.sortMatrix(grid);
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) cout << result[i][j] << " ";
        cout << endl;
    }
    return 0;
}
