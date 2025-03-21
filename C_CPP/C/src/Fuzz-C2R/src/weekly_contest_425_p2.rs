use ::libc;
extern "C" {
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub static mut LEN: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return strncmp(a as *mut libc::c_char, b as *mut libc::c_char, LEN as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn isPossibleToRearrange(
    mut s: *mut libc::c_char,
    mut t: *mut libc::c_char,
    mut k: libc::c_int,
) -> bool {
    LEN = (strlen(s)).wrapping_div(k as libc::c_ulong) as libc::c_int;
    qsort(
        s as *mut libc::c_void,
        k as size_t,
        (LEN as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    qsort(
        t as *mut libc::c_void,
        k as size_t,
        (LEN as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    return strcmp(s, t) == 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 200001] = [0; 200001];
    let mut t: [libc::c_char; 200001] = [0; 200001];
    let mut k: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    scanf(b"%s\0" as *const u8 as *const libc::c_char, t.as_mut_ptr());
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    if isPossibleToRearrange(s.as_mut_ptr(), t.as_mut_ptr(), k) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
