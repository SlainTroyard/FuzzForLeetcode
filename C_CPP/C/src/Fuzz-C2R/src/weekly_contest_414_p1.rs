use ::libc;
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn convertDateToBinary(
    mut date: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut ans: *mut libc::c_char = calloc(
        25 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    let mut ptr: *mut libc::c_char = ans.offset(24 as libc::c_int as isize);
    let mut d: libc::c_int = atoi(date.offset(8 as libc::c_int as isize));
    if d == 0 as libc::c_int {
        ptr = ptr.offset(-1);
        *ptr = '0' as i32 as libc::c_char;
    } else {
        while d != 0 {
            ptr = ptr.offset(-1);
            *ptr = ((d & 1 as libc::c_int) + '0' as i32) as libc::c_char;
            d >>= 1 as libc::c_int;
        }
    }
    ptr = ptr.offset(-1);
    *ptr = '-' as i32 as libc::c_char;
    let mut m: libc::c_int = atoi(date.offset(5 as libc::c_int as isize));
    if m == 0 as libc::c_int {
        ptr = ptr.offset(-1);
        *ptr = '0' as i32 as libc::c_char;
    } else {
        while m != 0 {
            ptr = ptr.offset(-1);
            *ptr = ((m & 1 as libc::c_int) + '0' as i32) as libc::c_char;
            m >>= 1 as libc::c_int;
        }
    }
    ptr = ptr.offset(-1);
    *ptr = '-' as i32 as libc::c_char;
    let mut y: libc::c_int = atoi(date);
    if y == 0 as libc::c_int {
        ptr = ptr.offset(-1);
        *ptr = '0' as i32 as libc::c_char;
    } else {
        while y != 0 {
            ptr = ptr.offset(-1);
            *ptr = ((y & 1 as libc::c_int) + '0' as i32) as libc::c_char;
            y >>= 1 as libc::c_int;
        }
    }
    let mut len: size_t = ans.offset(25 as libc::c_int as isize).offset_from(ptr)
        as libc::c_long as size_t;
    memmove(ans as *mut libc::c_void, ptr as *const libc::c_void, len);
    *ans.offset(len as isize) = '\0' as i32 as libc::c_char;
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut date: [libc::c_char; 11] = [0; 11];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, date.as_mut_ptr());
    let mut ans: *mut libc::c_char = convertDateToBinary(date.as_mut_ptr());
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, ans);
    free(ans as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
