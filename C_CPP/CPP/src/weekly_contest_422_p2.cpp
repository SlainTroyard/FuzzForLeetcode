// Problem: Weekly Contest 422 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <queue>
#include <climits>
#include <tuple>
#include <algorithm>
#include <sstream>
using namespace std;


class Solution {
public:
    int minTimeToReach(std::vector<std::vector<int>>& moveTime) {
        if (moveTime.empty() || moveTime[0].empty()) {
            return 0;
        }
        
        int rows = moveTime.size();
        int cols = moveTime[0].size();
        std::priority_queue<std::tuple<int, int, int>, std::vector<std::tuple<int, int, int>>, std::greater<std::tuple<int, int, int>>> minHeap;
        std::vector<std::vector<int>> time(rows, std::vector<int>(cols, INT_MAX));
        
        minHeap.emplace(0, 0, 0); 
        time[0][0] = 0;

        std::vector<std::pair<int, int>> directions = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

        while (!minHeap.empty()) {
            auto [currentTime, x, y] = minHeap.top();
            minHeap.pop();

            if (x == rows - 1 && y == cols - 1) {
                return currentTime;
            }

            for (const auto& [dx, dy] : directions) {
                int newX = x + dx, newY = y + dy;

                if (newX >= 0 && newX < rows && newY >= 0 && newY < cols) {
                    int waitTime = std::max(moveTime[newX][newY] - currentTime, 0);
                    int newTime = currentTime + 1 + waitTime;

                    if (newTime < time[newX][newY]) {
                        time[newX][newY] = newTime;
                        minHeap.emplace(newTime, newX, newY);
                    }
                }
            }
        }

        return -1; 
    }
};

void printMatrix(const vector<vector<int>>& matrix) {
    cout << "Matrix dimensions: " << matrix.size() << "x";
    if (!matrix.empty()) cout << matrix[0].size();
    cout << endl;
    
    for (const auto& row : matrix) {
        for (const auto& val : row) {
            cout << val << " ";
        }
        cout << endl;
    }
}

int main() {
    int rows, cols;
    
    cin >> rows >> cols;
    
    vector<vector<int>> moveTime(rows, vector<int>(cols));

    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            cin >> moveTime[i][j];
        }
    }

    Solution solution;

    int result = solution.minTimeToReach(moveTime);
    cout << result << endl;

    return 0;
}
