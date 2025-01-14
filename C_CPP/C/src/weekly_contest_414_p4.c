#include <stdio.h>
#include <limits.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#define INF INT_MAX
#define N 50
#define MAX_STATE (1LL << N)  // 使用long long

typedef struct {
    int x, y;
} Position;

typedef struct {
    Position pos[N + 1];
    int dist[N + 1][N];
    int n;
} Solution;

typedef struct {
    Position queue[2500];
    int front, rear;
} Queue;

void initQueue(Queue *q) {
    q->front = q->rear = 0;
}

void enqueue(Queue *q, Position p) {
    q->queue[q->rear++] = p;
}

Position dequeue(Queue *q) {
    return q->queue[q->front++];
}

bool isEmpty(Queue *q) {
    return q->front == q->rear;
}

bool isValid(int x, int y) {
    return x >= 0 && x < N && y >= 0 && y < N;
}

void calculateDistances(Solution *sol) {
    int directions[8][2] = {{-2, -1}, {-2, 1}, {-1, -2}, {-1, 2},
                            {1, -2}, {1, 2}, {2, -1}, {2, 1}};
    
    for (int i = 0; i <= sol->n; ++i) {
        for (int j = 0; j < sol->n; ++j) {
            if (i == j) continue;
            Queue queue;
            initQueue(&queue);
            enqueue(&queue, sol->pos[i]);
            bool seen[N][N] = {false};
            seen[sol->pos[i].x][sol->pos[i].y] = true;
            int steps = 0;

            while (!isEmpty(&queue)) {
                int size = queue.rear - queue.front;
                for (int s = 0; s < size; ++s) {
                    Position current = dequeue(&queue);
                    if (current.x == sol->pos[j].x && current.y == sol->pos[j].y) {
                        sol->dist[i][j] = steps;
                        goto next_pair;
                    }
                    for (int d = 0; d < 8; ++d) {
                        int nx = current.x + directions[d][0];
                        int ny = current.y + directions[d][1];
                        if (isValid(nx, ny) && !seen[nx][ny]) {
                            enqueue(&queue, (Position){nx, ny});
                            seen[nx][ny] = true;
                        }
                    }
                }
                ++steps;
            }
            next_pair:;
        }
    }
}

// 使用动态分配的memo数组
int ***memo;

int dfs(Solution *sol, int i, long long m, int turn) {
    if (m == (1LL << sol->n) - 1) return 0;
    if (memo[i][m][turn] != -1) return memo[i][m][turn];

    int ans = (turn == 0) ? 0 : INF;
    for (int k = 0; k < sol->n; ++k) {
        if (!(m & (1LL << k))) {
            long long next_m = m | (1LL << k);
            int result = sol->dist[i][k] + dfs(sol, k, next_m, 1 - turn);
            if (turn == 0)
                ans = (result > ans) ? result : ans;
            else
                ans = (result < ans) ? result : ans;
        }
    }
    return memo[i][m][turn] = ans;
}

int maxMoves(int kx, int ky, int** positions, int positionsSize, int* positionsColSize) {
    Solution sol;
    sol.n = positionsSize;
    for (int i = 0; i < positionsSize; ++i) {
        sol.pos[i].x = positions[i][0];
        sol.pos[i].y = positions[i][1];
    }
    sol.pos[positionsSize] = (Position){kx, ky};

    memset(sol.dist, 0, sizeof(sol.dist));

    // 动态分配memo数组
    memo = (int ***)malloc((N + 1) * sizeof(int **));
    for (int i = 0; i <= N; ++i) {
        memo[i] = (int **)malloc((1LL << positionsSize) * sizeof(int *));
        for (long long j = 0; j < (1LL << positionsSize); ++j) {
            memo[i][j] = (int *)malloc(2 * sizeof(int));
            memo[i][j][0] = memo[i][j][1] = -1;
        }
    }

    calculateDistances(&sol);
    int result = dfs(&sol, sol.n, 0, 0);

    // 释放内存
    for (int i = 0; i <= N; ++i) {
        for (long long j = 0; j < (1LL << positionsSize); ++j) {
            free(memo[i][j]);
        }
        free(memo[i]);
    }
    free(memo);

    return result;
}

int main() {
    int kx, ky, positionsSize;
    scanf("%d %d", &kx, &ky);

    scanf("%d", &positionsSize);

    int** positions = (int**)malloc(positionsSize * sizeof(int*));
    int* positionsColSize = malloc(positionsSize * sizeof(int));

    for (int i = 0; i < positionsSize; ++i) {
        positions[i] = (int*)malloc(2 * sizeof(int));
        positionsColSize[i] = 2;
        scanf("%d %d", &positions[i][0], &positions[i][1]);
    }

    int result = maxMoves(kx, ky, positions, positionsSize, positionsColSize);
    printf("%d\n", result);

    for (int i = 0; i < positionsSize; ++i) {
        free(positions[i]);
    }
    free(positions);
    free(positionsColSize);

    return 0;
}