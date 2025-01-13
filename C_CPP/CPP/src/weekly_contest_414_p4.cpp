// Problem: Weekly Contest 414 Problem 4
#include <iostream>
#include <vector>
#include <cstring>
#include <climits>
#include <algorithm>
#include <functional>

using namespace std;

class Solution {
    // Directions for the knight's movement (8 possible directions)
    static constexpr int dirs[8][2] = {{2, 1}, {1, 2}, {-1, 2}, {-2, 1}, {-2, -1}, {-1, -2}, {1, -2}, {2, -1}};
public:
    // Function to calculate the maximum number of moves
    int maxMoves(int kx, int ky, vector<vector<int>>& positions) {
        int n = positions.size();
        int dis[n][50][50];
        memset(dis, -1, sizeof(dis));

        // Calculate the number of moves required for the knight to reach each position
        for (int i = 0; i < n; i++) {
            int px = positions[i][0], py = positions[i][1];
            dis[i][px][py] = 0;
            vector<pair<int, int>> q = {{px, py}};
            // Perform a BFS to calculate the minimum steps from each soldier to all other positions
            for (int step = 1; !q.empty(); step++) {
                vector<pair<int, int>> tmp;
                for (auto& [qx, qy] : q) {
                    for (auto& [dx, dy] : dirs) {
                        int x = qx + dx, y = qy + dy;
                        // Ensure the new position is within bounds and not yet visited
                        if (0 <= x && x < 50 && 0 <= y && y < 50 && dis[i][x][y] < 0) {
                            dis[i][x][y] = step;
                            tmp.emplace_back(x, y);
                        }
                    }
                }
                q = move(tmp);
            }
        }

        // Add the knight's position to the list of positions
        positions.push_back({kx, ky});
        vector<vector<int>> memo(n + 1, vector<int>(1 << n, -1)); // -1 indicates that the result is not yet computed
        int u = (1 << n) - 1;

        // Use std::function to allow recursion within the lambda
        std::function<int(int, int)> dfs = [&](int i, int mask) -> int {
            if (mask == 0) {
                return 0; // No more soldiers to move
            }
            int& res = memo[i][mask]; // Use reference to store the result
            if (res != -1) { // If the result has been calculated before, return it
                return res;
            }
            int x = positions[i][0], y = positions[i][1];
            if (__builtin_parity(u ^ mask) == 0) { // Alice's move
                for (int j = 0; j < n; j++) {
                    if (mask >> j & 1) {
                        res = max(res, dfs(j, mask ^ (1 << j)) + dis[j][x][y]);
                    }
                }
            } else { // Bob's move
                res = INT_MAX;
                for (int j = 0; j < n; j++) {
                    if (mask >> j & 1) {
                        res = min(res, dfs(j, mask ^ (1 << j)) + dis[j][x][y]);
                    }
                }
            }
            return res;
        };
        return dfs(n, u); // Start the DFS from the knight's position
    }
};

// Main function for I/O
int main() {
    Solution solution;
    int kx, ky, n;
    
    // Read knight's position (kx, ky) and number of soldiers
    cin >> kx >> ky >> n;
    vector<vector<int>> positions(n, vector<int>(2));
    
    // Read the positions of soldiers
    for (int i = 0; i < n; i++) {
        cin >> positions[i][0] >> positions[i][1];
    }
    
    // Call the maxMoves function and output the result
    int result = solution.maxMoves(kx, ky, positions);
    cout << result << endl;
    
    return 0;
}
