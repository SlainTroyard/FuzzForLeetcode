// Problem: Weekly Contest 421 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MODULO_VAL    1000000007
#define RADIX_VAL     26

typedef struct
{
    int m[RADIX_VAL][RADIX_VAL];
}
Matrix;

static void matrixMultiply(Matrix *a, Matrix *b, Matrix *result);

int lengthAfterTransformations(char *s, int t, int *nums, int numsSize)
{
    int x = 0, y = 0, z = 0, digitsSize = 0, result = 0;
    int src[RADIX_VAL], digits[32];
    Matrix init;
    Matrix dp[2];
    memset(src, 0, sizeof(src));
    memset(&init, 0, sizeof(init));
    memset(dp, 0, sizeof(dp));
    for(x = 0; RADIX_VAL > x; x++)
    {
        dp[0].m[x][x] = 1;
        for(y = 1; nums[x] >= y; y++)
        {
            z = (RADIX_VAL > x + y) ? x + y : x + y - RADIX_VAL;
            init.m[z][x] = 1;
        }
    }
    for(x = 0; '\0' != s[x]; x++)
    {
        src[s[x] - 'a']++;
    }
    for(x = t; 0 != x; x = x >> 1)
    {
        digits[digitsSize] = x & 1;
        digitsSize++;
    }
    z = 0;
    for(x = digitsSize - 1; 0 <= x; x--)
    {
        matrixMultiply(&dp[z], &dp[z], &dp[1 - z]);
        if(1 == digits[x])
        {
            matrixMultiply(&dp[1 - z], &init, &dp[z]);
        }
        else
        {
            z = 1 - z;
        }
    }
    for(x = 0; RADIX_VAL > x; x++)
    {
        for(y = 0; RADIX_VAL > y; y++)
        {
            result = ((long long)dp[z].m[x][y] * src[y] + result) % MODULO_VAL;
        }
    }
    return result;
}

static void matrixMultiply(Matrix *a, Matrix *b, Matrix *result)
{
    int x = 0, y = 0, z = 0;
    for(x = 0; RADIX_VAL > x; x++)
    {
        for(y = 0; RADIX_VAL > y; y++)
        {
            result->m[x][y] = 0;
            for(z = 0; RADIX_VAL > z; z++)
            {
                result->m[x][y] = ((long long)a->m[x][z] * b->m[z][y] + result->m[x][y]) % MODULO_VAL;
            }
        }
    }
    return;
}


int main() {
    int s_len, t, nums_size;
    nums_size = 26;
    scanf("%d", &s_len);
    char* s = (char*)malloc((s_len + 1) * sizeof(char));
    scanf("%s", s);
    scanf("%d", &t);
    int* nums = (int*)malloc(nums_size * sizeof(int));
    for (int i = 0; i < nums_size; i++) {
        scanf("%d", &nums[i]);
    }
    int result = lengthAfterTransformations(s, t, nums, nums_size);
    printf("%d\n", result);
    free(s);
    free(nums);
    return 0;
}
