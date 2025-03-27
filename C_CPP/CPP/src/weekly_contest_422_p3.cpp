// Problem: Weekly Contest 422 Problem 3
#include <iostream>
#include <string>
#include <vector>
#include <queue>
#include <climits>
#include <algorithm>
using namespace std;

class Solution {
public:
    int minTimeToReach(vector<vector<int>>& moveTime) {
        if (moveTime.empty() || moveTime[0].empty()) {
            return 0;
        }
        
        int rows = moveTime.size();
        int cols = moveTime[0].size();
        vector<vector<int>> vis(rows,vector<int>(cols,0));
        priority_queue<pair<int,pair<int,int>>,vector<pair<int,pair<int,int>>>,greater<pair<int,pair<int,int>>>> pq;

        pq.push({0,{0,0}});
        int drow[] = {-1,0,1,0};
        int dcol[] = {0,1,0,-1};
        int time = 0;
        vis[0][0] = 1;
        while(!pq.empty()){
            time = pq.top().first;
            int r = pq.top().second.first;
            int c = pq.top().second.second;
            pq.pop();
            if(r == rows-1 && c == cols-1){
                return time;
            }
            for(int i = 0;i<4;i++){
                int nrow = r+drow[i];
                int ncol = c+dcol[i];
                if(nrow>=0 && nrow<rows && ncol>=0 && ncol<cols && vis[nrow][ncol] == 0){
                    int nextCost = 2 - (nrow+ncol)%2;
                    if(moveTime[nrow][ncol] >= time){
                        pq.push({moveTime[nrow][ncol]+nextCost,{nrow,ncol}});
                    }
                    else{
                        pq.push({time+nextCost,{nrow,ncol}});
                    }
                    vis[nrow][ncol] = 1;
                }
            }
        }
        return time;
    }
};


int main() {
    Solution sol;
    
    int rows, cols;
    cin >> rows >> cols;
    
    
    vector<vector<int>> moveTime(rows, vector<int>(cols));
    for(int i = 0; i < rows; i++) {
        for(int j = 0; j < cols; j++) {
            cin >> moveTime[i][j];
        }
    }
    
    int result = sol.minTimeToReach(moveTime);
    cout << result << endl;
    
    return 0;
}
