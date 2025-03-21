use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn maxDifference(mut s: *mut libc::c_char) -> libc::c_int {
    let mut cnt: [libc::c_int; 26] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut c1: libc::c_int = 0 as libc::c_int;
    let mut c2: libc::c_int = 100 as libc::c_int;
    while *s != 0 {
        let fresh0 = s;
        s = s.offset(1);
        cnt[(*fresh0 as libc::c_int - 97 as libc::c_int) as usize] += 1;
        cnt[(*fresh0 as libc::c_int - 97 as libc::c_int) as usize];
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        if cnt[i as usize] & 1 as libc::c_int != 0 {
            c1 = if cnt[i as usize] > c1 { cnt[i as usize] } else { c1 };
        } else if cnt[i as usize] != 0 as libc::c_int {
            c2 = if cnt[i as usize] < c2 { cnt[i as usize] } else { c2 };
        }
        i += 1;
        i;
    }
    return c1 - c2;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100] = [0; 100];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, maxDifference(s.as_mut_ptr()));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
