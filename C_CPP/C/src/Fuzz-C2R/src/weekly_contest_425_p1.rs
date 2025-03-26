use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minimumSumSubarray(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut l: libc::c_int,
    mut r: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_int = 0;
    let mut minSum: libc::c_int = '\0' as i32;
    let mut count: libc::c_int = 0 as libc::c_int;
    while l <= r {
        sum = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < numsSize {
            sum += *nums.offset(i as isize);
            if i >= l {
                sum -= *nums.offset((i - l) as isize);
            }
            if sum > 0 as libc::c_int && i >= l - 1 as libc::c_int {
                if minSum == '\0' as i32 || minSum > sum {
                    minSum = sum;
                }
            }
            i += 1;
            i;
        }
        l += 1;
        l;
    }
    return if minSum == '\0' as i32 { -(1 as libc::c_int) } else { minSum };
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
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
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut l as *mut libc::c_int,
        &mut r as *mut libc::c_int,
    );
    let mut result: libc::c_int = minimumSumSubarray(nums, numsSize, l, r);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
