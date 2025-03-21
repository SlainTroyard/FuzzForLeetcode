use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn zhuanhuan(
    mut point: *mut libc::c_int,
    mut side: libc::c_int,
) -> libc::c_longlong {
    if *point.offset(0 as libc::c_int as isize)
        >= *point.offset(1 as libc::c_int as isize)
    {
        return (*point.offset(0 as libc::c_int as isize)
            + *point.offset(1 as libc::c_int as isize)) as libc::c_longlong
    } else {
        let mut m: libc::c_longlong = side as libc::c_longlong;
        return 4 as libc::c_int as libc::c_longlong * m
            - (*point.offset(0 as libc::c_int as isize)
                + *point.offset(1 as libc::c_int as isize)) as libc::c_longlong;
    };
}
#[no_mangle]
pub unsafe extern "C" fn compar(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut val_a: libc::c_longlong = *(a as *const libc::c_longlong);
    let mut val_b: libc::c_longlong = *(b as *const libc::c_longlong);
    if val_a < val_b {
        return -(1 as libc::c_int);
    }
    if val_a > val_b {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn search(
    mut val: libc::c_longlong,
    mut r: *mut libc::c_longlong,
    mut pointsSize: libc::c_int,
    mut side: libc::c_int,
) -> libc::c_longlong {
    let mut m: libc::c_longlong = side as libc::c_longlong;
    let mut val1: libc::c_longlong = val % (4 as libc::c_int as libc::c_longlong * m);
    if val1 > *r.offset((pointsSize - 1 as libc::c_int) as isize) {
        return val - val1 + 4 as libc::c_int as libc::c_longlong * m
            + *r.offset(0 as libc::c_int as isize);
    }
    if val1 <= *r.offset(0 as libc::c_int as isize) {
        return val - val1 + *r.offset(0 as libc::c_int as isize);
    }
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = pointsSize - 1 as libc::c_int;
    while min < max - 1 as libc::c_int {
        let mut s: libc::c_int = (max + min) / 2 as libc::c_int;
        if *r.offset(s as isize) >= val1 {
            max = s;
        } else {
            min = s;
        }
    }
    return *r.offset(max as isize) + val - val1;
}
#[no_mangle]
pub unsafe extern "C" fn build(
    mut r: *mut libc::c_longlong,
    mut s: libc::c_int,
    mut k: libc::c_int,
    mut side: libc::c_int,
    mut pointsSize: libc::c_int,
) -> bool {
    let mut sum: libc::c_longlong = 0;
    let mut th: libc::c_longlong = 0;
    let mut val: libc::c_longlong = 0;
    let mut max_th: libc::c_longlong = 0;
    let mut m: libc::c_longlong = side as libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < pointsSize - 1 as libc::c_int {
        sum = 1 as libc::c_int as libc::c_longlong;
        th = *r.offset(i as isize);
        max_th = *r.offset(i as isize) + 4 as libc::c_int as libc::c_longlong * m
            - s as libc::c_longlong;
        while th <= max_th {
            if sum == k as libc::c_longlong {
                return 1 as libc::c_int != 0;
            }
            val = th + s as libc::c_longlong;
            th = search(val, r, pointsSize, side);
            sum += 1;
            sum;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn maxDistance(
    mut side: libc::c_int,
    mut points: *mut *mut libc::c_int,
    mut pointsSize: libc::c_int,
    mut pointsColSize: *mut libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut r: *mut libc::c_longlong = malloc(
        (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
            .wrapping_mul(pointsSize as libc::c_ulong),
    ) as *mut libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < pointsSize {
        *r.offset(i as isize) = zhuanhuan(*points.offset(i as isize), side);
        i += 1;
        i;
    }
    qsort(
        r as *mut libc::c_void,
        pointsSize as size_t,
        ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
        Some(
            compar
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut min: libc::c_int = 1 as libc::c_int;
    let mut max: libc::c_int = side + 1 as libc::c_int;
    let mut s: libc::c_int = 0;
    while min < max - 1 as libc::c_int {
        s = (min + max) / 2 as libc::c_int;
        if build(r, s, k, side, pointsSize) {
            min = s;
        } else {
            max = s;
        }
    }
    return min;
}
unsafe fn main_0() -> libc::c_int {
    let mut side: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        &mut side as *mut libc::c_int,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut points: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *points.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*points.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*points.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut pointsColSize: libc::c_int = 2 as libc::c_int;
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        maxDistance(side, points, n, &mut pointsColSize, k),
    );
    free(points as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
