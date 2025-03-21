use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn dfs(
    mut dp: *mut libc::c_int,
    mut a: *mut libc::c_int,
    mut k: libc::c_int,
    mut i: libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    if i >= size {
        return 0 as libc::c_int;
    }
    if *dp.offset(i as isize) != -(1 as libc::c_int) {
        return *dp.offset(i as isize);
    }
    let mut ans: libc::c_int = 1000000 as libc::c_int;
    if *a.offset(i as isize) >= k {
        let mut more: libc::c_int = *a.offset(i as isize) - k;
        ans = if ans
            < *a.offset(i as isize) - k + dfs(dp, a, k, i + 1 as libc::c_int, size)
        {
            ans
        } else {
            *a.offset(i as isize) - k + dfs(dp, a, k, i + 1 as libc::c_int, size)
        };
        if (i + 1 as libc::c_int) < size
            && *a.offset((i + 1 as libc::c_int) as isize) <= k
        {
            if *a.offset((i + 1 as libc::c_int) as isize) + more >= k {
                ans = if ans < more + dfs(dp, a, k, i + 2 as libc::c_int, size) {
                    ans
                } else {
                    more + dfs(dp, a, k, i + 2 as libc::c_int, size)
                };
            } else {
                ans = if ans
                    < more + k - (*a.offset((i + 1 as libc::c_int) as isize) + more)
                        + dfs(dp, a, k, i + 2 as libc::c_int, size)
                {
                    ans
                } else {
                    more + k - (*a.offset((i + 1 as libc::c_int) as isize) + more)
                        + dfs(dp, a, k, i + 2 as libc::c_int, size)
                };
            }
        }
    } else {
        ans = if ans
            < k - *a.offset(i as isize) + dfs(dp, a, k, i + 1 as libc::c_int, size)
        {
            ans
        } else {
            k - *a.offset(i as isize) + dfs(dp, a, k, i + 1 as libc::c_int, size)
        };
        if (i + 1 as libc::c_int) < size
            && *a.offset((i + 1 as libc::c_int) as isize) <= k
        {
            if *a.offset((i + 1 as libc::c_int) as isize) + *a.offset(i as isize) >= k {
                ans = if ans
                    < *a.offset(i as isize) + dfs(dp, a, k, i + 2 as libc::c_int, size)
                {
                    ans
                } else {
                    *a.offset(i as isize) + dfs(dp, a, k, i + 2 as libc::c_int, size)
                };
            } else {
                ans = if ans
                    < *a.offset(i as isize)
                        + (k
                            - (*a.offset(i as isize)
                                + *a.offset((i + 1 as libc::c_int) as isize)))
                        + dfs(dp, a, k, i + 2 as libc::c_int, size)
                {
                    ans
                } else {
                    *a.offset(i as isize)
                        + (k
                            - (*a.offset(i as isize)
                                + *a.offset((i + 1 as libc::c_int) as isize)))
                        + dfs(dp, a, k, i + 2 as libc::c_int, size)
                };
            }
        }
    }
    *dp.offset(i as isize) = ans;
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn makeStringGood(mut s: *const libc::c_char) -> libc::c_int {
    let mut n: libc::c_int = strlen(s) as libc::c_int;
    let mut a: [libc::c_int; 26] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        a[(*s.offset(i as isize) as libc::c_int - 'a' as i32) as usize] += 1;
        a[(*s.offset(i as isize) as libc::c_int - 'a' as i32) as usize];
        i += 1;
        i;
    }
    let mut ans: libc::c_int = n;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 26 as libc::c_int {
        ans = if ans < n - a[i_0 as usize] { ans } else { n - a[i_0 as usize] };
        i_0 += 1;
        i_0;
    }
    let mut k: libc::c_int = 1 as libc::c_int;
    while k <= n {
        let mut dp: [libc::c_int; 26] = [0; 26];
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 26 as libc::c_int {
            dp[i_1 as usize] = -(1 as libc::c_int);
            i_1 += 1;
            i_1;
        }
        ans = if ans
            < dfs(
                dp.as_mut_ptr(),
                a.as_mut_ptr(),
                k,
                0 as libc::c_int,
                26 as libc::c_int,
            )
        {
            ans
        } else {
            dfs(dp.as_mut_ptr(), a.as_mut_ptr(), k, 0 as libc::c_int, 26 as libc::c_int)
        };
        k += 1;
        k;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 20001] = [0; 20001];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    let mut result: libc::c_int = makeStringGood(s.as_mut_ptr());
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
