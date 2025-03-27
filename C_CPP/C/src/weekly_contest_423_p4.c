// Problem: Weekly Contest 423 Problem 4
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

#define MOST_CNT    801
#define MODULO_VAL  1000000007


static int hasCalc = 0;
static int digitsCnt[MOST_CNT];
static int reducibleCnt[MOST_CNT];
static int combVal[MOST_CNT][MOST_CNT];


static void preProcess(void);


int countKReducibleNumbers(char *s, int k)
{
    int i = 0, j = 0, m = 0, len = 0, one = 0, result = 0;
    
    if(0 == hasCalc)
    {
        preProcess();
        hasCalc = 1;
    }
    
    for(i = 0; '\0' != s[i]; i++)
    {
        if('1' == s[i])
        {
            one++;
        }
    }
    len = i;
    
    for(i = len - 1; 0 <= i; i--)
    {
        if('1' == s[i])
        {
            one--;
            
            j = len - i - 1;
            for(m = 0; j >= m; m++)
            {
                
                if(0 < one + m && k > reducibleCnt[one + m])
                {
                    result = (result + combVal[j][m]) % MODULO_VAL;
                }
            }
        }
    }
    return result;
}


static void preProcess(void)
{
    int i = 0, j = 0;
    
    digitsCnt[0] = 0;
    reducibleCnt[0] = 0;
    digitsCnt[1] = 1;
    reducibleCnt[1] = 0;
    combVal[0][0] = 1;
    combVal[1][0] = 1;
    combVal[1][1] = 1;
    for(i = 2; MOST_CNT > i; i++)
    {
        digitsCnt[i] = digitsCnt[i >> 1] + (i & 1);
        reducibleCnt[i] = reducibleCnt[digitsCnt[i]] + 1;
        combVal[i][0] = 1;
        combVal[i][i] = 1;
        for(j = 1; i > j; j++)
        {
            combVal[i][j] = (combVal[i - 1][j] + combVal[i - 1][j - 1]) % MODULO_VAL;
        }
    }
    return;
}

int main()
{
    char s[1001];
    int k;
    scanf("%s", s);
    scanf("%d", &k);
    
    int result = countKReducibleNumbers(s, k);
    
    printf("%d\n", result);
    
    return 0;
}
