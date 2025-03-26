use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zigzagTraversal(
    mut grid: *mut *mut libc::c_int,
    mut gridSize: libc::c_int,
    mut gridColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut ans: *mut libc::c_int = calloc(
        (*gridColSize * gridSize + 1 as libc::c_int >> 1 as libc::c_int)
            as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut r: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_int = 0;
    let mut c1: libc::c_int = *gridColSize - 1 as libc::c_int
        - (*gridColSize & 1 as libc::c_int);
    *returnSize = 0 as libc::c_int;
    while r < gridSize {
        if r & 1 as libc::c_int != 0 {
            c = c1;
            while c >= 0 as libc::c_int {
                let fresh0 = *returnSize;
                *returnSize = *returnSize + 1;
                *ans
                    .offset(
                        fresh0 as isize,
                    ) = *(*grid.offset(r as isize)).offset(c as isize);
                c -= 2 as libc::c_int;
            }
        } else {
            c = 0 as libc::c_int;
            while c < *gridColSize {
                let fresh1 = *returnSize;
                *returnSize = *returnSize + 1;
                *ans
                    .offset(
                        fresh1 as isize,
                    ) = *(*grid.offset(r as isize)).offset(c as isize);
                c += 2 as libc::c_int;
            }
        }
        r += 1;
        r;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut gridSize: libc::c_int = 0;
    let mut gridColSize: libc::c_int = 0;
    let mut returnSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut gridSize as *mut libc::c_int,
        &mut gridColSize as *mut libc::c_int,
    );
    let mut grid: *mut *mut libc::c_int = malloc(
        (gridSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < gridSize {
        let ref mut fresh2 = *grid.offset(i as isize);
        *fresh2 = malloc(
            (gridColSize as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
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
    let mut ans: *mut libc::c_int = zigzagTraversal(
        grid,
        gridSize,
        &mut gridColSize,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *ans.offset(i_0 as isize));
        i_0 += 1;
        i_0;
    }
    free(ans as *mut libc::c_void);
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
