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
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maximumSubarrayXor(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut queries: *mut *mut libc::c_int,
    mut queriesSize: libc::c_int,
    mut queriesColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut subarrayXors: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        let ref mut fresh0 = *subarrayXors.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(numsSize as libc::c_ulong),
        ) as *mut libc::c_int;
        memset(
            *subarrayXors.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(numsSize as libc::c_ulong),
        );
        *(*subarrayXors.offset(i as isize))
            .offset(i as isize) = *nums.offset(i as isize);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numsSize {
        *(*subarrayXors.offset(i_0 as isize))
            .offset(i_0 as isize) = *nums.offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    let mut subLength: libc::c_int = 2 as libc::c_int;
    while subLength <= numsSize {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = subLength - 1 as libc::c_int;
        while j < numsSize {
            *(*subarrayXors.offset(i_1 as isize))
                .offset(
                    j as isize,
                ) = *(*subarrayXors.offset(i_1 as isize))
                .offset((j - 1 as libc::c_int) as isize)
                ^ *(*subarrayXors.offset((i_1 + 1 as libc::c_int) as isize))
                    .offset(j as isize);
            i_1 += 1;
            i_1;
            j += 1;
            j;
        }
        subLength += 1;
        subLength;
    }
    let mut maxScores: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < numsSize {
        let ref mut fresh1 = *maxScores.offset(i_2 as isize);
        *fresh1 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(numsSize as libc::c_ulong),
        ) as *mut libc::c_int;
        memset(
            *maxScores.offset(i_2 as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(numsSize as libc::c_ulong),
        );
        *(*maxScores.offset(i_2 as isize))
            .offset(i_2 as isize) = *nums.offset(i_2 as isize);
        i_2 += 1;
        i_2;
    }
    let mut subLength_0: libc::c_int = 2 as libc::c_int;
    while subLength_0 <= numsSize {
        let mut i_3: libc::c_int = 0 as libc::c_int;
        let mut j_0: libc::c_int = subLength_0 - 1 as libc::c_int;
        while j_0 < numsSize {
            *(*maxScores.offset(i_3 as isize))
                .offset(
                    j_0 as isize,
                ) = fmax(
                *(*subarrayXors.offset(i_3 as isize)).offset(j_0 as isize)
                    as libc::c_double,
                fmax(
                    *(*maxScores.offset(i_3 as isize))
                        .offset((j_0 - 1 as libc::c_int) as isize) as libc::c_double,
                    *(*maxScores.offset((i_3 + 1 as libc::c_int) as isize))
                        .offset(j_0 as isize) as libc::c_double,
                ),
            ) as libc::c_int;
            i_3 += 1;
            i_3;
            j_0 += 1;
            j_0;
        }
        subLength_0 += 1;
        subLength_0;
    }
    let mut answer: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < queriesSize {
        *answer
            .offset(
                i_4 as isize,
            ) = *(*maxScores
            .offset(
                *(*queries.offset(i_4 as isize)).offset(0 as libc::c_int as isize)
                    as isize,
            ))
            .offset(
                *(*queries.offset(i_4 as isize)).offset(1 as libc::c_int as isize)
                    as isize,
            );
        i_4 += 1;
        i_4;
    }
    *returnSize = queriesSize;
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 < numsSize {
        free(*subarrayXors.offset(i_5 as isize) as *mut libc::c_void);
        free(*maxScores.offset(i_5 as isize) as *mut libc::c_void);
        i_5 += 1;
        i_5;
    }
    free(subarrayXors as *mut libc::c_void);
    free(maxScores as *mut libc::c_void);
    return answer;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    let mut queriesSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
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
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut queriesColSize: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < queriesSize {
        let ref mut fresh2 = *queries.offset(i_0 as isize);
        *fresh2 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*queries.offset(i_0 as isize)).offset(j as isize)
                    as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    let mut returnSize: libc::c_int = 0;
    let mut answer: *mut libc::c_int = maximumSubarrayXor(
        nums,
        numsSize,
        queries,
        queriesSize,
        queriesColSize,
        &mut returnSize,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *answer.offset(i_1 as isize),
        );
        i_1 += 1;
        i_1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(nums as *mut libc::c_void);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < queriesSize {
        free(*queries.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(queries as *mut libc::c_void);
    free(queriesColSize as *mut libc::c_void);
    free(answer as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
