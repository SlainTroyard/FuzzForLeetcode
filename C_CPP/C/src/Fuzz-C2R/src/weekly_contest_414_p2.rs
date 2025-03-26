use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maxPossibleScore(
    mut start: *mut libc::c_int,
    mut startSize: libc::c_int,
    mut d: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < startSize - 1 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < startSize - i - 1 as libc::c_int {
            if *start.offset(j as isize) > *start.offset((j + 1 as libc::c_int) as isize)
            {
                let mut temp: libc::c_int = *start.offset(j as isize);
                *start
                    .offset(j as isize) = *start.offset((j + 1 as libc::c_int) as isize);
                *start.offset((j + 1 as libc::c_int) as isize) = temp;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut left: libc::c_int = 0 as libc::c_int;
    let mut right: libc::c_int = (*start.offset((startSize - 1 as libc::c_int) as isize)
        + d - *start.offset(0 as libc::c_int as isize)) / (startSize - 1 as libc::c_int)
        + 1 as libc::c_int;
    while (left + 1 as libc::c_int) < right {
        let mut mid: libc::c_int = left + (right - left) / 2 as libc::c_int;
        let mut sum: libc::c_longlong = -(9223372036854775807 as libc::c_longlong)
            - 1 as libc::c_longlong;
        let mut valid: bool = 1 as libc::c_int != 0;
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < startSize {
            sum = fmax(
                (sum + mid as libc::c_longlong) as libc::c_double,
                *start.offset(i_0 as isize) as libc::c_longlong as libc::c_double,
            ) as libc::c_longlong;
            if sum > (*start.offset(i_0 as isize) + d) as libc::c_longlong {
                valid = 0 as libc::c_int != 0;
                break;
            } else {
                i_0 += 1;
                i_0;
            }
        }
        if valid {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
}
unsafe fn main_0() -> libc::c_int {
    let mut startSize: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut startSize as *mut libc::c_int,
        &mut d as *mut libc::c_int,
    );
    let mut start: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(startSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < startSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *start.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        maxPossibleScore(start, startSize, d),
    );
    free(start as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
