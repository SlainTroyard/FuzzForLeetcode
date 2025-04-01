// Problem: Weekly Contest 438 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <numeric>
#include <functional>
using namespace std;

class Solution
{
public:
    static void exgcd(int a, int b, int &x, int &y)
    {
        if (b == 0)
        {
            x = 1;
            y = 0;
            return;
        }
        exgcd(b, a % b, y, x);
        y -= a / b * x;
    }

    bool hasSameDigits(string s)
    {
        int n = s.size();

        int P2[n + 1], P5[n + 1];
        P2[0] = P5[0] = 1;
        for (int i = 1; i <= n; i++)
        {
            P2[i] = (P2[i - 1] * 2) % 10;
            P5[i] = (P5[i - 1] * 5) % 10;
        }

        auto calc = [&](int l, int r)
        {
            int len = r - l;
            int c = 1, two = 0, five = 0, sum = 0;
            for (int i = l, j = 0;; i++, j++)
            {
                sum = (sum + (s[i] - '0') * P2[two] * P5[five] * c) % 10;
                if (i == r)
                    break;
                int t = len - j;
                while (t % 2 == 0)
                {
                    two++;
                    t /= 2;
                }
                while (t % 5 == 0)
                {
                    five++;
                    t /= 5;
                }
                c = (c * t) % 10;
                t = j + 1;
                while (t % 2 == 0)
                {
                    two--;
                    t /= 2;
                }
                while (t % 5 == 0)
                {
                    five--;
                    t /= 5;
                }
                int x, y;
                Solution::exgcd(t, 10, x, y);
                c = (c * (x % 10 + 10)) % 10;
            }
            return sum;
        };

        return calc(0, n - 2) == calc(1, n - 1);
    }
};

int main()
{
    
    string s;
    cin >> s;
    Solution sol;
    cout << sol.hasSameDigits(s) << endl;
    return 0;
}
