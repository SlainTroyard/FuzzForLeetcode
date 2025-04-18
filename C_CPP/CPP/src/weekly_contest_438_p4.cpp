// Problem: Weekly Contest 438 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <cmath>
using namespace std;

class Solution
{
public:
    int maxDistance(int side, vector<vector<int>> &points, int K)
    {
        int n = points.size();

        auto ord = [&](long long x, long long y)
        {
            long long s = side;
            if (y == 0)
                return x;
            else if (x == s)
                return s + y;
            else if (y == s)
                return s * 3 - x;
            else
                return s * 4 - y;
        };
        sort(points.begin(), points.end(), [&](vector<int> &a, vector<int> &b)
             { return ord(a[0], a[1]) < ord(b[0], b[1]); });

        auto dis = [&](int i, int j)
        {
            return abs(points[i][0] - points[j][0]) + abs(points[i][1] - points[j][1]);
        };

        auto check = [&](int lim)
        {
            vector<int> vec = {0};
            for (int i = 1; i < n && vec.size() < K; i++)
                if (dis(i, vec.back()) >= lim)
                    vec.push_back(i);
            if (vec.size() < K)
                return false;
            if (dis(vec[0], vec.back()) >= lim)
                return true;
            for (int i = 1; i < n && vec.back() < n * 2; i++)
            {
                vec[0] = i;
                for (int j = 1; j < K; j++)
                {
                    while (dis(vec[j] % n, vec[j - 1] % n) < lim)
                    {
                        vec[j]++;
                        if (vec[j] >= n * 2)
                            return false;
                    }
                }
                if (vec.back() < i + n && dis(i, vec.back() % n) >= lim)
                    return true;
            }
            return false;
        };

        int head = 1, tail = side;
        while (head < tail)
        {
            int mid = (head + tail + 1) >> 1;
            if (check(mid))
                head = mid;
            else
                tail = mid - 1;
        }
        return head;
    }
};

int main()
{
    
    int side, n, K;
    cin >> side >> n >> K;
    vector<vector<int>> points(n, vector<int>(2));
    for (int i = 0; i < n; i++)
    {
        cin >> points[i][0] >> points[i][1];
    }
    Solution sol;
    cout << sol.maxDistance(side, points, K) << endl;
    return 0;
}
