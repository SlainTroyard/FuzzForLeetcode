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
pub unsafe extern "C" fn cmp_asc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_int) - *(b as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn cmp_desc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(b as *mut libc::c_int) - *(a as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sortMatrix(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
    mut returnColumnSizes: *mut *mut libc::c_int,
) -> *mut *mut libc::c_int {
    *returnSize = gridSize;
    *returnColumnSizes = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(gridSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < gridSize {
        *(*returnColumnSizes).offset(i as isize) = gridSize;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < gridSize {
        let mut len: libc::c_int = gridSize - i_0;
        let mut a: *mut libc::c_int = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(len as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut index: libc::c_int = 0 as libc::c_int;
        let mut k: libc::c_int = i_0;
        let mut m: libc::c_int = 0 as libc::c_int;
        while k < gridSize && m < gridSize {
            let fresh0 = index;
            index = index + 1;
            *a.offset(fresh0 as isize) = *(*grid.offset(k as isize)).offset(m as isize);
            k += 1;
            k;
            m += 1;
            m;
        }
        qsort(
            a as *mut libc::c_void,
            index as size_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            Some(
                cmp_desc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        index = 0 as libc::c_int;
        let mut k_0: libc::c_int = i_0;
        let mut m_0: libc::c_int = 0 as libc::c_int;
        while k_0 < gridSize && m_0 < gridSize {
            let fresh1 = index;
            index = index + 1;
            *(*grid.offset(k_0 as isize))
                .offset(m_0 as isize) = *a.offset(fresh1 as isize);
            k_0 += 1;
            k_0;
            m_0 += 1;
            m_0;
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 1 as libc::c_int;
    while i_1 < gridSize {
        let mut len_0: libc::c_int = gridSize - i_1;
        let mut a_0: *mut libc::c_int = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(len_0 as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut index_0: libc::c_int = 0 as libc::c_int;
        let mut k_1: libc::c_int = 0 as libc::c_int;
        let mut m_1: libc::c_int = i_1;
        while k_1 < gridSize && m_1 < gridSize {
            let fresh2 = index_0;
            index_0 = index_0 + 1;
            *a_0
                .offset(
                    fresh2 as isize,
                ) = *(*grid.offset(k_1 as isize)).offset(m_1 as isize);
            k_1 += 1;
            k_1;
            m_1 += 1;
            m_1;
        }
        qsort(
            a_0 as *mut libc::c_void,
            index_0 as size_t,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
            Some(
                cmp_asc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        index_0 = 0 as libc::c_int;
        let mut k_2: libc::c_int = 0 as libc::c_int;
        let mut m_2: libc::c_int = i_1;
        while k_2 < gridSize && m_2 < gridSize {
            let fresh3 = index_0;
            index_0 = index_0 + 1;
            *(*grid.offset(k_2 as isize))
                .offset(m_2 as isize) = *a_0.offset(fresh3 as isize);
            k_2 += 1;
            k_2;
            m_2 += 1;
            m_2;
        }
        i_1 += 1;
        i_1;
    }
    return grid;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut grid: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh4 = *grid.offset(i as isize);
        *fresh4 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(n as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < n {
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
    let mut returnSize: libc::c_int = 0;
    let mut returnColumnSizes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut result: *mut *mut libc::c_int = sortMatrix(
        grid,
        n,
        &mut n,
        &mut returnSize,
        &mut returnColumnSizes,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < n {
            printf(
                b"%d \0" as *const u8 as *const libc::c_char,
                *(*result.offset(i_0 as isize)).offset(j_0 as isize),
            );
            j_0 += 1;
            j_0;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
        i_0;
    }
    free(grid as *mut libc::c_void);
    free(returnColumnSizes as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
