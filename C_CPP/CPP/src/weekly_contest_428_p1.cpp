// Problem: Weekly Contest 428 Problem 1

#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    int buttonWithLongestTime(vector<vector<int>>& events) {
        int idx = events[0][0], max_diff = events[0][1];
        for (int i = 1; i < events.size(); i++) {
            auto& p = events[i - 1];
            auto& q = events[i];
            int d = q[1] - p[1];
            if (d > max_diff || (d == max_diff && q[0] < idx)) {
                idx = q[0];
                max_diff = d;
            }
        }
        return idx;
    }
};

int main() {
    int n;
    cin >> n; 
    vector<vector<int>> events(n, vector<int>(2));
    
    for (int i = 0; i < n; ++i) {
        cin >> events[i][0] >> events[i][1]; 
    }
    
    Solution solution;
    int result = solution.buttonWithLongestTime(events);
    cout << result << endl; 
    
    return 0;
}
