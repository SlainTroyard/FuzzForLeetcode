// Problem: Weekly Contest 419 Problem 1
#include <iostream>
#include <string>
#include <vector>
#include <unordered_map>
#include <algorithm>
using namespace std;

class Solution {
public:
    vector<int> findXSum(vector<int>& nums, int k, int x) {
        unordered_map<int, int> mp;
        vector<int> res;
        for(int l = 0, r = 0; r < nums.size(); ++r) {
            mp[nums[r]]++;
            if(r - l + 1 == k) {
                vector<pair<int, int>> vec(mp.begin(), mp.end());// num - cnt
                sort(vec.begin(), vec.end(), [&](pair<int, int> lhs, pair<int, int> rhs) {
                    if(lhs.second == rhs.second) {
                        return lhs.first > rhs.first; // 出现次数相同，num大的放前边
                    }
                    return lhs.second > rhs.second; // 出现次数不同，cnt大的在前边
                });

                int sum = 0;
                for(int i = 0; i < x && i < vec.size(); ++i) {
                    sum += vec[i].first * vec[i].second;
                }
                res.push_back(sum);
                
                mp[nums[l]]--;
                if(mp[nums[l]] == 0) {
                    mp.erase(nums[l]);
                }
                ++l;
            }
        }
        return res;
    }
};

int main() {
    int k, x;
    cin >> k >> x;
    int numsSize;
    cin >> numsSize;
    vector<int> nums(numsSize);
    for(int i = 0; i < numsSize; ++i) {
        cin >> nums[i];
    }
    Solution s;
    vector<int> res = s.findXSum(nums, k, x);
    for(int i = 0; i < res.size(); ++i) {
        cout << res[i] << " ";
    }
    cout << endl;
    return 0;
}
