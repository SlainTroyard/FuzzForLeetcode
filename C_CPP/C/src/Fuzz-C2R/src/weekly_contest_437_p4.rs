use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub static mut dp: [[libc::c_int; 500]; 500] = [[0; 500]; 500];
#[no_mangle]
pub unsafe extern "C" fn suc(mut a: libc::c_int, mut b: libc::c_int) -> bool {
    return a == 0 as libc::c_int && b == 2 as libc::c_int
        || a == 2 as libc::c_int && b == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn update_dp(
    mut G: *mut *mut libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut i: libc::c_int,
    mut j: libc::c_int,
) {
    dp[i as usize][j as usize] = 1 as libc::c_int;
    i -= 1;
    i;
    j += 1;
    j;
    while i >= 0 as libc::c_int && j < n {
        if suc(
            *(*G.offset(i as isize)).offset(j as isize),
            *(*G.offset((i + 1 as libc::c_int) as isize))
                .offset((j - 1 as libc::c_int) as isize),
        ) {
            dp[i
                as usize][j
                as usize] = dp[(i + 1 as libc::c_int)
                as usize][(j - 1 as libc::c_int) as usize] + 1 as libc::c_int;
        } else {
            dp[i as usize][j as usize] = 1 as libc::c_int;
        }
        i -= 1;
        i;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn solve(
    mut G: *mut *mut libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < m {
        update_dp(G, m, n, i, 0 as libc::c_int);
        i += 1;
        i;
    }
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < n {
        update_dp(G, m, n, m - 1 as libc::c_int, j);
        j += 1;
        j;
    }
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < m {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < n {
            if *(*G.offset(i_0 as isize)).offset(j_0 as isize) == 1 as libc::c_int {
                ans = fmax(ans as libc::c_double, 1 as libc::c_int as libc::c_double)
                    as libc::c_int;
                let mut ii: libc::c_int = i_0 + 1 as libc::c_int;
                let mut jj: libc::c_int = j_0 + 1 as libc::c_int;
                let mut len: libc::c_int = 1 as libc::c_int;
                while ii < m && jj < n {
                    if len == 1 as libc::c_int
                        && *(*G.offset(ii as isize)).offset(jj as isize)
                            != 2 as libc::c_int
                    {
                        break;
                    }
                    if len > 1 as libc::c_int
                        && !suc(
                            *(*G.offset(ii as isize)).offset(jj as isize),
                            *(*G.offset((ii - 1 as libc::c_int) as isize))
                                .offset((jj - 1 as libc::c_int) as isize),
                        )
                    {
                        break;
                    }
                    ans = fmax(
                        (len + dp[ii as usize][jj as usize]) as libc::c_double,
                        ans as libc::c_double,
                    ) as libc::c_int;
                    len += 1;
                    len;
                    ii += 1;
                    ii;
                    jj += 1;
                    jj;
                }
            }
            j_0 += 1;
            j_0;
        }
        i_0 += 1;
        i_0;
    }
    return ans;
}
#[no_mangle]
pub unsafe extern "C" fn rotate(
    mut grid: *mut *mut libc::c_int,
    mut m: libc::c_int,
    mut n: libc::c_int,
) -> *mut *mut libc::c_int {
    let mut arr: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *arr.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < m {
            *(*arr.offset(i as isize))
                .offset(
                    j as isize,
                ) = *(*grid.offset(j as isize))
                .offset((n - 1 as libc::c_int - i) as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return arr;
}
#[no_mangle]
pub unsafe extern "C" fn lenOfVDiagonal(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut m: libc::c_int = gridSize;
    let mut n: libc::c_int = *gridColSize.offset(0 as libc::c_int as isize);
    let mut arr_1: *mut *mut libc::c_int = rotate(grid, m, n);
    let mut arr_2: *mut *mut libc::c_int = rotate(arr_1, n, m);
    let mut arr_3: *mut *mut libc::c_int = rotate(arr_2, m, n);
    let mut res_1: libc::c_int = solve(grid, m, n);
    let mut res_2: libc::c_int = solve(arr_1, n, m);
    let mut res_3: libc::c_int = solve(arr_2, m, n);
    let mut res_4: libc::c_int = solve(arr_3, n, m);
    return fmax(
        fmax(
            fmax(res_1 as libc::c_double, res_2 as libc::c_double),
            res_3 as libc::c_double,
        ),
        res_4 as libc::c_double,
    ) as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
    );
    let mut grid: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *grid.offset(i as isize);
        *fresh1 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(m as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < m {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*grid.offset(i as isize)).offset(j as isize) as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut gridColSize: [libc::c_int; 1] = [m];
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        lenOfVDiagonal(grid, n, gridColSize.as_mut_ptr()),
    );
    free(grid as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
