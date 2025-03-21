use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn min(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn max(
    mut a: libc::c_int,
    mut b: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_int = if a > b { a } else { b };
    return if d > c { d } else { c };
}
#[no_mangle]
pub unsafe extern "C" fn maxIncreasingSubarrays(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut maxlen: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    let mut max1: libc::c_int = 1 as libc::c_int;
    while i < numsSize {
        let mut max2: libc::c_int = 1 as libc::c_int;
        while i < numsSize
            && *nums.offset(i as isize) > *nums.offset((i - 1 as libc::c_int) as isize)
        {
            max2 += 1;
            max2;
            i += 1;
            i;
        }
        let mut temp: libc::c_int = min(max1, max2);
        maxlen = max(maxlen, temp, max2 / 2 as libc::c_int);
        max1 = max2;
        i += 1;
        i;
    }
    return maxlen;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let vla = numsSize as usize;
    let mut nums: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = maxIncreasingSubarrays(nums.as_mut_ptr(), numsSize);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
