use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cal_time(
    mut mountainHeight: libc::c_longlong,
    mut workerTimes: *mut libc::c_int,
    mut workerTimesSize: libc::c_int,
    mut target_time: libc::c_longlong,
) -> bool {
    let mut l_height: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut r_height: libc::c_longlong = 1e6f64 as libc::c_longlong;
    let mut mid_height: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut use_time: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < workerTimesSize {
        l_height = 0 as libc::c_int as libc::c_longlong;
        r_height = 1e6f64 as libc::c_longlong;
        while r_height >= l_height {
            mid_height = (l_height + r_height) / 2 as libc::c_int as libc::c_longlong;
            use_time = mid_height
                * (*workerTimes.offset(i as isize) as libc::c_longlong
                    + mid_height * *workerTimes.offset(i as isize) as libc::c_longlong)
                / 2 as libc::c_int as libc::c_longlong;
            if use_time > target_time {
                r_height = mid_height - 1 as libc::c_int as libc::c_longlong;
            } else {
                l_height = mid_height + 1 as libc::c_int as libc::c_longlong;
            }
        }
        mountainHeight -= r_height;
        i += 1;
        i;
    }
    return if mountainHeight > 0 as libc::c_int as libc::c_longlong {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn minNumberOfSeconds(
    mut mountainHeight: libc::c_int,
    mut workerTimes: *mut libc::c_int,
    mut workerTimesSize: libc::c_int,
) -> libc::c_longlong {
    let mut r_time: libc::c_longlong = 1e18f64 as libc::c_longlong;
    let mut l_time: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut mid_time: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    while r_time >= l_time {
        mid_time = (r_time + l_time) / 2 as libc::c_int as libc::c_longlong;
        if cal_time(
            mountainHeight as libc::c_longlong,
            workerTimes,
            workerTimesSize,
            mid_time,
        ) {
            r_time = mid_time - 1 as libc::c_int as libc::c_longlong;
        } else {
            l_time = mid_time + 1 as libc::c_int as libc::c_longlong;
        }
    }
    r_time += 1;
    return r_time;
}
unsafe fn main_0() -> libc::c_int {
    let mut mountainHeight: libc::c_int = 0;
    let mut workerTimesSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut mountainHeight as *mut libc::c_int,
        &mut workerTimesSize as *mut libc::c_int,
    );
    let mut workerTimes: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(workerTimesSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < workerTimesSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *workerTimes.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        minNumberOfSeconds(mountainHeight, workerTimes, workerTimesSize),
    );
    free(workerTimes as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
