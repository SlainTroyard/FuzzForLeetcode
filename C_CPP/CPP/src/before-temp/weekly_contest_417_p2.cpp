// Problem: Weekly Contest 417 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
    const int VOWEL_MASK = 1065233;

    long long f(string& word, int k) {
        long long ans = 0;
        int cnt1['u' - 'a' + 1]{};
        int size1 = 0; // 元音种类数
        int cnt2 = 0;
        int left = 0;
        for (char b : word) {
            b -= 'a';
            if (VOWEL_MASK >> b & 1) {
                if (cnt1[b]++ == 0) {
                    size1++;
                }
            } else {
                cnt2++;
            }
            while (size1 == 5 && cnt2 >= k) {
                char out = word[left] - 'a';
                if (VOWEL_MASK >> out & 1) {
                    if (--cnt1[out] == 0) {
                        size1--;
                    }
                } else {
                    cnt2--;
                }
                left++;
            }
            ans += left;
        }
        return ans;
    }

public:
    long long countOfSubstrings(string word, int k) {
        return f(word, k) - f(word, k + 1);
    }
};

int main() {
    int wordSize, k;
    cin >> wordSize;
    string word;
    cin >> word;
    cin >> k;
    Solution sol;
    cout << sol.countOfSubstrings(word, k) << endl;
    return 0;
}
