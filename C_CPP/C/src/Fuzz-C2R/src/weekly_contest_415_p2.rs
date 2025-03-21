use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maxScore(
    mut a: *mut libc::c_int,
    mut aSize: libc::c_int,
    mut b: *mut libc::c_int,
    mut bSize: libc::c_int,
) -> libc::c_longlong {
    let mut n: libc::c_int = bSize;
    let vla = (bSize + 1 as libc::c_int) as usize;
    let mut f: Vec::<[libc::c_longlong; 5]> = ::std::vec::from_elem([0; 5], vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= bSize {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j <= 4 as libc::c_int {
            (*f
                .as_mut_ptr()
                .offset(
                    i as isize,
                ))[j
                as usize] = (-(2147483647 as libc::c_int) - 1 as libc::c_int)
                as libc::c_longlong;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    (*f
        .as_mut_ptr()
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_longlong;
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 <= n {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 <= 4 as libc::c_int {
            (*f
                .as_mut_ptr()
                .offset(
                    i_0 as isize,
                ))[j_0
                as usize] = (*f
                .as_mut_ptr()
                .offset((i_0 - 1 as libc::c_int) as isize))[j_0 as usize];
            if j_0 > 0 as libc::c_int {
                (*f
                    .as_mut_ptr()
                    .offset(
                        i_0 as isize,
                    ))[j_0
                    as usize] = fmax(
                    (*f.as_mut_ptr().offset(i_0 as isize))[j_0 as usize]
                        as libc::c_double,
                    ((*f
                        .as_mut_ptr()
                        .offset(
                            (i_0 - 1 as libc::c_int) as isize,
                        ))[(j_0 - 1 as libc::c_int) as usize]
                        + 1 as libc::c_longlong
                            * *a.offset((j_0 - 1 as libc::c_int) as isize)
                                as libc::c_longlong
                            * *b.offset((i_0 - 1 as libc::c_int) as isize)
                                as libc::c_longlong) as libc::c_double,
                ) as libc::c_longlong;
            }
            j_0 += 1;
            j_0;
        }
        i_0 += 1;
        i_0;
    }
    return (*f.as_mut_ptr().offset(n as isize))[4 as libc::c_int as usize];
}
unsafe fn main_0() -> libc::c_int {
    let mut aSize: libc::c_int = 0;
    let mut bSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut aSize as *mut libc::c_int,
        &mut bSize as *mut libc::c_int,
    );
    let vla = aSize as usize;
    let mut a: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = bSize as usize;
    let mut b: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < aSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *a.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < bSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *b.as_mut_ptr().offset(i_0 as isize) as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        maxScore(a.as_mut_ptr(), aSize, b.as_mut_ptr(), bSize),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
