use ::libc;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn countValidSelections(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut sumLeft: libc::c_int = 0 as libc::c_int;
    let mut sumRight: libc::c_int = 0 as libc::c_int;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        sumRight += *nums.offset(i as isize);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numsSize {
        if *nums.offset(i_0 as isize) != 0 as libc::c_int {
            sumLeft += *nums.offset(i_0 as isize);
            sumRight -= *nums.offset(i_0 as isize);
        } else if sumLeft == sumRight {
            ret += 2 as libc::c_int;
        } else if abs(sumLeft - sumRight) == 1 as libc::c_int {
            ret += 1;
            ret;
        }
        i_0 += 1;
        i_0;
    }
    return ret;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
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
    let mut result: libc::c_int = countValidSelections(nums.as_mut_ptr(), n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
