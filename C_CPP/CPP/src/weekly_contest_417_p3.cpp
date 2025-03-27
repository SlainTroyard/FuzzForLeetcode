// Problem: Weekly Contest 417 Problem 3
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    long long countOfSubstrings(string word, int k) {
        const int VOWEL_MASK = 1065233;
        long long ans = 0;
        int cnt_vowel1['u' - 'a' + 1]{}, cnt_vowel2['u' - 'a' + 1]{};
        int size_vowel1 = 0, size_vowel2 = 0; 
        int cnt_consonant1 = 0, cnt_consonant2 = 0;
        int left1 = 0, left2 = 0;
        for (int b : word) {
            b -= 'a';
            if (VOWEL_MASK >> b & 1) {
                if (cnt_vowel1[b]++ == 0) {
                    size_vowel1++;
                }
                if (cnt_vowel2[b]++ == 0) {
                    size_vowel2++;
                }
            } else {
                cnt_consonant1++;
                cnt_consonant2++;
            }

            while (size_vowel1 == 5 && cnt_consonant1 >= k) {
                char out = word[left1] - 'a';
                if (VOWEL_MASK >> out & 1) {
                    if (--cnt_vowel1[out] == 0) {
                        size_vowel1--;
                    }
                } else {
                    cnt_consonant1--;
                }
                left1++;
            }

            while (size_vowel2 == 5 && cnt_consonant2 > k) {
                char out = word[left2] - 'a';
                if (VOWEL_MASK >> out & 1) {
                    if (--cnt_vowel2[out] == 0) {
                        size_vowel2--;
                    }
                } else {
                    cnt_consonant2--;
                }
                left2++;
            }

            ans += left1 - left2;
        }
        return ans;
    }
};

int main() {
    int word_size, k;
    string word;
    cin >> word_size >> word >> k;
    long long ans = Solution().countOfSubstrings(word, k);
    cout << ans << endl;
    return 0;
}
