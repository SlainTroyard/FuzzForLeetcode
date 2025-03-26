use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn log2(_: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn smallestNumber(mut n: libc::c_int) -> libc::c_int {
    let mut b: libc::c_int = log2(n as libc::c_double) as libc::c_int + 1 as libc::c_int;
    return ((1 as libc::c_int) << b) - 1 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut result: libc::c_int = smallestNumber(n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
