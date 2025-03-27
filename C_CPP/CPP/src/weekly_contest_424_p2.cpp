// Problem: Weekly Contest 424 Problem 2
#include <iostream>
#include <string>
#include <vector>
using namespace std;

class Solution {
public:
    bool isZeroArray(vector<int>& nums, vector<vector<int>>& q) {
        vector<int>v(nums.size()+1,0);
        for(int i=0;i<q.size();i++)
        {
            v[q[i][0]]++;
            v[q[i][1]+1]--;
        }
        for(int i=1;i<nums.size();i++)
        {
            v[i] +=v[i-1];
        }
        for(int i=0;i<nums.size();i++)
        {
            if(nums[i]-v[i]>0)
                return false;
            
        }
        return true;
        
        
    }
};

int main() {
    Solution sol;
    
    int n;
    cin >> n;
    
    vector<int> nums(n);
    for (int i = 0; i < n; i++) {
        cin >> nums[i];
    }
    
    int m;
    cin >> m;
    
    vector<vector<int>> queries(m, vector<int>(2));
    for (int i = 0; i < m; i++) {
        cin >> queries[i][0] >> queries[i][1];
    }
    
    if (sol.isZeroArray(nums, queries)) {
        cout << "true" << endl;
    } else {
        cout << "false" << endl;
    }
    
    return 0;
}
