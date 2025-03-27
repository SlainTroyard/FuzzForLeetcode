// Problem: Weekly Contest 437 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    long long maxWeight(vector<int>& pizzas) {
        sort(pizzas.begin(), pizzas.end(), greater<int>());
        int days = pizzas.size() / 4;
        int odd = (days + 1) / 2;
        long long ans = 0;
        for (int i = 0; i < odd; i++) {
            ans += pizzas[i];
        }
        for (int i = 0; i < days / 2; i++) {
            ans += pizzas[odd + i * 2 + 1];
        }
        return ans;
    }
};

int main() {
    int n;
    cin >> n;
    vector<int> pizzas(n);
    for (int i = 0; i < n; i++) {
        cin >> pizzas[i];
    }
    Solution solution;
    cout << solution.maxWeight(pizzas) << endl;
    return 0;
}
