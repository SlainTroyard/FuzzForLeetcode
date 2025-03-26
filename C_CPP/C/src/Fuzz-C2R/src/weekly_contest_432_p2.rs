use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maximumAmount(
    mut coins: *mut *mut libc::c_int,
    mut coinsSize: libc::c_int,
    mut coinsColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut row: libc::c_int = coinsSize - 1 as libc::c_int;
    let mut col: libc::c_int = *coinsColSize.offset(0 as libc::c_int as isize)
        - 1 as libc::c_int;
    let vla = (row + 1 as libc::c_int) as usize;
    let vla_0 = (col + 1 as libc::c_int) as usize;
    let mut dp: Vec::<[libc::c_int; 3]> = ::std::vec::from_elem([0; 3], vla * vla_0);
    (*dp
        .as_mut_ptr()
        .offset(row as isize * vla_0 as isize)
        .offset(
            col as isize,
        ))[0 as libc::c_int
        as usize] = *(*coins.offset(row as isize)).offset(col as isize);
    (*dp
        .as_mut_ptr()
        .offset(row as isize * vla_0 as isize)
        .offset(
            col as isize,
        ))[1 as libc::c_int
        as usize] = fmax(
        *(*coins.offset(row as isize)).offset(col as isize) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ) as libc::c_int;
    (*dp
        .as_mut_ptr()
        .offset(row as isize * vla_0 as isize)
        .offset(
            col as isize,
        ))[2 as libc::c_int
        as usize] = fmax(
        *(*coins.offset(row as isize)).offset(col as isize) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    ) as libc::c_int;
    let mut i: libc::c_int = col - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        (*dp
            .as_mut_ptr()
            .offset(row as isize * vla_0 as isize)
            .offset(
                i as isize,
            ))[0 as libc::c_int
            as usize] = (*dp
            .as_mut_ptr()
            .offset(row as isize * vla_0 as isize)
            .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
            + *(*coins.offset(row as isize)).offset(i as isize);
        (*dp
            .as_mut_ptr()
            .offset(row as isize * vla_0 as isize)
            .offset(
                i as isize,
            ))[1 as libc::c_int
            as usize] = fmax(
            (*dp
                .as_mut_ptr()
                .offset(row as isize * vla_0 as isize)
                .offset((i + 1 as libc::c_int) as isize))[0 as libc::c_int as usize]
                as libc::c_double,
            ((*dp
                .as_mut_ptr()
                .offset(row as isize * vla_0 as isize)
                .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                + *(*coins.offset(row as isize)).offset(i as isize)) as libc::c_double,
        ) as libc::c_int;
        (*dp
            .as_mut_ptr()
            .offset(row as isize * vla_0 as isize)
            .offset(
                i as isize,
            ))[2 as libc::c_int
            as usize] = fmax(
            (*dp
                .as_mut_ptr()
                .offset(row as isize * vla_0 as isize)
                .offset((i + 1 as libc::c_int) as isize))[1 as libc::c_int as usize]
                as libc::c_double,
            ((*dp
                .as_mut_ptr()
                .offset(row as isize * vla_0 as isize)
                .offset((i + 1 as libc::c_int) as isize))[2 as libc::c_int as usize]
                + *(*coins.offset(row as isize)).offset(i as isize)) as libc::c_double,
        ) as libc::c_int;
        i -= 1;
        i;
    }
    let mut i_0: libc::c_int = row - 1 as libc::c_int;
    while i_0 >= 0 as libc::c_int {
        (*dp
            .as_mut_ptr()
            .offset(i_0 as isize * vla_0 as isize)
            .offset(
                col as isize,
            ))[0 as libc::c_int
            as usize] = (*dp
            .as_mut_ptr()
            .offset((i_0 + 1 as libc::c_int) as isize * vla_0 as isize)
            .offset(col as isize))[0 as libc::c_int as usize]
            + *(*coins.offset(i_0 as isize)).offset(col as isize);
        (*dp
            .as_mut_ptr()
            .offset(i_0 as isize * vla_0 as isize)
            .offset(
                col as isize,
            ))[1 as libc::c_int
            as usize] = fmax(
            (*dp
                .as_mut_ptr()
                .offset((i_0 + 1 as libc::c_int) as isize * vla_0 as isize)
                .offset(col as isize))[0 as libc::c_int as usize] as libc::c_double,
            ((*dp
                .as_mut_ptr()
                .offset((i_0 + 1 as libc::c_int) as isize * vla_0 as isize)
                .offset(col as isize))[1 as libc::c_int as usize]
                + *(*coins.offset(i_0 as isize)).offset(col as isize)) as libc::c_double,
        ) as libc::c_int;
        (*dp
            .as_mut_ptr()
            .offset(i_0 as isize * vla_0 as isize)
            .offset(
                col as isize,
            ))[2 as libc::c_int
            as usize] = fmax(
            (*dp
                .as_mut_ptr()
                .offset((i_0 + 1 as libc::c_int) as isize * vla_0 as isize)
                .offset(col as isize))[1 as libc::c_int as usize] as libc::c_double,
            ((*dp
                .as_mut_ptr()
                .offset((i_0 + 1 as libc::c_int) as isize * vla_0 as isize)
                .offset(col as isize))[2 as libc::c_int as usize]
                + *(*coins.offset(i_0 as isize)).offset(col as isize)) as libc::c_double,
        ) as libc::c_int;
        i_0 -= 1;
        i_0;
    }
    let mut j: libc::c_int = row - 1 as libc::c_int;
    while j >= 0 as libc::c_int {
        let mut i_1: libc::c_int = col - 1 as libc::c_int;
        while i_1 >= 0 as libc::c_int {
            (*dp
                .as_mut_ptr()
                .offset(j as isize * vla_0 as isize)
                .offset(
                    i_1 as isize,
                ))[0 as libc::c_int
                as usize] = (fmax(
                (*dp
                    .as_mut_ptr()
                    .offset((j + 1 as libc::c_int) as isize * vla_0 as isize)
                    .offset(i_1 as isize))[0 as libc::c_int as usize] as libc::c_double,
                (*dp
                    .as_mut_ptr()
                    .offset(j as isize * vla_0 as isize)
                    .offset(
                        (i_1 + 1 as libc::c_int) as isize,
                    ))[0 as libc::c_int as usize] as libc::c_double,
            ) + *(*coins.offset(j as isize)).offset(i_1 as isize) as libc::c_double)
                as libc::c_int;
            (*dp
                .as_mut_ptr()
                .offset(j as isize * vla_0 as isize)
                .offset(
                    i_1 as isize,
                ))[1 as libc::c_int
                as usize] = fmax(
                fmax(
                    (*dp
                        .as_mut_ptr()
                        .offset(j as isize * vla_0 as isize)
                        .offset(
                            (i_1 + 1 as libc::c_int) as isize,
                        ))[0 as libc::c_int as usize] as libc::c_double,
                    ((*dp
                        .as_mut_ptr()
                        .offset(j as isize * vla_0 as isize)
                        .offset(
                            (i_1 + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize]
                        + *(*coins.offset(j as isize)).offset(i_1 as isize))
                        as libc::c_double,
                ),
                fmax(
                    (*dp
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize * vla_0 as isize)
                        .offset(i_1 as isize))[0 as libc::c_int as usize]
                        as libc::c_double,
                    ((*dp
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize * vla_0 as isize)
                        .offset(i_1 as isize))[1 as libc::c_int as usize]
                        + *(*coins.offset(j as isize)).offset(i_1 as isize))
                        as libc::c_double,
                ),
            ) as libc::c_int;
            (*dp
                .as_mut_ptr()
                .offset(j as isize * vla_0 as isize)
                .offset(
                    i_1 as isize,
                ))[2 as libc::c_int
                as usize] = fmax(
                fmax(
                    (*dp
                        .as_mut_ptr()
                        .offset(j as isize * vla_0 as isize)
                        .offset(
                            (i_1 + 1 as libc::c_int) as isize,
                        ))[1 as libc::c_int as usize] as libc::c_double,
                    ((*dp
                        .as_mut_ptr()
                        .offset(j as isize * vla_0 as isize)
                        .offset(
                            (i_1 + 1 as libc::c_int) as isize,
                        ))[2 as libc::c_int as usize]
                        + *(*coins.offset(j as isize)).offset(i_1 as isize))
                        as libc::c_double,
                ),
                fmax(
                    (*dp
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize * vla_0 as isize)
                        .offset(i_1 as isize))[1 as libc::c_int as usize]
                        as libc::c_double,
                    ((*dp
                        .as_mut_ptr()
                        .offset((j + 1 as libc::c_int) as isize * vla_0 as isize)
                        .offset(i_1 as isize))[2 as libc::c_int as usize]
                        + *(*coins.offset(j as isize)).offset(i_1 as isize))
                        as libc::c_double,
                ),
            ) as libc::c_int;
            i_1 -= 1;
            i_1;
        }
        j -= 1;
        j;
    }
    return (*dp
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize * vla_0 as isize)
        .offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
    );
    let mut coins: *mut *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *coins.offset(i as isize);
        *fresh0 = malloc(
            (m as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < m {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*coins.offset(i as isize)).offset(j as isize) as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let vla = n as usize;
    let mut coinsColSize: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        *coinsColSize.as_mut_ptr().offset(i_0 as isize) = m;
        i_0 += 1;
        i_0;
    }
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        maximumAmount(coins, n, coinsColSize.as_mut_ptr()),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
