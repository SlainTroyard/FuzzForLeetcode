use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn minOperations(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut max_factor: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    if 1 as libc::c_int == numsSize {
        return res;
    }
    let mut i: libc::c_int = numsSize - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *nums.offset(i as isize) > *nums.offset((i + 1 as libc::c_int) as isize) {
            let mut max: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            let mut count: libc::c_int = 1 as libc::c_int;
            let mut original: libc::c_int = *nums.offset(i as isize);
            loop {
                max = -(2147483647 as libc::c_int) - 1 as libc::c_int;
                max_factor = (sqrt(*nums.offset(i as isize) as libc::c_double)
                    + 1 as libc::c_int as libc::c_double) as libc::c_int;
                j = 2 as libc::c_int;
                while j <= max_factor {
                    if *nums.offset(i as isize) % j == 0 {
                        max = if max > j { max } else { j };
                        if *nums.offset(i as isize) % (*nums.offset(i as isize) / j) == 0
                        {
                            max = if max > *nums.offset(i as isize) / j {
                                max
                            } else {
                                *nums.offset(i as isize) / j
                            };
                        }
                    }
                    j += 1;
                    j;
                }
                if max == -(2147483647 as libc::c_int) - 1 as libc::c_int
                    || {
                        count *= max;
                        count == original
                    }
                {
                    return -(1 as libc::c_int)
                } else {
                    *nums.offset(i as isize) /= max;
                    if !(*nums.offset(i as isize)
                        <= *nums.offset((i + 1 as libc::c_int) as isize))
                    {
                        continue;
                    }
                    res += 1;
                    res;
                    break;
                }
            }
        }
        i -= 1;
        i;
    }
    return res;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0 as libc::c_int;
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
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, minOperations(nums, numsSize));
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
