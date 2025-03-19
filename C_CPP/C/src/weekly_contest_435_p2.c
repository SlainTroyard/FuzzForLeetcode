// Problem: Weekly Contest 435 Problem 2
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

// 计算绝对值
int abs_val(int x) {
    return x < 0 ? -x : x;
}

// 返回两个数中的较小值
int min(int a, int b) {
    return a < b ? a : b;
}

// 返回两个数中的较大值
int max(int a, int b) {
    return a > b ? a : b;
}

// 主函数实现
int maxDistance(char* s, int k) {
    int ans = 0, x = 0, y = 0;
    int length = strlen(s);
    
    for (int i = 0; i < length; i++) {
        if (s[i] == 'N') y++;
        else if (s[i] == 'S') y--;
        else if (s[i] == 'E') x++;
        else if (s[i] == 'W') x--;
        
        // 根据当前位置、当前坐标和k值计算最大距离
        int current_max = min(abs_val(x) + abs_val(y) + k * 2, i + 1);
        ans = max(ans, current_max);
    }
    
    return ans;
}

int main() {
    // 读取输入
    char s[100001]; // 假设字符串最大长度为10^5
    int k;
    
    if (scanf("%s %d", s, &k) != 2) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    // 调用函数计算结果
    int result = maxDistance(s, k);
    
    // 输出结果
    printf("%d\n", result);
    
    return 0;
}
