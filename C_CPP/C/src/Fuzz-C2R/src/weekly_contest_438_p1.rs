use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hasSameDigits(mut s: *mut libc::c_char) -> bool {
    let mut len: libc::c_int = strlen(s) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let fresh0 = i;
        i = i + 1;
        let ref mut fresh1 = *s.offset(fresh0 as isize);
        *fresh1 = (*fresh1 as libc::c_int & 0xf as libc::c_int) as libc::c_char;
    }
    loop {
        len -= 1;
        if !(len > 1 as libc::c_int) {
            break;
        }
        i = 0 as libc::c_int;
        while i < len {
            *s
                .offset(
                    i as isize,
                ) = ((*s.offset(i as isize) as libc::c_int
                + *s.offset((i + 1 as libc::c_int) as isize) as libc::c_int)
                % 10 as libc::c_int) as libc::c_char;
            i += 1;
            i;
        }
    }
    return *s.offset(0 as libc::c_int as isize) as libc::c_int
        == *s.offset(1 as libc::c_int as isize) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100] = [0; 100];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        hasSameDigits(s.as_mut_ptr()) as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
