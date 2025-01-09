// Problem: Weekly Contest 416 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <queue>
using namespace std;

class Solution {
public:
    long long minNumberOfSeconds(int mountainHeight, vector<int>& workerTimes) {
        priority_queue<tuple<long long, long long, int>, vector<tuple<long long, long long, int>>, greater<>> pq;
        for (int t : workerTimes) {
            pq.emplace(t, t, t);
        }
        long long ans = 0;
        while (mountainHeight--) {
            auto [nxt, delta, base] = pq.top(); pq.pop();
            ans = nxt;
            pq.emplace(nxt + delta + base, delta + base, base);
        }
        return ans;
    }
};

int main() {
    int mountainHeight, workerTimesSize;
    cin >> mountainHeight >> workerTimesSize;
    vector<int> workerTimes(workerTimesSize);
    for (int i = 0; i < workerTimesSize; i++) {
        cin >> workerTimes[i];
    }
    Solution s;
    cout << s.minNumberOfSeconds(mountainHeight, workerTimes) << endl;
    return 0;
}
