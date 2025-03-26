use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isBalanced(mut num: *mut libc::c_char) -> bool {
    let mut total: libc::c_int = 0 as libc::c_int;
    while *num != 0 {
        total += '0' as i32 - *num as libc::c_int;
        total = -total;
        num = num.offset(1);
        num;
    }
    return total == 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut num: [libc::c_char; 101] = [0; 101];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, num.as_mut_ptr());
    if isBalanced(num.as_mut_ptr()) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
