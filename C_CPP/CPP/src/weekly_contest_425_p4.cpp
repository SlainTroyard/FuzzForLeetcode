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

        for (auto [v, w] : adj[u]) {
            if (v == parent) 
                continue;  

            dfs(adj, v, u, k, dp);  

            differences.push_back(w + dp[v][1] - dp[v][0]);

            sumWeights += dp[v][0];
        }

        sort(rbegin(differences), rend(differences));

        dp[u][0] = sumWeights;
        for (int i = 0; i < min(k, (int)differences.size()); ++i) {
            if (differences[i] > 0) 
                dp[u][0] += differences[i];
        }

        dp[u][1] = sumWeights;
        for (int i = 0; i < min(k - 1, (int)differences.size()); ++i) {
            if (differences[i] > 0) 
                dp[u][1] += differences[i];
        }
    }

public:
    long long maximizeSumOfWeights(vector<vector<int>>& edges, int k) {
        int n = edges.size() + 1;  
        vector<vector<pair<int, int>>> adj(n);

        for (auto& e : edges) {
            int u = e[0], v = e[1], w = e[2];
            adj[u].emplace_back(v, w);
            adj[v].emplace_back(u, w);
        }

        vector<vector<long long>> dp(n, vector<long long>(2, -1));

        dfs(adj, 0, -1, k, dp);

        return dp[0][0];
    }
};

int main() {
    int n, k;
    
    cin >> n >> k;

    vector<vector<int>> edges(n);
    
    for (int i = 0; i < n; i++) {
        int u, v, w;
        cin >> u >> v >> w;
        edges[i] = {u, v, w};
    }

    Solution solution;

    long long result = solution.maximizeSumOfWeights(edges, k);
    cout << result << endl;

    return 0;
}