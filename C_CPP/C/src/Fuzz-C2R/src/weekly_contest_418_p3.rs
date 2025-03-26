use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut edgesSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut edgesSize as *mut libc::c_int,
    );
    let mut edges: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(edgesSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut edgesColSize: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edgesSize {
        let ref mut fresh1 = *edges.offset(i as isize);
        *fresh1 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*edges.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut returnColumnSizes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut res: *mut *mut libc::c_int = constructGridLayout(
        n,
        edges,
        edgesSize,
        edgesColSize,
        &mut returnSize,
        &mut returnColumnSizes,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < *returnColumnSizes.offset(i_0 as isize) {
            printf(
                b"%d \0" as *const u8 as *const libc::c_char,
                *(*res.offset(i_0 as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
        i_0;
    }
    free(edgesColSize as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < edgesSize {
        free(*edges.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(edges as *mut libc::c_void);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < returnSize {
        free(*res.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(res as *mut libc::c_void);
    free(returnColumnSizes as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
