// Problem: Weekly Contest 437 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <functional> 
using namespace std;

class Solution {
public:
int lenOfVDiagonal(vector<vector<int>>& grid) {
    const int DIRS[4][2] = {{1,1}, {1,-1}, {-1,-1}, {-1,1}};
    int m = grid.size(), n = grid[0].size();
    
    vector<vector<vector<vector<int>>>> memo(
        m, vector<vector<vector<int>>>(
            n, vector<vector<int>>(
                4, vector<int>(2, 0)
            )
        )
    );

    function<int(int, int, int, bool, int)> dfs;
    dfs = [&](int i, int j, int k, bool can_turn, int target) -> int {
        if (i < 0 || i >= m || j < 0 || j >= n) return 0;
        
        int ni = i + DIRS[k][0];
        int nj = j + DIRS[k][1];
        
        if (ni < 0 || nj < 0 || ni >= m || nj >= n || grid[ni][nj] != target)
            return 0;
            
        int& res = memo[ni][nj][k][can_turn];
        if (res != 0) return res;
        
        res = dfs(ni, nj, k, can_turn, 2 - target);
        
        if (can_turn) {
            int nk = (k + 1) % 4;
            res = max(res, dfs(ni, nj, nk, false, 2 - target));
        }
        
        return ++res;
    };

    int ans = 0;
    for (int i = 0; i < m; ++i) {
        for (int j = 0; j < n; ++j) {
            if (grid[i][j] == 1) {
                for (int k = 0; k < 4; ++k) {
                    ans = max(ans, dfs(i, j, k, true, 2) + 1);
                }
            }
        }
    }
    return ans;
}
};

int main() {
    
    int n, m;
    cin >> n >> m;
    vector<vector<int>> grid(n, vector<int>(m));
    for (int i = 0; i < n; i++) for (int j = 0; j < m; j++) cin >> grid[i][j];
    Solution sol;
    cout << sol.lenOfVDiagonal(grid) << endl;
    return 0;
}
