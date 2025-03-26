use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn countPartitions(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize - 1 as libc::c_int {
        let mut leftSum: libc::c_int = 0 as libc::c_int;
        let mut rightSum: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j <= i {
            leftSum += *nums.offset(j as isize);
            j += 1;
            j;
        }
        let mut j_0: libc::c_int = i + 1 as libc::c_int;
        while j_0 < numsSize {
            rightSum += *nums.offset(j_0 as isize);
            j_0 += 1;
            j_0;
        }
        if (leftSum - rightSum) % 2 as libc::c_int == 0 as libc::c_int {
            count += 1;
            count;
        }
        i += 1;
        i;
    }
    return count;
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
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, countPartitions(nums, n));
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
