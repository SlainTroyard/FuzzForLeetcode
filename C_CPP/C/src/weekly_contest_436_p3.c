// Problem: Weekly Contest 436 Problem 3
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// 主函数实现
long long countSubstrings(char* s) {
    long long ans = 0;
    int f[10][9] = {0}; // 初始化计数数组为0
    
    int len = strlen(s);
    for (int i = 0; i < len; i++) {
        int d = s[i] - '0'; // 当前数字
        
        for (int m = 1; m < 10; ++m) {
            int nf[9] = {0}; // 临时数组，用于保存新的计数结果
            nf[d % m] = 1;   // 单个数字d模m的余数
            
            for (int rem = 0; rem < m; ++rem) {
                // 更新计数：将当前数字d添加到已有余数rem后面形成的新余数
                nf[(rem * 10 + d) % m] += f[m][rem];
            }
            
            // 更新f数组
            for (int rem = 0; rem < m; ++rem) {
                f[m][rem] = nf[rem];
            }
        }
        
        // 当前数字被自身整除，增加结果计数
        ans += f[d][0];
    }
    
    return ans;
}

int main() {
    // 读取输入
    char s[100001]; // 假设字符串最大长度为100000
    
    if (scanf("%s", s) != 1) {
        fprintf(stderr, "Error reading input\n");
        return 1;
    }
    
    // 调用函数计算结果
    long long result = countSubstrings(s);
    
    // 输出结果
    printf("%lld\n", result);
    
    return 0;
}
