use ::libc;
extern "C" {
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
pub unsafe extern "C" fn get(
    mut points: *mut [libc::c_int; 2],
    mut size: libc::c_int,
) -> libc::c_int {
    let mut maxArea: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < size - 3 as libc::c_int {
        if (*points.offset(i as isize))[0 as libc::c_int as usize]
            == (*points
                .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            && (*points
                .offset((i + 2 as libc::c_int) as isize))[0 as libc::c_int as usize]
                == (*points
                    .offset((i + 3 as libc::c_int) as isize))[0 as libc::c_int as usize]
            && (*points.offset(i as isize))[1 as libc::c_int as usize]
                == (*points
                    .offset((i + 2 as libc::c_int) as isize))[1 as libc::c_int as usize]
            && (*points
                .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                == (*points
                    .offset((i + 3 as libc::c_int) as isize))[1 as libc::c_int as usize]
        {
            let mut area: libc::c_int = ((*points
                .offset((i + 2 as libc::c_int) as isize))[0 as libc::c_int as usize]
                - (*points.offset(i as isize))[0 as libc::c_int as usize])
                * ((*points
                    .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                    - (*points.offset(i as isize))[1 as libc::c_int as usize]);
            if area > maxArea {
                maxArea = area;
            }
        }
        i += 1;
        i;
    }
    return maxArea;
}
#[no_mangle]
pub unsafe extern "C" fn comparePoints(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut pointA: *mut libc::c_int = a as *mut libc::c_int;
    let mut pointB: *mut libc::c_int = b as *mut libc::c_int;
    if *pointA.offset(0 as libc::c_int as isize)
        != *pointB.offset(0 as libc::c_int as isize)
    {
        return *pointA.offset(0 as libc::c_int as isize)
            - *pointB.offset(0 as libc::c_int as isize);
    }
    return *pointA.offset(1 as libc::c_int as isize)
        - *pointB.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn maxRectangleArea(
    mut points: *mut [libc::c_int; 2],
    mut n: libc::c_int,
) -> libc::c_int {
    let mut maxArea: libc::c_int = -(1 as libc::c_int);
    qsort(
        points as *mut libc::c_void,
        n as size_t,
        ::core::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong,
        Some(
            comparePoints
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n - 3 as libc::c_int {
        let mut rectanglePoints: [[libc::c_int; 2]; 4] = [[0; 2]; 4];
        rectanglePoints[0 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = (*points.offset(i as isize))[0 as libc::c_int as usize];
        rectanglePoints[0 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = (*points.offset(i as isize))[1 as libc::c_int as usize];
        rectanglePoints[1 as libc::c_int
            as usize][0 as libc::c_int
            as usize] = (*points
            .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize];
        rectanglePoints[1 as libc::c_int
            as usize][1 as libc::c_int
            as usize] = (*points
            .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize];
        let mut j: libc::c_int = i + 2 as libc::c_int;
        while j < n - 2 as libc::c_int {
            if !((*points.offset(j as isize))[1 as libc::c_int as usize]
                > (*points
                    .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                || (*points.offset(j as isize))[1 as libc::c_int as usize]
                    < (*points.offset(i as isize))[1 as libc::c_int as usize])
            {
                break;
            }
            j += 1;
            j;
        }
        if j < n - 1 as libc::c_int {
            rectanglePoints[2 as libc::c_int
                as usize][0 as libc::c_int
                as usize] = (*points.offset(j as isize))[0 as libc::c_int as usize];
            rectanglePoints[2 as libc::c_int
                as usize][1 as libc::c_int
                as usize] = (*points.offset(j as isize))[1 as libc::c_int as usize];
            rectanglePoints[3 as libc::c_int
                as usize][0 as libc::c_int
                as usize] = (*points
                .offset((j + 1 as libc::c_int) as isize))[0 as libc::c_int as usize];
            rectanglePoints[3 as libc::c_int
                as usize][1 as libc::c_int
                as usize] = (*points
                .offset((j + 1 as libc::c_int) as isize))[1 as libc::c_int as usize];
            let mut area: libc::c_int = get(
                rectanglePoints.as_mut_ptr(),
                4 as libc::c_int,
            );
            if area > maxArea {
                maxArea = area;
            }
        }
        i += 1;
        i;
    }
    return maxArea;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    if n < 4 as libc::c_int {
        printf(b"-1\n\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int;
    }
    let mut points: [[libc::c_int; 2]; 100] = [[0; 2]; 100];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*points.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_int,
            &mut *(*points.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = maxRectangleArea(points.as_mut_ptr(), n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
