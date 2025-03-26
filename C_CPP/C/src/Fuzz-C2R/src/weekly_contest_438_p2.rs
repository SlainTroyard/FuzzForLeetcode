use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(b as *mut libc::c_int) - *(a as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn maxSum(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
    mut limits: *mut libc::c_int,
    mut limitsSize: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut size: libc::c_int = ::core::mem::size_of::<libc::c_int>() as libc::c_ulong
        as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < limitsSize {
        len += *limits.offset(i as isize);
        i += 1;
        i;
    }
    let mut lst: *mut libc::c_int = calloc(len as libc::c_ulong, size as libc::c_ulong)
        as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    while i_0 < gridSize {
        qsort(
            *grid.offset(i_0 as isize) as *mut libc::c_void,
            *gridColSize as size_t,
            size as size_t,
            Some(
                cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < *limits.offset(i_0 as isize) {
            let fresh0 = l;
            l = l + 1;
            *lst
                .offset(
                    fresh0 as isize,
                ) = *(*grid.offset(i_0 as isize)).offset(j as isize);
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    qsort(
        lst as *mut libc::c_void,
        len as size_t,
        size as size_t,
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut ans: libc::c_longlong = 0 as libc::c_longlong;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < k {
        ans += *lst.offset(i_1 as isize) as libc::c_longlong;
        i_1 += 1;
        i_1;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut grid: *mut *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *grid.offset(i as isize);
        *fresh1 = malloc(
            (m as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
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
    let mut limits: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *limits.offset(i_0 as isize) as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        maxSum(grid, n, &mut m, limits, n, k),
    );
    free(limits as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n {
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
