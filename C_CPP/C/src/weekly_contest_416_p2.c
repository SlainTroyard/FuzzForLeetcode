// Problem: Weekly Contest 416 Problem 2
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdio.h>

bool cal_time(long long mountainHeight, int* workerTimes, int workerTimesSize, long long target_time)
{
    long long l_height = 0, r_height = 1e6;
    long long mid_height = 0;
    long long use_time = 0;
    for(int i = 0; i < workerTimesSize; ++i)
    {
        l_height = 0;
        r_height = 1e6;
        while(r_height >= l_height)
        {
            mid_height = (l_height + r_height) / 2;
            use_time = mid_height * (workerTimes[i] + mid_height * workerTimes[i]) / 2;
            if(use_time > target_time)
            {
                r_height = mid_height - 1;
            }
            else
            {
                l_height = mid_height + 1;
            }
        }
        mountainHeight -= r_height;
    }
    return (mountainHeight > 0) ? false : true;
}

long long minNumberOfSeconds(int mountainHeight, int* workerTimes, int workerTimesSize) {
    long long r_time = 1e18, l_time = 0;
    long long mid_time = 0;

    while(r_time >= l_time)
    {
        mid_time = (r_time + l_time) / 2;
        if(cal_time(mountainHeight, workerTimes, workerTimesSize, mid_time))
        {
            r_time = mid_time - 1;
        }
        else
        {
            l_time = mid_time + 1;
        }
    }
    return ++r_time;
}

int main() {
    int mountainHeight, workerTimesSize;
    scanf("%d %d", &mountainHeight, &workerTimesSize);
    int* workerTimes = (int*)malloc(sizeof(int) * workerTimesSize);
    for(int i = 0; i < workerTimesSize; ++i)
    {
        scanf("%d", &workerTimes[i]);
    }
    printf("%lld\n", minNumberOfSeconds(mountainHeight, workerTimes, workerTimesSize));
    free(workerTimes);
    return 0;
}
