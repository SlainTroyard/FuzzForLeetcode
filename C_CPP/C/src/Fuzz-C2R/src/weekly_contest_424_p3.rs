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
}
#[no_mangle]
pub unsafe extern "C" fn minZeroArray(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut queries: *mut *mut libc::c_int,
    mut queriesSize: libc::c_int,
    mut queriesColSize: *mut libc::c_int,
) -> libc::c_int {
    if nums.is_null() || numsSize == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if queries.is_null() || queriesSize == 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut max: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((numsSize + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        max as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
    );
    let mut sum: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        while sum + *max.offset(i as isize) < *nums.offset(i as isize) {
            if k == queriesSize {
                return -(1 as libc::c_int);
            }
            let mut start: libc::c_int = *(*queries.offset(k as isize))
                .offset(0 as libc::c_int as isize);
            let mut end: libc::c_int = *(*queries.offset(k as isize))
                .offset(1 as libc::c_int as isize);
            let mut increment: libc::c_int = *(*queries.offset(k as isize))
                .offset(2 as libc::c_int as isize);
            k += 1;
            k;
            if i > end {
                continue;
            }
            if i > start {
                *max.offset(i as isize) += increment;
            } else {
                *max.offset(start as isize) += increment;
            }
            *max.offset((end + 1 as libc::c_int) as isize) -= increment;
        }
        sum = sum + *max.offset(i as isize);
        i += 1;
        i;
    }
    return k;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut queriesSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut queriesSize as *mut libc::c_int,
    );
    let mut queries: *mut *mut libc::c_int = malloc(
        (queriesSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut queriesColSize: *mut libc::c_int = malloc(
        (queriesSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < queriesSize {
        let ref mut fresh0 = *queries.offset(i_0 as isize);
        *fresh0 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*queries.offset(i_0 as isize)).offset(j as isize)
                    as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        *queriesColSize.offset(i_0 as isize) = 3 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    let mut result: libc::c_int = minZeroArray(
        nums,
        numsSize,
        queries,
        queriesSize,
        queriesColSize,
    );
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < queriesSize {
        free(*queries.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(queries as *mut libc::c_void);
    free(queriesColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
