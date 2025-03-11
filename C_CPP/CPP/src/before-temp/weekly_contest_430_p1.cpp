// Problem: Weekly Contest 430 Problem 1
#include <string>
#include <vector>
#include <iostream>
using namespace std;

class Solution {
public:
    int minimumOperations(vector<vector<int>>& grid) {
        vector<vector<int>> calGrid = grid;
        int ans = 0;
        int m = calGrid.size();
        int n = calGrid[0].size();
        if (m == 1)
            return 0;
        for (int i = 0; i < n; i++) {
            for (int j = 1; j < m; j++) {
                if (calGrid[j][i] <= calGrid[j-1][i]) {
                    ans += calGrid[j-1][i] + 1 - calGrid[j][i];
                    calGrid[j][i] = calGrid[j-1][i] + 1;
                }
            }
        }
        return ans;
    }
};

int main() {
    Solution solution;

    int m, n;
    cin >> m >> n; // m rows and n columns

    vector<vector<int>> grid(m, vector<int>(n));
    for (int i = 0; i < m; i++) {
        for (int j = 0; j < n; j++) {
            cin >> grid[i][j]; // input matrix element
        }
    }

    cout << solution.minimumOperations(grid) << endl; // result
    return 0;
}

