// Problem: Weekly Contest 413 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool checkTwoChessboards(string coordinate1, string coordinate2) {
        return (coordinate1[0] - coordinate2[0] + coordinate1[1] - coordinate2[1]) % 2 == 0;
    }
};

int main() {
    string coordinate1, coordinate2;
    cin >> coordinate1 >> coordinate2;
    Solution sol;
    if (sol.checkTwoChessboards(coordinate1, coordinate2)) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }
}
