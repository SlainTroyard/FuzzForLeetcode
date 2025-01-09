// Problem: Weekly Contest 416 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <unordered_set>
using namespace std;

class Solution {
public:
    bool reportSpam(vector<string>& message, vector<string>& bannedWords) {
        unordered_set<string> banned(bannedWords.begin(), bannedWords.end());
        int cnt = 0;
        for (auto& s : message) {
            if (banned.find(s) != banned.end() && ++cnt > 1) {
                return true;
            }
        }
        return false;
    }
};

int main() {
    int messageSize, bannedWordsSize;
    cin >> messageSize;
    vector<string> message(messageSize);
    for (int i = 0; i < messageSize; i++) {
        cin >> message[i];
    }
    cin >> bannedWordsSize;
    vector<string> bannedWords(bannedWordsSize);
    for (int i = 0; i < bannedWordsSize; i++) {
        cin >> bannedWords[i];
    }
    Solution s;
    if (s.reportSpam(message, bannedWords)) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }
    return 0;
}
