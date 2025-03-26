use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lengthAfterTransformations(
    mut s: *mut libc::c_char,
    mut t: libc::c_int,
) -> libc::c_int {
    let mut lst: [libc::c_int; 26] = [
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
    let mut z: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut ans: libc::c_int = 0 as libc::c_int;
    while *s != 0 {
        let fresh0 = s;
        s = s.offset(1);
        lst[(*fresh0 as libc::c_int - 'a' as i32) as usize] += 1;
        lst[(*fresh0 as libc::c_int - 'a' as i32) as usize];
    }
    loop {
        let fresh1 = t;
        t = t - 1;
        if !(fresh1 != 0) {
            break;
        }
        i = 25 as libc::c_int;
        z = lst[25 as libc::c_int as usize];
        while i > 1 as libc::c_int {
            lst[i as usize] = lst[(i - 1 as libc::c_int) as usize];
            i -= 1;
            i;
        }
        lst[1 as libc::c_int
            as usize] = (lst[0 as libc::c_int as usize] + z) % 1000000007 as libc::c_int;
        lst[0 as libc::c_int as usize] = z;
    }
    i = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        ans = (ans + lst[i as usize]) % 1000000007 as libc::c_int;
        i += 1;
        i;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100000] = [0; 100000];
    let mut t: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut t as *mut libc::c_int);
    printf(
        b"%d\0" as *const u8 as *const libc::c_char,
        lengthAfterTransformations(s.as_mut_ptr(), t),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
