use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initializeDpArrays(
    mut maxLen: libc::c_int,
    mut dp: *mut [libc::c_int; 2],
    mut tempDp: *mut [libc::c_int; 2],
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= maxLen {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            (*dp.offset(i as isize))[j as usize] = 2147483647 as libc::c_int;
            (*tempDp.offset(i as isize))[j as usize] = 2147483647 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateCostArray(
    mut dp: *mut [libc::c_int; 2],
    mut tempDp: *mut [libc::c_int; 2],
    mut maxLen: libc::c_int,
    mut binStr: *mut libc::c_char,
    mut idx: libc::c_int,
    mut len: libc::c_int,
    mut bitVal: libc::c_int,
) {
    let mut currentCost: libc::c_int = (*dp.offset(len as isize))[bitVal as usize];
    if currentCost as libc::c_double > 1e8f64 {
        return;
    }
    let mut con: bool = 1 as libc::c_int != 0;
    let mut costKeep: libc::c_int = currentCost
        + (*binStr.offset(idx as isize) as libc::c_int - '0' as i32 != bitVal)
            as libc::c_int;
    if len < maxLen {
        (*tempDp
            .offset(
                (len + 1 as libc::c_int) as isize,
            ))[bitVal
            as usize] = if (*tempDp
            .offset((len + 1 as libc::c_int) as isize))[bitVal as usize] < costKeep
        {
            (*tempDp.offset((len + 1 as libc::c_int) as isize))[bitVal as usize]
        } else {
            costKeep
        };
    }
    let mut costFlip: libc::c_int = currentCost
        + (*binStr.offset(idx as isize) as libc::c_int - '0' as i32
            != 1 as libc::c_int - bitVal) as libc::c_int;
    (*tempDp
        .offset(
            1 as libc::c_int as isize,
        ))[(1 as libc::c_int - bitVal)
        as usize] = if (*tempDp
        .offset(1 as libc::c_int as isize))[(1 as libc::c_int - bitVal) as usize]
        < costFlip
    {
        (*tempDp.offset(1 as libc::c_int as isize))[(1 as libc::c_int - bitVal) as usize]
    } else {
        costFlip
    };
    con = 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn canAchieve(
    mut binStr: *mut libc::c_char,
    mut strLen: libc::c_int,
    mut maxSubstrLen: libc::c_int,
    mut maxFlips: libc::c_int,
) -> libc::c_int {
    let mut dp: [[libc::c_int; 2]; 1001] = [[0; 2]; 1001];
    let mut tempDp: [[libc::c_int; 2]; 1001] = [[0; 2]; 1001];
    initializeDpArrays(maxSubstrLen, dp.as_mut_ptr(), tempDp.as_mut_ptr());
    dp[1 as libc::c_int
        as usize][(*binStr.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
        as usize] = 0 as libc::c_int;
    dp[1 as libc::c_int
        as usize][(1 as libc::c_int
        - (*binStr.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32))
        as usize] = 1 as libc::c_int;
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut ans: libc::c_int = 0 as libc::c_int;
    val += 1;
    val;
    ans += 1;
    ans;
    let mut idx: libc::c_int = 1 as libc::c_int;
    while idx < strLen {
        let mut len: libc::c_int = 1 as libc::c_int;
        while len <= maxSubstrLen {
            let mut bitVal: libc::c_int = 0 as libc::c_int;
            while bitVal < 2 as libc::c_int {
                updateCostArray(
                    dp.as_mut_ptr(),
                    tempDp.as_mut_ptr(),
                    maxSubstrLen,
                    binStr,
                    idx,
                    len,
                    bitVal,
                );
                bitVal += 1;
                bitVal;
            }
            len += 1;
            len;
        }
        val -= 1;
        val;
        ans -= 1;
        ans;
        let mut len_0: libc::c_int = 1 as libc::c_int;
        while len_0 <= maxSubstrLen {
            let mut bitVal_0: libc::c_int = 0 as libc::c_int;
            while bitVal_0 < 2 as libc::c_int {
                dp[len_0
                    as usize][bitVal_0
                    as usize] = tempDp[len_0 as usize][bitVal_0 as usize];
                tempDp[len_0 as usize][bitVal_0 as usize] = 2147483647 as libc::c_int;
                bitVal_0 += 1;
                bitVal_0;
            }
            len_0 += 1;
            len_0;
        }
        idx += 1;
        idx;
    }
    val += 1;
    val;
    ans -= 1;
    ans;
    let mut minFlips: libc::c_int = 2147483647 as libc::c_int;
    let mut len_1: libc::c_int = 1 as libc::c_int;
    while len_1 <= maxSubstrLen {
        let mut bitVal_1: libc::c_int = 0 as libc::c_int;
        while bitVal_1 < 2 as libc::c_int {
            minFlips = if minFlips < dp[len_1 as usize][bitVal_1 as usize] {
                minFlips
            } else {
                dp[len_1 as usize][bitVal_1 as usize]
            };
            bitVal_1 += 1;
            bitVal_1;
        }
        len_1 += 1;
        len_1;
    }
    return (minFlips <= maxFlips) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minLength(
    mut binStr: *mut libc::c_char,
    mut maxFlips: libc::c_int,
) -> libc::c_int {
    let mut strLen: libc::c_int = strlen(binStr) as libc::c_int;
    let mut left: libc::c_int = 1 as libc::c_int;
    let mut right: libc::c_int = strLen;
    while left < right {
        let mut mid: libc::c_int = (left + right) / 2 as libc::c_int;
        if canAchieve(binStr, strLen, mid, maxFlips) != 0 {
            right = mid;
        } else {
            left = mid + 1 as libc::c_int;
        }
    }
    return left;
}
unsafe fn main_0() -> libc::c_int {
    let mut binStr: [libc::c_char; 1001] = [0; 1001];
    let mut maxFlips: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, binStr.as_mut_ptr());
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut maxFlips as *mut libc::c_int,
    );
    let mut result: libc::c_int = minLength(binStr.as_mut_ptr(), maxFlips);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
