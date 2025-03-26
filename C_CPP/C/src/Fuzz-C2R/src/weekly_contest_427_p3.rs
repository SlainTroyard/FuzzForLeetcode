use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn maxValue(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn maxSubarraySum(
    mut nums: *mut libc::c_int,
    mut n: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut cur: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let vla = (n - k + 1 as libc::c_int) as usize;
    let mut keep: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        cur += *nums.offset(i as isize) as libc::c_longlong;
        if i >= k - 1 as libc::c_int {
            *keep.as_mut_ptr().offset((i - k + 1 as libc::c_int) as isize) = cur;
            cur -= *nums.offset((i - k + 1 as libc::c_int) as isize) as libc::c_longlong;
        }
        i += 1;
        i;
    }
    let mut ans: libc::c_longlong = (-(9223372036854775807 as libc::c_long)
        - 1 as libc::c_long) as libc::c_longlong;
    let mut max: libc::c_longlong = 0;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < k && i_0 < n - k + 1 as libc::c_int {
        cur = 0 as libc::c_int as libc::c_longlong;
        max = *keep.as_mut_ptr().offset(i_0 as isize);
        let mut l: libc::c_int = i_0;
        while l < n - k + 1 as libc::c_int {
            cur += *keep.as_mut_ptr().offset(l as isize);
            if cur > max {
                max = cur;
            }
            if cur < 0 as libc::c_int as libc::c_longlong {
                cur = 0 as libc::c_int as libc::c_longlong;
            }
            l += k;
        }
        ans = maxValue(ans, max);
        i_0 += 1;
        i_0;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    let vla = n as usize;
    let mut nums: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_longlong = maxSubarraySum(nums.as_mut_ptr(), n, k);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
