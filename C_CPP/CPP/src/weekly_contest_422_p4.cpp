// Problem: Weekly Contest 422 Problem 4
#include <iostream>
#include <string>
#include <vector>
#include <cstring> 
using namespace std;

class Solution {
    const static int M=1e9+7;
    int n;
    int cnt[10], left_s[10], left_c[10];
    long dp[10][721][81];  
    long r1[11];
    long dfs(int i, int s, int c) {
        if (s==0 and c==0) return r1[i];
        if (i==10) return 0;
        if (s>left_s[i] or c>left_c[i]) return 0;
        if (dp[i][s][c]!=-1) return dp[i][s][c];
        long res=0;
        int a=s;
        for (int x=0, y=cnt[i]; x<=cnt[i] and a>=0 and c>=x; ++x, --y, a-=i) {
            if (y>left_c[i]-c) continue;
            long b=(dfs(i+1,a,c-x)*cb[c][x])%M;
            res=(res+b*cb[left_c[i]-c][y])%M;
        }
        return dp[i][s][c]=res;
    }
    int cb[81][81];
    void pascal() {
        memset(cb,0,sizeof(cb));
        cb[0][0]=1;
        for (int i=1; i<=80; ++i) {
            cb[i][0]=1;
            cb[i][i]=1;
            for (int j=1; j<i; ++j)
                cb[i][j]=(cb[i-1][j-1]+cb[i-1][j])%M;
        }
    }
public:
    int countBalancedPermutations(string num) {
        int s=0;
        memset(cnt,0,sizeof(cnt));
        for (char c : num) {
            s+=(c-'0');
            ++cnt[c-'0'];
        }
        if (s&1) return 0;
        pascal();
        r1[10]=1;
        int ls=0, lc=0;
        for (int i=9; i>-1; --i) {
            ls+=i*cnt[i];
            lc+=cnt[i];
            left_s[i]=ls;
            left_c[i]=lc;
            r1[i]=(r1[i+1]*cb[left_c[i]][cnt[i]])%M;
        }
        n=int(num.size());
        memset(dp,-1,sizeof(dp));
        return dfs(0,s>>1,n>>1);
    }
};

int main() {
    Solution sol;
    string num;
    
    cin >> num;
    
    const int MAX_LENGTH = 80;
    if (num.length() > MAX_LENGTH) {
        cout << "Input too long, maximum allowed length is " << MAX_LENGTH << endl;
        return 1;
    }
    
    int result = sol.countBalancedPermutations(num);
    cout << result << endl;
    return 0;
}
