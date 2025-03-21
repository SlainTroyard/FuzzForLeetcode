use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stringSequence(
    mut target: *mut libc::c_char,
    mut returnSize: *mut libc::c_int,
) -> *mut *mut libc::c_char {
    *returnSize = 0 as libc::c_int;
    let mut t: *mut libc::c_char = target;
    while *t != 0 {
        *returnSize += *t as libc::c_int - 96 as libc::c_int;
        t = t.offset(1);
        t;
    }
    let mut ans: *mut *mut libc::c_char = calloc(
        *returnSize as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
    ) as *mut *mut libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        as libc::c_int;
    let mut t_0: *mut libc::c_char = target;
    while *t_0 != 0 {
        let mut c: libc::c_char = 'a' as i32 as libc::c_char;
        while c as libc::c_int <= *t_0 as libc::c_int {
            let ref mut fresh0 = *ans.offset(i as isize);
            *fresh0 = calloc(
                (l + 2 as libc::c_int) as libc::c_ulong,
                size as libc::c_ulong,
            ) as *mut libc::c_char;
            strncpy(*ans.offset(i as isize), target, l as libc::c_ulong);
            let fresh1 = i;
            i = i + 1;
            *(*ans.offset(fresh1 as isize)).offset(l as isize) = c;
            c += 1;
            c;
        }
        t_0 = t_0.offset(1);
        t_0;
        l += 1;
        l;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut target: [libc::c_char; 100] = [0; 100];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, target.as_mut_ptr());
    let mut returnSize: libc::c_int = 0;
    let mut ans: *mut *mut libc::c_char = stringSequence(
        target.as_mut_ptr(),
        &mut returnSize,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < returnSize {
        printf(b"%s \0" as *const u8 as *const libc::c_char, *ans.offset(i as isize));
        free(*ans.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(ans as *mut libc::c_void);
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
