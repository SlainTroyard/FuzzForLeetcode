// Problem: Weekly Contest 427 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <limits.h>

#define FATHER_NODE(i)      (0 == (i) ? -1 : (i) - 1 >> 1)
#define LEFT_NODE(i)        (((i) << 1) + 1)
#define RIGHT_NODE(i)       (((i) << 1) + 2)
#define HIGH_INT(i)         ((i) >> 32)
#define LOW_INT(i)          ((i) & 0xFFFFFFFF)
#define MER_LONG(i, j)      ((long long)(i) << 32 | (long long)(j))
#define MAX_VAL(i, j)       ((i) > (j) ? (i) : (j))


typedef struct
{
    long long *arr;
    int arrSize;
}
PriorityQueue;


typedef struct
{
    int *arr;
    int highestBit;
}
BinaryTree;

static void queuePush(PriorityQueue *queue, long long value);
static void queuePop(PriorityQueue *queue);
static int binarySearch(int *map, int mapSize, int value);
static void treeHighestBit(BinaryTree *tree, int max);
static int treePushCount(BinaryTree *tree, int value);


long long maxRectangleArea(int *xCoord, int xCoordSize, int *yCoord, int yCoordSize)
{
    const int n = xCoordSize, treeSize = n * 3;
    int i = 0, j = 0, k = INT_MIN, x = 0, y = 0, number = 0, yMapSize = 0, buffSize = 0;
    int xMap[n], yMap[n], listsSize[n], listsBuff[n], prefixBuff[n], xLast[n], arr1[treeSize];
    
    int *lists[n], *prefix[n];
    long long arr2[n];
    BinaryTree tree;
    PriorityQueue queue;
    long long t = 0, result = -1;
    
    memset(xLast, -1, sizeof(xLast));
    memset(arr1, 0, sizeof(arr1));
    queue.arr = arr2;
    queue.arrSize = 0;
    tree.arr = arr1;
    treeHighestBit(&tree, n - 1);
    
    for (j = 0; n > j; j++)
    {
        queuePush(&queue, yCoord[j]);
    }
    while (0 < queue.arrSize)
    {
        
        if (k < queue.arr[0])
        {
            k = queue.arr[0];
            yMap[yMapSize] = k;
            yMapSize++;
        }
        queuePop(&queue);
    }
    
    for (j = 0; n > j; j++)
    {
        y = binarySearch(yMap, yMapSize, yCoord[j]);
        queuePush(&queue, MER_LONG(xCoord[j], y));
    }
    
    while (0 < queue.arrSize)
    {
        j = 0;
        lists[i] = &listsBuff[buffSize];
        prefix[i] = &prefixBuff[buffSize];
        xMap[i] = HIGH_INT(queue.arr[0]);
        
        while (0 < queue.arrSize && xMap[i] == HIGH_INT(queue.arr[0]))
        {
            
            lists[i][j] = LOW_INT(queue.arr[0]);
            prefix[i][j] = treePushCount(&tree, lists[i][j]);
            
            if (0 < j && -1 != xLast[lists[i][j]] && xLast[lists[i][j]] == k)
            {
                
                x = binarySearch(lists[k], listsSize[k], lists[i][j]);
                y = binarySearch(lists[k], listsSize[k], lists[i][j - 1]);
                number = prefix[i][j] - prefix[i][j - 1] - prefix[k][x] + prefix[k][y];
                
                if (x - 1 == y && 1 == number)
                {
                    t = (long long)(xMap[i] - xMap[k]) * (yMap[lists[i][j]] - yMap[lists[i][j - 1]]);
                    result = MAX_VAL(result, t);
                }
            }
            
            k = xLast[lists[i][j]];
            xLast[lists[i][j]] = i;
            
            queuePop(&queue);
            j++;
        }
        
        listsSize[i] = j;
        buffSize += j;
        i++;
    }
    return result;
}


static void queuePush(PriorityQueue *queue, long long value)
{
    int son = queue->arrSize, father = FATHER_NODE(son);
    queue->arrSize++;
    while (-1 != father && value < queue->arr[father])
    {
        queue->arr[son] = queue->arr[father];
        son = father;
        father = FATHER_NODE(son);
    }
    queue->arr[son] = value;
    return;
}


static void queuePop(PriorityQueue *queue)
{
    int father = 0, left = LEFT_NODE(father), right = RIGHT_NODE(father), son = 0;
    queue->arrSize--;
    while ((queue->arrSize > left && queue->arr[queue->arrSize] > queue->arr[left])
        || (queue->arrSize > right && queue->arr[queue->arrSize] > queue->arr[right]))
    {
        son = (queue->arrSize > right && queue->arr[left] > queue->arr[right]) ? right : left;
        queue->arr[father] = queue->arr[son];
        father = son;
        left = LEFT_NODE(father);
        right = RIGHT_NODE(father);
    }
    queue->arr[father] = queue->arr[queue->arrSize];
    return;
}


static int binarySearch(int *map, int mapSize, int value)
{
    int mid = -1, left = 0, right = mapSize - 1;
    if (value < map[left])
    {
        return mid;
    }
    while (left < right)
    {
        mid = left + right + 1 >> 1;
        if (value < map[mid])
        {
            right = mid - 1;
        }
        else
        {
            left = mid;
        }
    }
    return left;
}


static void treeHighestBit(BinaryTree *tree, int max)
{
    int i = 1;
    if (0 != max)
    {
        while (0 != max)
        {
            i++;
            max = max >> 1;
        }
        i = 1 << i - 2;
    }
    tree->highestBit = i;
    return;
}


static int treePushCount(BinaryTree *tree, int value)
{
    int i = 0, bit = tree->highestBit, result = 0;
    
    while (0 != bit)
    {
        if (bit & value)
        {
            result += tree->arr[LEFT_NODE(i)];
            i = RIGHT_NODE(i);
        }
        else
        {
            i = LEFT_NODE(i);
        }
        tree->arr[i]++;
        bit = bit >> 1;
    }
    
    result += tree->arr[i];
    return result;
}



int main() {
    int n;

    scanf("%d", &n);

    int xCoord[n], yCoord[n];


    for (int i = 0; i < n; i++) {
        scanf("%d %d", &xCoord[i], &yCoord[i]);
    }

    long long maxArea = maxRectangleArea(xCoord, n, yCoord, n);

    printf("%lld\n", maxArea);

    return 0;
}
