use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minArraySum(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
    mut op1: libc::c_int,
    mut op2: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = numsSize;
    let mut INF: libc::c_int = 2147483647 as libc::c_int / 2 as libc::c_int;
    let mut dp: *mut *mut *mut libc::c_int = malloc(
        ((n + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_int>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= n {
        let ref mut fresh0 = *dp.offset(i as isize);
        *fresh0 = malloc(
            ((op1 + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j <= op1 {
            let ref mut fresh1 = *(*dp.offset(i as isize)).offset(j as isize);
            *fresh1 = malloc(
                ((op2 + 1 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            let mut l: libc::c_int = 0 as libc::c_int;
            while l <= op2 {
                *(*(*dp.offset(i as isize)).offset(j as isize)).offset(l as isize) = INF;
                l += 1;
                l;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    *(*(*dp.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let mut usedOp1: libc::c_int = 0 as libc::c_int;
        while usedOp1 <= op1 {
            let mut usedOp2: libc::c_int = 0 as libc::c_int;
            while usedOp2 <= op2 {
                let mut currSum: libc::c_int = *(*(*dp.offset(i_0 as isize))
                    .offset(usedOp1 as isize))
                    .offset(usedOp2 as isize);
                if !(currSum >= INF) {
                    let mut num: libc::c_int = *nums.offset(i_0 as isize);
                    if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                        .offset(usedOp1 as isize))
                        .offset(usedOp2 as isize) > currSum + num
                    {
                        *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                            .offset(usedOp1 as isize))
                            .offset(usedOp2 as isize) = currSum + num;
                    }
                    if usedOp1 < op1 {
                        let mut newNum: libc::c_int = (num + 1 as libc::c_int)
                            / 2 as libc::c_int;
                        if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                            .offset((usedOp1 + 1 as libc::c_int) as isize))
                            .offset(usedOp2 as isize) > currSum + newNum
                        {
                            *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                .offset((usedOp1 + 1 as libc::c_int) as isize))
                                .offset(usedOp2 as isize) = currSum + newNum;
                        }
                    }
                    if usedOp2 < op2 && num >= k {
                        let mut newNum_0: libc::c_int = num - k;
                        if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                            .offset(usedOp1 as isize))
                            .offset((usedOp2 + 1 as libc::c_int) as isize)
                            > currSum + newNum_0
                        {
                            *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                .offset(usedOp1 as isize))
                                .offset(
                                    (usedOp2 + 1 as libc::c_int) as isize,
                                ) = currSum + newNum_0;
                        }
                    }
                    if usedOp1 < op1 && usedOp2 < op2 {
                        let mut tempNum: libc::c_int = (num + 1 as libc::c_int)
                            / 2 as libc::c_int;
                        if tempNum >= k {
                            let mut newNum_1: libc::c_int = tempNum - k;
                            if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                .offset((usedOp1 + 1 as libc::c_int) as isize))
                                .offset((usedOp2 + 1 as libc::c_int) as isize)
                                > currSum + newNum_1
                            {
                                *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                    .offset((usedOp1 + 1 as libc::c_int) as isize))
                                    .offset(
                                        (usedOp2 + 1 as libc::c_int) as isize,
                                    ) = currSum + newNum_1;
                            }
                        } else if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                            .offset((usedOp1 + 1 as libc::c_int) as isize))
                            .offset((usedOp2 + 1 as libc::c_int) as isize)
                            > currSum + tempNum
                        {
                            *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                .offset((usedOp1 + 1 as libc::c_int) as isize))
                                .offset(
                                    (usedOp2 + 1 as libc::c_int) as isize,
                                ) = currSum + tempNum;
                        }
                        if num >= k {
                            tempNum = num - k;
                            let mut newNum_2: libc::c_int = (tempNum + 1 as libc::c_int)
                                / 2 as libc::c_int;
                            if *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                .offset((usedOp1 + 1 as libc::c_int) as isize))
                                .offset((usedOp2 + 1 as libc::c_int) as isize)
                                > currSum + newNum_2
                            {
                                *(*(*dp.offset((i_0 + 1 as libc::c_int) as isize))
                                    .offset((usedOp1 + 1 as libc::c_int) as isize))
                                    .offset(
                                        (usedOp2 + 1 as libc::c_int) as isize,
                                    ) = currSum + newNum_2;
                            }
                        }
                    }
                }
                usedOp2 += 1;
                usedOp2;
            }
            usedOp1 += 1;
            usedOp1;
        }
        i_0 += 1;
        i_0;
    }
    let mut minSum: libc::c_int = INF;
    let mut usedOp1_0: libc::c_int = 0 as libc::c_int;
    while usedOp1_0 <= op1 {
        let mut usedOp2_0: libc::c_int = 0 as libc::c_int;
        while usedOp2_0 <= op2 {
            if minSum
                > *(*(*dp.offset(n as isize)).offset(usedOp1_0 as isize))
                    .offset(usedOp2_0 as isize)
            {
                minSum = *(*(*dp.offset(n as isize)).offset(usedOp1_0 as isize))
                    .offset(usedOp2_0 as isize);
            }
            usedOp2_0 += 1;
            usedOp2_0;
        }
        usedOp1_0 += 1;
        usedOp1_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 <= n {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 <= op1 {
            free(*(*dp.offset(i_1 as isize)).offset(j_0 as isize) as *mut libc::c_void);
            j_0 += 1;
            j_0;
        }
        free(*dp.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(dp as *mut libc::c_void);
    return minSum;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut op1: libc::c_int = 0;
    let mut op2: libc::c_int = 0;
    scanf(
        b"%d %d %d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
        &mut op1 as *mut libc::c_int,
        &mut op2 as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = minArraySum(nums, n, k, op1, op2);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
