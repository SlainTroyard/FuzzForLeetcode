// Problem: Weekly Contest 428 Problem 2
#include <iostream>
#include <vector>
#include <unordered_map>
#include <string>
#include <algorithm>
#include <iomanip> // for std::fixed and std::setprecision
using namespace std;

class Solution {
public:
    void bellman(unordered_map<string, double>& best, vector<vector<string>>& pairs, vector<double>& rates) {
        for (int relax_iteration = 0; relax_iteration < pairs.size(); ++relax_iteration) {
            for (int i = 0; i < pairs.size(); ++i) {
                best[pairs[i][1]] = max(best[pairs[i][1]], best[pairs[i][0]] * rates[i]);
                best[pairs[i][0]] = max(best[pairs[i][0]], best[pairs[i][1]] / rates[i]);
            }
        }
    }

    double maxAmount(string initialCurrency, vector<vector<string>>& pairs1, vector<double>& rates1, vector<vector<string>>& pairs2, vector<double>& rates2) {
        unordered_map<string, double> best;
        best[initialCurrency] = 1;
        bellman(best, pairs1, rates1);
        bellman(best, pairs2, rates2);
        return best[initialCurrency];
    }
};

int main() {
    Solution solution;

    string initialCurrency;
    int n1, n2;

    cin >> initialCurrency;
    cin >> n1;

    vector<vector<string>> pairs1(n1, vector<string>(2));
    vector<double> rates1(n1);

    for (int i = 0; i < n1; ++i) {
        cin >> pairs1[i][0] >> pairs1[i][1] >> rates1[i];
    }

    cin >> n2;
    vector<vector<string>> pairs2(n2, vector<string>(2));
    vector<double> rates2(n2);

    for (int i = 0; i < n2; ++i) {
        cin >> pairs2[i][0] >> pairs2[i][1] >> rates2[i];
    }

    double result = solution.maxAmount(initialCurrency, pairs1, rates1, pairs2, rates2);
    std::cout << std::fixed << std::setprecision(5) << result << std::endl;

    return 0;
}
