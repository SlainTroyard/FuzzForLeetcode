// Problem: Weekly Contest 436 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long maxScore(vector<int>& points, int m) {
        auto check = [&](long long low) -> bool {
            int n = points.size(), rem = m, pre = 0;
            for (int i = 0; i < n; i++) {
                int k = (low - 1) / points[i] + 1 - pre;
                if (i == n - 1 && k <= 0) {
                    break;
                }
                k = max(k, 1);
                rem -= k * 2 - 1;
                if (rem < 0) {
                    return false;
                }
                pre = k - 1;
            }
            return true;
        };

        long long left = 0;
        long long right = 1LL * (m + 1) / 2 * *min_element(points.begin(), points.end()) + 1;
        while (left + 1 < right) {
            long long mid = left + (right - left) / 2;
            (check(mid) ? left : right) = mid;
        }
        return left;
    }
};

int main() {
    // TODO: Add the base I/O interface here
    int n, m;
    cin >> n >> m;
    vector<int> points(n);
    for (int i = 0; i < n; i++) {
        cin >> points[i];
    }
    Solution solution;
    cout << solution.maxScore(points, m) << endl;
    return 0;
}
