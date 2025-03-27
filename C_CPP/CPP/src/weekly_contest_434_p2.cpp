// Problem: Weekly Contest 434 Problem 2
#include <iostream>
#include <vector>
#include <string>
#include <algorithm>
#include <regex>
#include <sstream>

using namespace std;

class Solution {
private:
    static bool cmp(vector<string>& a, vector<string>& b){
        int time_a = stoi(a[1]);
        int time_b = stoi(b[1]);
        return (time_a == time_b && a[0] != b[0]) ? a[0] == "OFFLINE" : time_a < time_b;
    }

    int extractId(const string& idStr) {
        if (idStr.substr(0, 2) == "id") {
            try {
                return stoi(idStr.substr(2));
            } catch (const std::exception& e) {
                cerr << "Error parsing ID: " << idStr << endl;
                return 0; 
            }
        } else {
            try {
                return stoi(idStr);
            } catch (const std::exception& e) {
                cerr << "Error parsing ID: " << idStr << endl;
                return 0; 
            }
        }
    }

    void updateCounts(vector<int>& counts, string& mentions){
        istringstream iss(mentions);
        string idStr;
        
        while (iss >> idStr) {
            try {
                int id = extractId(idStr);
                if (id >= 0 && id < counts.size()) {
                    counts[id]++;
                }
            } catch (const std::exception& e) {
                cerr << "Error in updateCounts: " << e.what() << endl;
            }
        }
    }

public:
    vector<int> countMentions(int numberOfUsers, vector<vector<string>>& events) {
        sort(events.begin(), events.end(), cmp);
        vector<int> onlineTimes(numberOfUsers, 0);
        vector<int> counts(numberOfUsers, 0);
        int allCount = 0;
        
        for(vector<string>& event: events)
        {
            int time = stoi(event[1]);
            if(event[0] == "OFFLINE")
            {
                try {
                    int id = extractId(event[2]);
                    if (id >= 0 && id < numberOfUsers) {
                        onlineTimes[id] = time + 60;
                    }
                } catch (const std::exception& e) {
                    cerr << "Error processing OFFLINE event: " << e.what() << endl;
                }
                continue;
            }
            
            string mentions = event[2];
            if(mentions == "ALL")
                allCount++;
            else if(mentions == "HERE")
            {
                for(int i=0; i<numberOfUsers; i++)
                    if(onlineTimes[i] <= time)
                        counts[i]++;
            }
            else
                updateCounts(counts, mentions);
        }
        
        if(allCount > 0)
            for(int& count: counts)
                count += allCount;
        
        return counts;
    }
};

int main() {
    int numberOfUsers, numberOfEvents;
    cin >> numberOfUsers >> numberOfEvents;
    
    vector<vector<string>> events(numberOfEvents, vector<string>(3));
    for (int i = 0; i < numberOfEvents; i++) {
        cin >> events[i][0] >> events[i][1];
        
        if (events[i][0] == "MESSAGE") {
            string messageType;
            cin >> messageType;
            
            if (messageType == "ALL" || messageType == "HERE") {
                events[i][2] = messageType;
            } else {
                events[i][2] = messageType;
                string restOfLine;
                getline(cin, restOfLine);
                if (!restOfLine.empty()) {
                    events[i][2] += restOfLine;
                }
            }
        } else if (events[i][0] == "OFFLINE") {
            cin >> events[i][2];
        }
    }
    
    try {
        Solution solution;
        vector<int> result = solution.countMentions(numberOfUsers, events);
        
        for (int count : result) {
            cout << count << " ";
        }
        cout << endl;
    } catch (const std::exception& e) {
        cerr << "Exception in main: " << e.what() << endl;
        return 1;
    }
    
    return 0;
}
