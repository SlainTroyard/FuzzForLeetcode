// Problem: Weekly Contest 419 Problem 2
#include <iostream>
#include <string>
#include <vector>
#include <queue>
#include <algorithm>
using namespace std;

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};

class Solution {
public:
    int kthLargestPerfectSubtree(TreeNode* root, int k) {
        vector<int> hs;
        auto dfs = [&](auto&& dfs, TreeNode* node) -> int {
            if (node == nullptr) {
                return 0;
            }
            int left_h = dfs(dfs, node->left);
            int right_h = dfs(dfs, node->right);
            if (left_h < 0 || left_h != right_h) {
                return -1;
            }
            hs.push_back(left_h + 1);
            return left_h + 1;
        };
        dfs(dfs, root);

        if (k > hs.size()) {
            return -1;
        }
        std::nth_element(hs.begin(), hs.end() - k, hs.end());
        return (1 << hs[hs.size() - k]) - 1;
    }
};

TreeNode* createTree(vector<int>& arr) {
    if(arr.empty()) return nullptr;
    
    TreeNode* root = new TreeNode(arr[0]);
    queue<TreeNode*> q;
    q.push(root);
    
    for(int i = 1; i < arr.size(); i += 2) {
        TreeNode* current = q.front();
        q.pop();
        
        if(arr[i] != 0) {
            current->left = new TreeNode(arr[i]);
            q.push(current->left);
        }
        
        if(i + 1 < arr.size() && arr[i + 1] != 0) {
            current->right = new TreeNode(arr[i + 1]);
            q.push(current->right);
        }
    }
    
    return root;
}

void freeTree(TreeNode* root) {
    if(!root) return;
    freeTree(root->left);
    freeTree(root->right);
    delete root;
}

int main() {
    int k, n;
    cin >> k >> n;

    vector<int> arr(n);
    for(int i = 0; i < n; i++) {
        cin >> arr[i];
    }

    TreeNode* root = createTree(arr);
    Solution solution;
    cout << solution.kthLargestPerfectSubtree(root, k) << endl;

    freeTree(root);
    return 0;
}
