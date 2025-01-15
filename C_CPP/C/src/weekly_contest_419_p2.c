// Problem: Weekly Contest 419 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <math.h>

struct TreeNode {
    int val;
    struct TreeNode *left;
    struct TreeNode *right;
};

static int *arr = NULL;
static int arr_size = 0;

int tree_judge(struct TreeNode* root, int floor)
{
    int left_floor = 0, right_floor = 0;
    if(root->left != NULL && root->right != NULL)
    {
        left_floor = tree_judge(root->left, floor + 1);
        right_floor = tree_judge(root->right, floor + 1);
    }
    else if(root->left != NULL)
    {
        left_floor = tree_judge(root->left, floor + 1);
        right_floor = 0;
    }
    else if(root->right != NULL)
    {
        left_floor = 0;
        right_floor = tree_judge(root->right, floor + 1);
    }
    else
    {
        left_floor = floor;
        right_floor = floor;
    }

    if(left_floor == right_floor && right_floor >= floor)
    {
        arr[arr_size++] = pow(2, right_floor - floor + 1) - 1;

        return left_floor;
    }

    return 0;
}

void quick_sort(int l, int r)
{
    if(l >= r)
    {
        return;
    }

    int l_t = l - 1;
    int r_t = r + 1;
    int mid_val = arr[(l + r) >> 1];

    while(l_t < r_t)
    {
        do
        {
            ++l_t;
        }while(arr[l_t] < mid_val);

        do
        {
            --r_t;
        }while(arr[r_t] > mid_val);

        if(l_t < r_t)
        {
            int tmp = arr[l_t];
            arr[l_t] = arr[r_t];
            arr[r_t] = tmp;
        }
    }

    quick_sort(l, r_t);
    quick_sort(r_t + 1, r);
}

int kthLargestPerfectSubtree(struct TreeNode* root, int k) {
    arr = (int *)malloc(10000 * sizeof(int));
    arr_size = 0;
    tree_judge(root, 1);

    quick_sort(0, arr_size - 1);
    if(arr_size - k < 0)
    {
        return -1;
    }

    return arr[arr_size - k];
}

struct TreeNode *create_tree(int *arr, int size)
{
    if (size == 0) return NULL;

    struct TreeNode *root = (struct TreeNode *)malloc(sizeof(struct TreeNode));
    root->val = arr[0];
    root->left = NULL;
    root->right = NULL;

    struct TreeNode **queue = (struct TreeNode **)malloc(size * sizeof(struct TreeNode *));
    int front = 0, rear = 0;
    queue[rear++] = root;

    for (int i = 1; i < size; i += 2)
    {
        struct TreeNode *current = queue[front++];
        
        if (arr[i] != 0)
        {
            struct TreeNode *leftNode = (struct TreeNode *)malloc(sizeof(struct TreeNode));
            leftNode->val = arr[i];
            leftNode->left = NULL;
            leftNode->right = NULL;
            current->left = leftNode;
            queue[rear++] = leftNode;
        }

        if (i + 1 < size && arr[i + 1] != 0)
        {
            struct TreeNode *rightNode = (struct TreeNode *)malloc(sizeof(struct TreeNode));
            rightNode->val = arr[i + 1];
            rightNode->left = NULL;
            rightNode->right = NULL;
            current->right = rightNode;
            queue[rear++] = rightNode;
        }
    }

    free(queue);
    return root;
}

// 用于释放二叉树的内存
void free_tree(struct TreeNode *root)
{
    if(root == NULL)
    {
        return;
    }

    free_tree(root->left);
    free_tree(root->right);
    free(root);
}

// 用于调试打印二叉树，使用层次格式打印
void print_tree(struct TreeNode *root)
{
    struct TreeNode **queue = (struct TreeNode **)malloc(10000 * sizeof(struct TreeNode *));
    int front = 0, rear = 0;
    queue[rear++] = root;
    while(front < rear)
    {
        struct TreeNode *node = queue[front++];
        if(node == NULL)
        {
            printf("null ");
        }
        else
        {
            printf("%d ", node->val);
            queue[rear++] = node->left;
            queue[rear++] = node->right;
        }
    }
}

int main() {
    int k;
    scanf("%d", &k);
    int arrSize;
    scanf("%d", &arrSize);
    int *arr = (int *)malloc(arrSize * sizeof(int));
    for(int i = 0; i < arrSize; ++i)
    {
        scanf("%d", &arr[i]);
    }
    struct TreeNode *root = create_tree(arr, arrSize);

    printf("%d\n", kthLargestPerfectSubtree(root, k));
    
    free(arr);
    free_tree(root);

    return 0;
}
