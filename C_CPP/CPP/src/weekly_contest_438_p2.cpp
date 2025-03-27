// Problem: Weekly Contest 438 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <queue>
#include <array>
using namespace std;

class Solution
{
public:
    long long maxSum(vector<vector<int>> &grid, vector<int> &limits, int k)
    {
        int n = grid.size(), m = grid[0].size();

        priority_queue<array<int, 3>> pq;
        for (int i = 0; i < n; i++)
        {
            sort(grid[i].begin(), grid[i].end(), greater<int>());
            pq.push({grid[i][0], i, 0});
        }

        long long ans = 0;
        while (k > 0 && !pq.empty())
        {
            auto arr = pq.top();
            pq.pop();
            int r = arr[1], c = arr[2];
            if (c >= limits[r])
                continue;
            ans += arr[0];
            k--;
            if (c + 1 < m)
                pq.push({grid[r][c + 1], r, c + 1});
        }
        return ans;
    }
};

int main()
{
    int n, m, k;
    cin >> n >> m >> k;
    vector<vector<int>> grid(n, vector<int>(m));
    for (int i = 0; i < n; i++)
        for (int j = 0; j < m; j++)
            cin >> grid[i][j];
    vector<int> limits(n);
    for (int i = 0; i < n; i++)
        cin >> limits[i];
    Solution sol;
    cout << sol.maxSum(grid, limits, k) << endl;
    return 0;
}
