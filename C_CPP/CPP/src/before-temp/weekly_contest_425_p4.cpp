// Problem: Weekly Contest 425 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
    void dfs(vector<vector<pair<int, int>>>& adj, int u, int parent, int k, vector<vector<long long>>& dp) {
        vector<long long> differences;
        long long sumWeights = 0;

        // Explore all neighbors of the current node
        for (auto [v, w] : adj[u]) {
            if (v == parent) 
                continue;  // Skip the parent node to avoid backtracking

            dfs(adj, v, u, k, dp);  // Recursively process the child node

            // Difference in weight contribution between keeping and removing the edge
            differences.push_back(w + dp[v][1] - dp[v][0]);

            // Accumulate the base case sum (dp[v][0]) for the subtree rooted at v
            sumWeights += dp[v][0];
        }

        // Sort differences in descending order to prioritize edges with higher contributions
        sort(rbegin(differences), rend(differences));

        // Case 1: Select at most `k` edges for the current node
        dp[u][0] = sumWeights;
        for (int i = 0; i < min(k, (int)differences.size()); ++i) {
            if (differences[i] > 0) 
                dp[u][0] += differences[i];
        }

        // Case 2: Select at most `k-1` edges for the current node (for parent inclusion)
        dp[u][1] = sumWeights;
        for (int i = 0; i < min(k - 1, (int)differences.size()); ++i) {
            if (differences[i] > 0) 
                dp[u][1] += differences[i];
        }
    }

public:
    long long maximizeSumOfWeights(vector<vector<int>>& edges, int k) {
        int n = edges.size() + 1;  // Total nodes in the tree
        vector<vector<pair<int, int>>> adj(n);

        // Build adjacency list
        for (auto& e : edges) {
            int u = e[0], v = e[1], w = e[2];
            adj[u].emplace_back(v, w);
            adj[v].emplace_back(u, w);
        }

        // dp[u][0]: Maximum sum when at most `k` edges are selected for node `u`
        // dp[u][1]: Maximum sum when at most `k-1` edges are selected for node `u` (to consider parent inclusion)
        vector<vector<long long>> dp(n, vector<long long>(2, -1));

        // Start DFS from the root node (0)
        dfs(adj, 0, -1, k, dp);

        // The result is the maximum sum when starting from the root node with at most `k` edges
        return dp[0][0];
    }
};

int main() {
    int n, k;
    
    // Read the number of edges and the allowed number of edges to select
    cin >> n >> k;

    vector<vector<int>> edges(n);
    
    // Read the edges, each as a triplet (u, v, w)
    for (int i = 0; i < n; i++) {
        int u, v, w;
        cin >> u >> v >> w;
        edges[i] = {u, v, w};
    }

    // Create an instance of the Solution class
    Solution solution;

    // Call the maximizeSumOfWeights function and print the result
    long long result = solution.maximizeSumOfWeights(edges, k);
    cout << result << endl;

    return 0;
}