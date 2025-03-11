// Problem: Weekly Contest 414 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <climits>
#include <ranges>
using namespace std;

class Solution {
public:
    int maxPossibleScore(vector<int>& start, int d) {
        sort(start.begin(), start.end());

        auto check = [&](int score) -> bool {
            long long x = LLONG_MIN;
            for (int s : start) {
                x = max(x + score, (long long)s);
                if (x > s + d) {
                    return false;
                }
            }
            return true;
        };

        int left = 0, right = (start.back() + d - start[0]) / (start.size() - 1) + 1;
        while (left + 1 < right) {
            int mid = left + (right - left) / 2;
            (check(mid) ? left : right) = mid;
        }
        return left;
    }
};

int main() {
    int startSize, d;
    cin >> startSize >> d;
    vector<int> start(startSize);
    for (int i = 0; i < startSize; i++) {
        cin >> start[i];
    }
    Solution sol;
    cout << sol.maxPossibleScore(start, d) << endl;
    return 0;
}
