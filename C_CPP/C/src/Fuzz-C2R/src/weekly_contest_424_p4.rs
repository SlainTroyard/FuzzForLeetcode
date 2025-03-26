use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minDifference(
    mut nums: *mut libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut min_l: libc::c_int = 2147483647 as libc::c_int;
    let mut max_r: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if *nums.offset(i as isize) != -(1 as libc::c_int)
            && (i > 0 as libc::c_int
                && *nums.offset((i - 1 as libc::c_int) as isize) == -(1 as libc::c_int)
                || i < n - 1 as libc::c_int
                    && *nums.offset((i + 1 as libc::c_int) as isize)
                        == -(1 as libc::c_int))
        {
            if *nums.offset(i as isize) < min_l {
                min_l = *nums.offset(i as isize);
            }
            if *nums.offset(i as isize) > max_r {
                max_r = *nums.offset(i as isize);
            }
        }
        i += 1;
        i;
    }
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut pre_i: libc::c_int = -(1 as libc::c_int);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        if !(*nums.offset(i_0 as isize) == -(1 as libc::c_int)) {
            if pre_i >= 0 as libc::c_int {
                if i_0 - pre_i == 1 as libc::c_int {
                    let mut diff: libc::c_int = abs(
                        *nums.offset(i_0 as isize) - *nums.offset(pre_i as isize),
                    );
                    if diff > ans {
                        ans = diff;
                    }
                } else {
                    update_ans(
                        if *nums.offset(pre_i as isize) < *nums.offset(i_0 as isize) {
                            *nums.offset(pre_i as isize)
                        } else {
                            *nums.offset(i_0 as isize)
                        },
                        if *nums.offset(pre_i as isize) > *nums.offset(i_0 as isize) {
                            *nums.offset(pre_i as isize)
                        } else {
                            *nums.offset(i_0 as isize)
                        },
                        (i_0 - pre_i > 2 as libc::c_int) as libc::c_int,
                    );
                }
            } else if i_0 > 0 as libc::c_int {
                update_ans(
                    *nums.offset(i_0 as isize),
                    *nums.offset(i_0 as isize),
                    0 as libc::c_int,
                );
            }
            pre_i = i_0;
        }
        i_0 += 1;
        i_0;
    }
    if 0 as libc::c_int <= pre_i && pre_i < n - 1 as libc::c_int {
        update_ans(
            *nums.offset(pre_i as isize),
            *nums.offset(pre_i as isize),
            0 as libc::c_int,
        );
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
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
    let mut result: libc::c_int = minDifference(nums, n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
