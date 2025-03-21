use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isZeroArray(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut queries: *mut *mut libc::c_int,
    mut queriesSize: libc::c_int,
    mut queriesColSize: *mut libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut diff: *mut libc::c_int = calloc(
        numsSize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < queriesSize {
        let ref mut fresh0 = *diff
            .offset(
                *(*queries.offset(i as isize)).offset(0 as libc::c_int as isize) as isize,
            );
        *fresh0 += 1;
        *fresh0;
        r = *(*queries.offset(i as isize)).offset(1 as libc::c_int as isize);
        if (r + 1 as libc::c_int) < numsSize {
            let ref mut fresh1 = *diff.offset((r + 1 as libc::c_int) as isize);
            *fresh1 -= 1;
            *fresh1;
        }
        i += 1;
        i;
    }
    i = 0 as libc::c_int;
    while i < numsSize {
        count += *diff.offset(i as isize);
        if *nums.offset(i as isize) > count {
            free(diff as *mut libc::c_void);
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    free(diff as *mut libc::c_void);
    return 1 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    let mut queriesSize: libc::c_int = 0;
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
        let ref mut fresh2 = *queries.offset(i_0 as isize);
        *fresh2 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*queries.offset(i_0 as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*queries.offset(i_0 as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        *queriesColSize.offset(i_0 as isize) = 2 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    if isZeroArray(nums, numsSize, queries, queriesSize, queriesColSize) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
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
