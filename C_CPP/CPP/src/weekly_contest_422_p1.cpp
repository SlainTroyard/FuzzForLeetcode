// Problem: Weekly Contest 422 Problem 1
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool isBalanced(std::string num) {
        int evenSum = 0;
        int oddSum = 0;

        for (size_t i = 0; i < num.length(); ++i) {
            int digit = num[i] - '0';
            if (i % 2 == 0) {
                evenSum += digit;} else {
                oddSum += digit;}
        }

        return evenSum == oddSum;}
};


int main() {
    string num;
    
    cin >> num;
    
    Solution solution;
    bool result = solution.isBalanced(num);
    
    if (result) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }
    
    return 0;
}
