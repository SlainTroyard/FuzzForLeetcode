use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkTwoChessboards(
    mut coordinate1: *mut libc::c_char,
    mut coordinate2: *mut libc::c_char,
) -> bool {
    return (*coordinate1.offset(0 as libc::c_int as isize) as libc::c_int
        - *coordinate2.offset(0 as libc::c_int as isize) as libc::c_int
        + *coordinate1.offset(1 as libc::c_int as isize) as libc::c_int
        - *coordinate2.offset(1 as libc::c_int as isize) as libc::c_int)
        % 2 as libc::c_int == 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut coordinate1: [libc::c_char; 3] = [0; 3];
    let mut coordinate2: [libc::c_char; 3] = [0; 3];
    scanf(
        b"%s %s\0" as *const u8 as *const libc::c_char,
        coordinate1.as_mut_ptr(),
        coordinate2.as_mut_ptr(),
    );
    if checkTwoChessboards(coordinate1.as_mut_ptr(), coordinate2.as_mut_ptr()) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
