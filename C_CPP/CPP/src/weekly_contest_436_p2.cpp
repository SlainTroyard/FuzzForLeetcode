// Problem: Weekly Contest 436 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<int> assignElements(vector<int>& groups, vector<int>& elements) {
        int mx = *max_element(elements.begin(), elements.end());
        vector<int> target(mx + 1, -1);
        for (int i = 0; i < elements.size(); i++) {
            int x = elements[i];
            if (x > mx || target[x] >= 0) {
                continue;
            }
            for (int y = x; y <= mx; y += x) {
                if (target[y] < 0) {
                    target[y] = i;
                }
            }
        }

        for (int& x : groups) {
            x = target[x];
        }
        return groups;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n, m;
    cin >> n >> m;
    vector<int> groups(n), elements(m);
    for (int i = 0; i < n; i++) {
        cin >> groups[i];
    }
    for (int i = 0; i < m; i++) {
        cin >> elements[i];
    }
    Solution solution;
    vector<int> result = solution.assignElements(groups, elements);
    for (int i = 0; i < n; i++) {
        cout << result[i] << " ";
    }
    cout << endl;
    return 0;
}
