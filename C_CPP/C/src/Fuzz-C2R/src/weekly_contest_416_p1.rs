use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn bsearch(
        __key: *const libc::c_void,
        __base: *const libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    ) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return strcmp(*(a as *mut *mut libc::c_char), *(b as *mut *mut libc::c_char));
}
#[no_mangle]
pub unsafe extern "C" fn reportSpam(
    mut message: *mut *mut libc::c_char,
    mut messageSize: libc::c_int,
    mut bannedWords: *mut *mut libc::c_char,
    mut bannedWordsSize: libc::c_int,
) -> bool {
    qsort(
        bannedWords as *mut libc::c_void,
        bannedWordsSize as size_t,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < messageSize {
        if !(bsearch(
            &mut *message.offset(i as isize) as *mut *mut libc::c_char
                as *const libc::c_void,
            bannedWords as *const libc::c_void,
            bannedWordsSize as size_t,
            ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            Some(
                cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        ))
            .is_null()
        {
            cnt += 1;
            cnt;
        }
        if cnt >= 2 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut messageSize: libc::c_int = 0;
    let mut bannedWordsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut messageSize as *mut libc::c_int,
    );
    let mut message: *mut *mut libc::c_char = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(messageSize as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < messageSize {
        let ref mut fresh0 = *message.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        scanf(b"%s\0" as *const u8 as *const libc::c_char, *message.offset(i as isize));
        i += 1;
        i;
    }
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut bannedWordsSize as *mut libc::c_int,
    );
    let mut bannedWords: *mut *mut libc::c_char = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(bannedWordsSize as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < bannedWordsSize {
        let ref mut fresh1 = *bannedWords.offset(i_0 as isize);
        *fresh1 = malloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(100 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        scanf(
            b"%s\0" as *const u8 as *const libc::c_char,
            *bannedWords.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    if reportSpam(message, messageSize, bannedWords, bannedWordsSize) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
    free(message as *mut libc::c_void);
    free(bannedWords as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
