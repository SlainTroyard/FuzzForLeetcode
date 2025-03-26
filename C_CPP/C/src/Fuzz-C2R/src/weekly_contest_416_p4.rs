use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update(
    mut diff: *mut libc::c_int,
    mut c: libc::c_int,
    mut add: libc::c_int,
    mut cnt: *mut libc::c_int,
) {
    *diff.offset(c as isize) += add;
    if add == 1 as libc::c_int && *diff.offset(c as isize) == 0 as libc::c_int {
        *cnt -= 1;
        *cnt;
    } else if add == -(1 as libc::c_int)
        && *diff.offset(c as isize) == -(1 as libc::c_int)
    {
        *cnt += 1;
        *cnt;
    }
}
#[no_mangle]
pub unsafe extern "C" fn validSubstringCount(
    mut word1: *mut libc::c_char,
    mut word2: *mut libc::c_char,
) -> libc::c_longlong {
    let mut diff: [libc::c_int; 26] = [
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
    let mut c: *const libc::c_char = word2;
    while *c != 0 {
        diff[(*c as libc::c_int - 'a' as i32) as usize] -= 1;
        diff[(*c as libc::c_int - 'a' as i32) as usize];
        c = c.offset(1);
        c;
    }
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        if diff[i as usize] < 0 as libc::c_int {
            cnt += 1;
            cnt;
        }
        i += 1;
        i;
    }
    let mut res: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut len1: libc::c_int = strlen(word1) as libc::c_int;
    while l < len1 {
        while r < len1 && cnt > 0 as libc::c_int {
            update(
                diff.as_mut_ptr(),
                *word1.offset(r as isize) as libc::c_int - 'a' as i32,
                1 as libc::c_int,
                &mut cnt,
            );
            r += 1;
            r;
        }
        if cnt == 0 as libc::c_int {
            res += (len1 - r + 1 as libc::c_int) as libc::c_longlong;
        }
        update(
            diff.as_mut_ptr(),
            *word1.offset(l as isize) as libc::c_int - 'a' as i32,
            -(1 as libc::c_int),
            &mut cnt,
        );
        l += 1;
        l;
    }
    return res;
}
unsafe fn main_0() -> libc::c_int {
    let mut len1: libc::c_int = 0 as libc::c_int;
    let mut len2: libc::c_int = 0 as libc::c_int;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut len1 as *mut libc::c_int);
    let mut word1: *mut libc::c_char = malloc((len1 + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, word1);
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut len2 as *mut libc::c_int);
    let mut word2: *mut libc::c_char = malloc((len2 + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, word2);
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        validSubstringCount(word1, word2),
    );
    free(word1 as *mut libc::c_void);
    free(word2 as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
