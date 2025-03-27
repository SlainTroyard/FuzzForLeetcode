// Problem: Weekly Contest 432 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

typedef struct Node {
    int vertex;
    struct Node* next;
} Node;

typedef struct {
    int* data;
    int front;
    int rear;
    int capacity;
    int size;
} Queue;

Queue* createQueue(int capacity) {
    Queue* queue = (Queue*)malloc(sizeof(Queue));
    if (!queue) {
        fprintf(stderr, "Memory allocation failed for queue\n");
        exit(1);
    }
    queue->data = (int*)malloc(capacity * sizeof(int));
    if (!queue->data) {
        fprintf(stderr, "Memory allocation failed for queue data\n");
        free(queue);
        exit(1);
    }
    queue->front = 0;
    queue->rear = -1;
    queue->capacity = capacity;
    queue->size = 0;
    return queue;
}

bool isEmpty(Queue* queue) {
    return queue->size == 0;
}

void enqueue(Queue* queue, int item) {
    if (queue->size == queue->capacity) return;
    queue->rear = (queue->rear + 1) % queue->capacity;
    queue->data[queue->rear] = item;
    queue->size++;
}

int dequeue(Queue* queue) {
    if (isEmpty(queue)) return -1;
    int item = queue->data[queue->front];
    queue->front = (queue->front + 1) % queue->capacity;
    queue->size--;
    return item;
}

void destroyQueue(Queue* queue) {
    free(queue->data);
    free(queue);
}

void addEdge(Node** adjList, int src, int dest) {
    Node* newNode = (Node*)malloc(sizeof(Node));
    if (!newNode) {
        fprintf(stderr, "Memory allocation failed for new node\n");
        exit(1);
    }
    newNode->vertex = dest;
    newNode->next = adjList[src];
    adjList[src] = newNode;
}

bool check(int n, int** edges, int edgesSize, int limit) {
    Node** adjList = (Node**)malloc(n * sizeof(Node*));
    if (!adjList) {
        fprintf(stderr, "Memory allocation failed for adjacency list\n");
        exit(1);
    }
    for (int i = 0; i < n; i++) {
        adjList[i] = NULL;
    }
    
    for (int i = 0; i < edgesSize; i++) {
        if (edges[i][2] <= limit) {
            addEdge(adjList, edges[i][1], edges[i][0]); 
        }
    }
    
    bool* visited = (bool*)calloc(n, sizeof(bool));
    if (!visited) {
        fprintf(stderr, "Memory allocation failed for visited array\n");
        for (int i = 0; i < n; i++) {
            Node* current = adjList[i];
            while (current != NULL) {
                Node* temp = current;
                current = current->next;
                free(temp);
            }
        }
        free(adjList);
        exit(1);
    }
    
    Queue* queue = createQueue(n);
    
    visited[0] = true;
    enqueue(queue, 0);
    
    while (!isEmpty(queue)) {
        int currentVertex = dequeue(queue);
        Node* temp = adjList[currentVertex];
        
        while (temp != NULL) {
            int adjVertex = temp->vertex;
            if (!visited[adjVertex]) {
                visited[adjVertex] = true;
                enqueue(queue, adjVertex);
            }
            temp = temp->next;
        }
    }
    
    bool allVisited = true;
    for (int i = 0; i < n; i++) {
        if (!visited[i]) {
            allVisited = false;
            break;
        }
    }
    
    for (int i = 0; i < n; i++) {
        Node* current = adjList[i];
        while (current != NULL) {
            Node* temp = current;
            current = current->next;
            free(temp);
        }
    }
    free(adjList);
    free(visited);
    destroyQueue(queue);
    
    return allVisited;
}

int findMaxWeight(int** edges, int edgesSize) {
    int maxWeight = 0;
    for (int i = 0; i < edgesSize; i++) {
        if (edges[i][2] > maxWeight) {
            maxWeight = edges[i][2];
        }
    }
    return maxWeight;
}

int minMaxWeight(int n, int** edges, int edgesSize, int* edgesColSize, int threshold) {
    int maxWeight = findMaxWeight(edges, edgesSize);
    
    if (!check(n, edges, edgesSize, maxWeight)) {
        return -1;
    }
    
    int left = 0;
    int right = maxWeight;
    
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (check(n, edges, edgesSize, mid)) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    return left;
}

int main() {
    int n, threshold;
    if (scanf("%d %d", &n, &threshold) != 2) {
        fprintf(stderr, "Error reading input for n and threshold\n");
        return 1;
    }
    
    int edgesSize = 0;
    int capacity = 100000; 
    
    int** edges = (int**)malloc(capacity * sizeof(int*));

    
    int* edgesColSize = (int*)malloc(capacity * sizeof(int));

    
    int src, dest, weight;
    while (scanf("%d %d %d", &src, &dest, &weight) == 3) {
        if (edgesSize >= capacity) {
            fprintf(stderr, "Warning: Maximum edge capacity reached (%d edges)\n", capacity);
            break;
        }
        
        edges[edgesSize] = (int*)malloc(3 * sizeof(int));

        
        edges[edgesSize][0] = src;
        edges[edgesSize][1] = dest;
        edges[edgesSize][2] = weight;
        edgesColSize[edgesSize] = 3;
        edgesSize++;
    }
    
    int result = minMaxWeight(n, edges, edgesSize, edgesColSize, threshold);
    printf("%d\n", result);
    
    for (int i = 0; i < edgesSize; i++) {
        free(edges[i]);
    }
    free(edges);
    free(edgesColSize);
    
    return 0;
}
