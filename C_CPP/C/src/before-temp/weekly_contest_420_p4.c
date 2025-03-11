// Problem: Weekly Contest 420 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

typedef struct
{
    int index;
    int next;
}
SubNode;

typedef struct
{
    int start;
    int end;
}
SectionNode;

typedef struct
{
    char *srcStr;
    char *dfsStr;
    int strIndex;
    int *hp;
    SubNode *list;
    SectionNode *sec;
}
DfsNode;

static void dfsCalc(DfsNode *node, int root);

bool *findAnswer(int *parent, int parentSize, char *s, int *returnSize)
{
    const int module = 1000000007, radix = 26;
    int x = 0, y = 0, z = 0, i = 0, j = 0, k = 0;
    DfsNode node;
    char dfsStr[parentSize];
    int hp[parentSize], forward[parentSize], backward[parentSize];
    SubNode list[parentSize];
    SectionNode sec[parentSize];
    bool *result = NULL;
    memset(hp, -1, sizeof(hp));
    result = (bool *)malloc(sizeof(bool) * parentSize);
    if(NULL == result)
    {
        *returnSize = 0;
        return result;
    }
    for(x = parentSize - 1; 0 < x; x--)
    {
        list[y].index = x;
        list[y].next = hp[parent[x]];
        hp[parent[x]] = y;
        y++;
    }
    node.srcStr = s;
    node.dfsStr = dfsStr;
    node.strIndex = 0;
    node.hp = hp;
    node.list = list;
    node.sec = sec;
    dfsCalc(&node, 0);
    for(x = 0; parentSize > x; x++)
    {
        y = parentSize - 1 - x;
        hp[x] = (0 == x) ? 1 : (long long)hp[x - 1] * radix % module;
        forward[x] = (0 == x) ? dfsStr[x] - 'a' : ((long long)forward[x - 1] * radix + dfsStr[x] - 'a') % module;
        backward[x] = (0 == x) ? dfsStr[y] - 'a' : ((long long)backward[x - 1] * radix + dfsStr[y] - 'a') % module;
    }
    for(x = 0; parentSize > x; x++)
    {
        k = sec[x].end - sec[x].start + 1;
        i = sec[x].start;
        j = sec[x].end;
        y = (0 == i) ? forward[j] : (((long long)forward[j] - (long long)forward[i - 1] * hp[k]) % module + module) % module;
        i = parentSize - 1 - sec[x].end;
        j = parentSize - 1 - sec[x].start;
        z = (0 == i) ? backward[j] : (((long long)backward[j] - (long long)backward[i - 1] * hp[k]) % module + module) % module;
        result[x] = (y == z) ? true : false;
    }
    *returnSize = parentSize;
    return result;
}

static void dfsCalc(DfsNode *node, int root)
{
    int x = 0;
    node->sec[root].start = node->strIndex;
    for(x = node->hp[root]; -1 != x; x = node->list[x].next)
    {
        dfsCalc(node, node->list[x].index);
    }
    node->dfsStr[node->strIndex] = node->srcStr[root];
    node->sec[root].end = node->strIndex;
    node->strIndex++;
    return;
}


int main() {
    int n;
    scanf("%d", &n);
    int *parent = (int *)malloc(sizeof(int) * n);
    for (int i = 0; i < n; i++) {
        scanf("%d", &parent[i]);
    }
    char *s = (char *)malloc(n + 1);
    scanf("%s", s);
    int returnSize = 0;
    bool *ans = findAnswer(parent, n, s, &returnSize);
    for (int i = 0; i < returnSize; i++) {
        if (ans[i]) {
            printf("true ");
        } else {
            printf("false ");
        }
    }
    printf("\n");
    free(parent);
    free(s);
    free(ans);
    return 0;
}
