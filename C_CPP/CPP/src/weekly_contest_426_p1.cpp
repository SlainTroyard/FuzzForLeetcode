// Problem: Weekly Contest 426 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <cmath>
using namespace std;

#include <iostream>
#include <cmath>
using namespace std;

class Solution {
public:
    int smallestNumber(int n) {
        int b = log2(n) + 1;           
        return (1 << b) - 1;          
    }
};

int main() {
    Solution solution;
    int n;

    cin >> n;

    int result = solution.smallestNumber(n);

    cout << result << endl;

    return 0;
}
