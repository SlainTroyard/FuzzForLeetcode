use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maxScore(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut maxnum: libc::c_int = 0 as libc::c_int;
    let mut m: libc::c_int = gridSize;
    let mut n: libc::c_int = *gridColSize.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
            maxnum = fmax(
                maxnum as libc::c_double,
                *(*grid.offset(i as isize)).offset(j as isize) as libc::c_double,
            ) as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let vla = (maxnum + 1 as libc::c_int) as usize;
    let mut numsRaw: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    memset(
        numsRaw.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < m {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < n {
            *numsRaw
                .as_mut_ptr()
                .offset(*(*grid.offset(i_0 as isize)).offset(j_0 as isize) as isize)
                |= (1 as libc::c_int) << i_0;
            j_0 += 1;
            j_0;
        }
        i_0 += 1;
        i_0;
    }
    let vla_0 = (maxnum + 1 as libc::c_int) as usize;
    let vla_1 = ((1 as libc::c_int) << m + 1 as libc::c_int) as usize;
    let mut dp: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0 * vla_1);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < maxnum + 1 as libc::c_int {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < (1 as libc::c_int) << m + 1 as libc::c_int {
            *dp
                .as_mut_ptr()
                .offset(i_1 as isize * vla_1 as isize)
                .offset(j_1 as isize) = -(2147483647 as libc::c_int) - 1 as libc::c_int;
            j_1 += 1;
            j_1;
        }
        i_1 += 1;
        i_1;
    }
    *dp
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize * vla_1 as isize)
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut i_2: libc::c_int = 1 as libc::c_int;
    while i_2 <= maxnum {
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < (1 as libc::c_int) << m {
            *dp
                .as_mut_ptr()
                .offset(i_2 as isize * vla_1 as isize)
                .offset(
                    j_2 as isize,
                ) = fmax(
                *dp
                    .as_mut_ptr()
                    .offset(i_2 as isize * vla_1 as isize)
                    .offset(j_2 as isize) as libc::c_double,
                *dp
                    .as_mut_ptr()
                    .offset((i_2 - 1 as libc::c_int) as isize * vla_1 as isize)
                    .offset(j_2 as isize) as libc::c_double,
            ) as libc::c_int;
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < m {
                if *numsRaw.as_mut_ptr().offset(i_2 as isize) >> k & 1 as libc::c_int
                    != 0 && j_2 >> k & 1 as libc::c_int != 0
                {
                    *dp
                        .as_mut_ptr()
                        .offset(i_2 as isize * vla_1 as isize)
                        .offset(
                            j_2 as isize,
                        ) = fmax(
                        *dp
                            .as_mut_ptr()
                            .offset(i_2 as isize * vla_1 as isize)
                            .offset(j_2 as isize) as libc::c_double,
                        (*dp
                            .as_mut_ptr()
                            .offset((i_2 - 1 as libc::c_int) as isize * vla_1 as isize)
                            .offset((j_2 ^ (1 as libc::c_int) << k) as isize) + i_2)
                            as libc::c_double,
                    ) as libc::c_int;
                    ans = fmax(
                        ans as libc::c_double,
                        *dp
                            .as_mut_ptr()
                            .offset(i_2 as isize * vla_1 as isize)
                            .offset(j_2 as isize) as libc::c_double,
                    ) as libc::c_int;
                }
                k += 1;
                k;
            }
            j_2 += 1;
            j_2;
        }
        i_2 += 1;
        i_2;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut gridSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut gridSize as *mut libc::c_int,
    );
    let vla = gridSize as usize;
    let mut gridColSize: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < gridSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *gridColSize.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut grid: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(gridSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < gridSize {
        let ref mut fresh0 = *grid.offset(i_0 as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(
                    *gridColSize.as_mut_ptr().offset(i_0 as isize) as libc::c_ulong,
                ),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < *gridColSize.as_mut_ptr().offset(i_0 as isize) {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*grid.offset(i_0 as isize)).offset(j as isize) as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    let mut ans: libc::c_int = maxScore(grid, gridSize, gridColSize.as_mut_ptr());
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, ans);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < gridSize {
        free(*grid.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(grid as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
