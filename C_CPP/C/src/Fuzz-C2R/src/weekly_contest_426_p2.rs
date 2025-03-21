use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getLargestOutlier(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut total_sum: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        total_sum += *nums.offset(i as isize);
        i += 1;
        i;
    }
    let mut set: [libc::c_int; 2001] = [0; 2001];
    memset(
        &mut set as *mut [libc::c_int; 2001] as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 2001]>() as libc::c_ulong,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numsSize {
        set[(*nums.offset(i_0 as isize) + 1000 as libc::c_int) as usize] += 1;
        set[(*nums.offset(i_0 as isize) + 1000 as libc::c_int) as usize];
        i_0 += 1;
        i_0;
    }
    let mut curr_sum: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    let mut half: libc::c_int = 0;
    let mut ans: libc::c_int = -(1001 as libc::c_int);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < numsSize {
        curr_sum = total_sum - *nums.offset(i_1 as isize);
        if curr_sum & 1 as libc::c_int == 0 as libc::c_int {
            half = curr_sum / 2 as libc::c_int;
            if half == *nums.offset(i_1 as isize) {
                threshold = 1 as libc::c_int;
            } else {
                threshold = 0 as libc::c_int;
            }
            if half >= -(1000 as libc::c_int) && half <= 1000 as libc::c_int {
                if set[(half + 1000 as libc::c_int) as usize] > threshold {
                    if ans < *nums.offset(i_1 as isize) {
                        ans = *nums.offset(i_1 as isize);
                    }
                }
            }
        }
        i_1 += 1;
        i_1;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
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
    let mut result: libc::c_int = getLargestOutlier(nums, numsSize);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
