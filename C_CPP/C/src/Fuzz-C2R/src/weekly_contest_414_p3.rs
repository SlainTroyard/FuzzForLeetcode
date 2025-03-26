use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findMaximumScore(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_longlong {
    let mut ans: libc::c_longlong = 0 as libc::c_longlong;
    let mut l: libc::c_longlong = 0 as libc::c_longlong;
    let mut r: libc::c_longlong = 1 as libc::c_longlong;
    while r < numsSize as libc::c_longlong {
        if *nums.offset(l as isize) < *nums.offset(r as isize) {
            ans += (r - l) * *nums.offset(l as isize) as libc::c_longlong;
            l = r;
        }
        r += 1;
        r;
    }
    return ans
        + (r - l - 1 as libc::c_int as libc::c_longlong)
            * *nums.offset(l as isize) as libc::c_longlong;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        findMaximumScore(nums, numsSize),
    );
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
