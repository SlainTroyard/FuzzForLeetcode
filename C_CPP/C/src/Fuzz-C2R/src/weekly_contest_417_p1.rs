use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kthCharacter(mut k: libc::c_int) -> libc::c_char {
    let mut l: *mut libc::c_int = calloc(
        (k + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut a: libc::c_int = 0 as libc::c_int;
    loop {
        let mut i: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = (1 as libc::c_int) << a;
        while i < (1 as libc::c_int) << a {
            *l.offset((i + j) as isize) = *l.offset(i as isize) + 1 as libc::c_int;
            if i + j >= k - 1 as libc::c_int {
                return (97 as libc::c_int
                    + *l.offset((k - 1 as libc::c_int) as isize) % 26 as libc::c_int)
                    as libc::c_char;
            }
            i += 1;
            i;
        }
        a += 1;
        a;
    };
}
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_int = 0 as libc::c_int;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    printf(
        b"%c\n\0" as *const u8 as *const libc::c_char,
        kthCharacter(k) as libc::c_int,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
