use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn abs(_: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn resultsArray(
    mut queries: *mut *mut libc::c_int,
    mut queriesSize: libc::c_int,
    mut queriesColSize: *mut libc::c_int,
    mut k: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut libc::c_int;
    *returnSize = queriesSize;
    memset(
        result as *mut libc::c_void,
        -(1 as libc::c_int),
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    );
    let mut distanceArr: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul((k + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut distanceSize: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < queriesSize {
        let mut distance: libc::c_int = abs(
            *(*queries.offset(i as isize)).offset(0 as libc::c_int as isize),
        ) + abs(*(*queries.offset(i as isize)).offset(1 as libc::c_int as isize));
        let mut j: libc::c_int = distanceSize;
        while j > 0 as libc::c_int
            && *distanceArr.offset((j - 1 as libc::c_int) as isize) < distance
        {
            if j < k {
                *distanceArr
                    .offset(
                        j as isize,
                    ) = *distanceArr.offset((j - 1 as libc::c_int) as isize);
            }
            j -= 1;
            j;
        }
        if j < k {
            *distanceArr.offset(j as isize) = distance;
            if distanceSize < k {
                distanceSize += 1;
                distanceSize;
            }
        }
        if distanceSize == k {
            *result
                .offset(
                    i as isize,
                ) = *distanceArr.offset((k - 1 as libc::c_int) as isize);
        }
        i += 1;
        i;
    }
    free(distanceArr as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut queriesSize: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut queriesSize as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut queries: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < queriesSize {
        let ref mut fresh0 = *queries.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*queries.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*queries.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut queriesColSize: libc::c_int = 2 as libc::c_int;
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_int = resultsArray(
        queries,
        queriesSize,
        &mut queriesColSize,
        k,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *result.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    free(result as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < queriesSize {
        free(*queries.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(queries as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
