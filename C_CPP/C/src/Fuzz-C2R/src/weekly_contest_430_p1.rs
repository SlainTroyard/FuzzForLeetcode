use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn minimumOperations(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut calGrid: *mut *mut libc::c_int = malloc(
        (gridSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < gridSize {
        let ref mut fresh0 = *calGrid.offset(i as isize);
        *fresh0 = malloc(
            (*gridColSize.offset(i as isize) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < *gridColSize.offset(i as isize) {
            *(*calGrid.offset(i as isize))
                .offset(j as isize) = *(*grid.offset(i as isize)).offset(j as isize);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut ans: libc::c_int = 0 as libc::c_int;
    if gridSize == 1 as libc::c_int {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < gridSize {
            free(*calGrid.offset(i_0 as isize) as *mut libc::c_void);
            i_0 += 1;
            i_0;
        }
        free(calGrid as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < *gridColSize.offset(0 as libc::c_int as isize) {
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while j_0 < gridSize {
            if *(*calGrid.offset(j_0 as isize)).offset(i_1 as isize)
                <= *(*calGrid.offset((j_0 - 1 as libc::c_int) as isize))
                    .offset(i_1 as isize)
            {
                ans
                    += *(*calGrid.offset((j_0 - 1 as libc::c_int) as isize))
                        .offset(i_1 as isize) + 1 as libc::c_int
                        - *(*calGrid.offset(j_0 as isize)).offset(i_1 as isize);
                *(*calGrid.offset(j_0 as isize))
                    .offset(
                        i_1 as isize,
                    ) = *(*calGrid.offset((j_0 - 1 as libc::c_int) as isize))
                    .offset(i_1 as isize) + 1 as libc::c_int;
            }
            j_0 += 1;
            j_0;
        }
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < gridSize {
        free(*calGrid.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(calGrid as *mut libc::c_void);
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut gridSize: libc::c_int = 0;
    let mut gridColSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut gridSize as *mut libc::c_int,
        &mut gridColSize as *mut libc::c_int,
    );
    let mut colSizes: *mut libc::c_int = malloc(
        (gridSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut grid: *mut *mut libc::c_int = malloc(
        (gridSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < gridSize {
        let ref mut fresh1 = *grid.offset(i as isize);
        *fresh1 = malloc(
            (gridColSize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *colSizes.offset(i as isize) = gridColSize;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < gridColSize {
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
    let mut result: libc::c_int = minimumOperations(grid, gridSize, colSizes);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < gridSize {
        free(*grid.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(grid as *mut libc::c_void);
    free(colSizes as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
