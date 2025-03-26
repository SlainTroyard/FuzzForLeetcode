use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn buttonWithLongestTime(
    mut events: *mut *mut libc::c_int,
    mut eventsSize: libc::c_int,
    mut eventsColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut longest_time: libc::c_int = *(*events.offset(0 as libc::c_int as isize))
        .offset(1 as libc::c_int as isize);
    let mut prev: libc::c_int = *(*events.offset(0 as libc::c_int as isize))
        .offset(1 as libc::c_int as isize);
    let mut longest_index: libc::c_int = *(*events.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i != eventsSize {
        let current_press: libc::c_int = *(*events.offset(i as isize))
            .offset(1 as libc::c_int as isize);
        let current_time: libc::c_int = current_press - prev;
        if current_time > longest_time
            || current_time == longest_time
                && *(*events.offset(i as isize)).offset(0 as libc::c_int as isize)
                    < longest_index
        {
            longest_time = current_time;
            longest_index = *(*events.offset(i as isize))
                .offset(0 as libc::c_int as isize);
        }
        prev = current_press;
        i += 1;
        i;
    }
    return longest_index;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut events: *mut *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *events.offset(i as isize);
        *fresh0 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*events.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*events.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut eventsColSize: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        *eventsColSize.offset(i_0 as isize) = 2 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    let mut result: libc::c_int = buttonWithLongestTime(events, n, eventsColSize);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n {
        free(*events.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(events as *mut libc::c_void);
    free(eventsColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
