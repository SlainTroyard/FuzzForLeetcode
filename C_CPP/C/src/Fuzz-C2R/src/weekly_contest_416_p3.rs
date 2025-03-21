use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get(
    mut l: libc::c_int,
    mut r: libc::c_int,
    mut preCount: *mut [libc::c_int; 26],
    mut count: *mut libc::c_int,
) -> libc::c_int {
    let mut border: libc::c_int = l;
    while l < r {
        let mut m: libc::c_int = l + r >> 1 as libc::c_int;
        let mut f: libc::c_int = 1 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 26 as libc::c_int {
            if (*preCount.offset(m as isize))[i as usize]
                - (*preCount.offset((border - 1 as libc::c_int) as isize))[i as usize]
                < *count.offset(i as isize)
            {
                f = 0 as libc::c_int;
                break;
            } else {
                i += 1;
                i;
            }
        }
        if f != 0 {
            r = m;
        } else {
            l = m + 1 as libc::c_int;
        }
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn validSubstringCount(
    mut word1: *mut libc::c_char,
    mut word2: *mut libc::c_char,
) -> libc::c_longlong {
    let mut count: [libc::c_int; 26] = [
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
    let mut i: libc::c_int = 0 as libc::c_int;
    while *word2.offset(i as isize) != 0 {
        count[(*word2.offset(i as isize) as libc::c_int - 'a' as i32) as usize] += 1;
        count[(*word2.offset(i as isize) as libc::c_int - 'a' as i32) as usize];
        i += 1;
        i;
    }
    let mut n: libc::c_int = strlen(word1) as libc::c_int;
    let vla = (n + 1 as libc::c_int) as usize;
    let mut preCount: Vec::<[libc::c_int; 26]> = ::std::vec::from_elem([0; 26], vla);
    memset(
        preCount.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla * ::core::mem::size_of::<[libc::c_int; 26]>()) as libc::c_ulong,
    );
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 <= n {
        memcpy(
            (*preCount.as_mut_ptr().offset(i_0 as isize)).as_mut_ptr()
                as *mut libc::c_void,
            (*preCount.as_mut_ptr().offset((i_0 - 1 as libc::c_int) as isize))
                .as_mut_ptr() as *const libc::c_void,
            ::core::mem::size_of::<[libc::c_int; 26]>() as libc::c_ulong,
        );
        let ref mut fresh0 = (*preCount
            .as_mut_ptr()
            .offset(
                i_0 as isize,
            ))[(*word1.offset((i_0 - 1 as libc::c_int) as isize) as libc::c_int
            - 'a' as i32) as usize];
        *fresh0 += 1;
        *fresh0;
        i_0 += 1;
        i_0;
    }
    let mut res: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut l: libc::c_int = 1 as libc::c_int;
    while l <= n {
        let mut r: libc::c_int = get(
            l,
            n + 1 as libc::c_int,
            preCount.as_mut_ptr(),
            count.as_mut_ptr(),
        );
        res += (n - r + 1 as libc::c_int) as libc::c_longlong;
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
