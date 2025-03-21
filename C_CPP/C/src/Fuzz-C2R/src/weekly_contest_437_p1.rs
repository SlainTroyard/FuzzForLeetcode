use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hasSpecialSubstring(
    mut s: *mut libc::c_char,
    mut k: libc::c_int,
) -> bool {
    let mut length: libc::c_int = strlen(s) as libc::c_int;
    let mut consecutive: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        consecutive += 1;
        consecutive;
        if i == length - 1 as libc::c_int
            || *s.offset(i as isize) as libc::c_int
                != *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int
        {
            if consecutive == k {
                return 1 as libc::c_int != 0
            } else {
                consecutive = 0 as libc::c_int;
            }
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100] = [0; 100];
    let mut k: libc::c_int = 0;
    scanf(
        b"%s %d\0" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
        &mut k as *mut libc::c_int,
    );
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        hasSpecialSubstring(s.as_mut_ptr(), k) as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
