// Problem: Weekly Contest 432 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    vector<int> zigzagTraversal(vector<vector<int>>& grid) {
        int n = grid.size(), m = grid[0].size();
        vector<int> vec;
        for (int i = 0, cnt = 0; i < n; i++) {
            if (i & 1) {
                for (int j = m - 1; j >= 0; j--, cnt ^= 1) if (cnt == 0) vec.push_back(grid[i][j]);
            } else {
                for (int j = 0; j < m; j++, cnt ^= 1) if (cnt == 0) vec.push_back(grid[i][j]);
            }
        }
        return vec;
    }
};
    

int main() {
    
    int gridSize, gridColSize;
    cin >> gridSize >> gridColSize;
    vector<vector<int>> grid(gridSize, vector<int>(gridColSize));
    for (int i = 0; i < gridSize; i ++) {
        for (int j = 0; j < gridColSize; j ++) {
            cin >> grid[i][j];
        }
    }
    Solution solution;
    vector<int> ans = solution.zigzagTraversal(grid);
    for (int i = 0; i < ans.size(); i ++) {
        cout << ans[i] << " ";
    }
    return 0;
}
