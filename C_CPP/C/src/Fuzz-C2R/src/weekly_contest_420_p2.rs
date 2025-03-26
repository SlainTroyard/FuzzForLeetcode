use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn numberOfSubstrings(
    mut s: *mut libc::c_char,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut hash_arr: [libc::c_int; 26] = [
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
    let mut left: libc::c_int = 0 as libc::c_int;
    let mut right: libc::c_int = 0 as libc::c_int;
    let mut s_l: libc::c_int = strlen(s) as libc::c_int;
    let mut res: libc::c_int = 0 as libc::c_int;
    while left < s_l && right < s_l {
        hash_arr[(*s.offset(right as isize) as libc::c_int - 'a' as i32) as usize] += 1;
        if hash_arr[(*s.offset(right as isize) as libc::c_int - 'a' as i32) as usize]
            == k
        {
            res += s_l - right;
            while left <= right {
                let fresh0 = left;
                left = left + 1;
                hash_arr[(*s.offset(fresh0 as isize) as libc::c_int - 'a' as i32)
                    as usize] -= 1;
                if !(hash_arr[(*s.offset(fresh0 as isize) as libc::c_int - 'a' as i32)
                    as usize] != k - 1 as libc::c_int)
                {
                    break;
                }
                res += s_l - right;
            }
            right += 1;
            right;
        } else {
            right += 1;
            right;
        }
    }
    return res;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 3000] = [0; 3000];
    let mut k: libc::c_int = 0;
    scanf(
        b"%s %d\0" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
        &mut k as *mut libc::c_int,
    );
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        numberOfSubstrings(s.as_mut_ptr(), k),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
