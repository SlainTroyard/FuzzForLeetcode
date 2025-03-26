use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pair {
    pub val: libc::c_int,
    pub freq: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut pa: *mut Pair = a as *mut Pair;
    let mut pb: *mut Pair = b as *mut Pair;
    if (*pa).freq != (*pb).freq {
        return (*pb).freq - (*pa).freq;
    }
    return (*pb).val - (*pa).val;
}
#[no_mangle]
pub unsafe extern "C" fn findXSum(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
    mut x: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_longlong {
    *returnSize = numsSize - k + 1 as libc::c_int;
    let mut result: *mut libc::c_longlong = malloc(
        (*returnSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong),
    ) as *mut libc::c_longlong;
    if result.is_null() {
        return 0 as *mut libc::c_longlong;
    }
    let mut hashTable: *mut *mut Pair = malloc(
        (1031 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Pair>() as libc::c_ulong),
    ) as *mut *mut Pair;
    if hashTable.is_null() {
        free(result as *mut libc::c_void);
        return 0 as *mut libc::c_longlong;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 1031 as libc::c_int {
        let ref mut fresh0 = *hashTable.offset(i as isize);
        *fresh0 = calloc(
            32 as libc::c_int as libc::c_ulong,
            ::core::mem::size_of::<Pair>() as libc::c_ulong,
        ) as *mut Pair;
        if (*hashTable.offset(i as isize)).is_null() {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                free(*hashTable.offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free(hashTable as *mut libc::c_void);
            free(result as *mut libc::c_void);
            return 0 as *mut libc::c_longlong;
        }
        i += 1;
        i;
    }
    let mut activeElements: *mut Pair = malloc(
        (k as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Pair>() as libc::c_ulong),
    ) as *mut Pair;
    let mut activeCount: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < k {
        let mut val: libc::c_int = *nums.offset(i_0 as isize);
        let mut hash: libc::c_uint = (val as libc::c_uint)
            .wrapping_rem(1031 as libc::c_int as libc::c_uint);
        let mut found: bool = 0 as libc::c_int != 0;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 32 as libc::c_int {
            if (*(*hashTable.offset(hash as isize)).offset(j_0 as isize)).freq
                == 0 as libc::c_int
            {
                break;
            }
            if (*(*hashTable.offset(hash as isize)).offset(j_0 as isize)).val == val {
                let ref mut fresh1 = (*(*hashTable.offset(hash as isize))
                    .offset(j_0 as isize))
                    .freq;
                *fresh1 += 1;
                *fresh1;
                found = 1 as libc::c_int != 0;
                break;
            } else {
                j_0 += 1;
                j_0;
            }
        }
        if !found {
            let mut j_1: libc::c_int = 0 as libc::c_int;
            while j_1 < 32 as libc::c_int {
                if (*(*hashTable.offset(hash as isize)).offset(j_1 as isize)).freq
                    == 0 as libc::c_int
                {
                    (*(*hashTable.offset(hash as isize)).offset(j_1 as isize)).val = val;
                    (*(*hashTable.offset(hash as isize)).offset(j_1 as isize))
                        .freq = 1 as libc::c_int;
                    break;
                } else {
                    j_1 += 1;
                    j_1;
                }
            }
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 1031 as libc::c_int {
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < 32 as libc::c_int {
            if (*(*hashTable.offset(i_1 as isize)).offset(j_2 as isize)).freq
                > 0 as libc::c_int
            {
                let fresh2 = activeCount;
                activeCount = activeCount + 1;
                *activeElements
                    .offset(
                        fresh2 as isize,
                    ) = *(*hashTable.offset(i_1 as isize)).offset(j_2 as isize);
                if activeCount >= k {
                    break;
                }
            }
            j_2 += 1;
            j_2;
        }
        if activeCount >= k {
            break;
        }
        i_1 += 1;
        i_1;
    }
    qsort(
        activeElements as *mut libc::c_void,
        activeCount as size_t,
        ::core::mem::size_of::<Pair>() as libc::c_ulong,
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut sum: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut countToSum: libc::c_int = if activeCount < x { activeCount } else { x };
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < countToSum {
        sum
            += (*activeElements.offset(i_2 as isize)).val as libc::c_longlong
                * (*activeElements.offset(i_2 as isize)).freq as libc::c_longlong;
        i_2 += 1;
        i_2;
    }
    *result.offset(0 as libc::c_int as isize) = sum;
    let mut i_3: libc::c_int = 1 as libc::c_int;
    while i_3 <= numsSize - k {
        let mut outVal: libc::c_int = *nums.offset((i_3 - 1 as libc::c_int) as isize);
        let mut inVal: libc::c_int = *nums.offset((i_3 + k - 1 as libc::c_int) as isize);
        let mut outHash: libc::c_uint = (outVal as libc::c_uint)
            .wrapping_rem(1031 as libc::c_int as libc::c_uint);
        let mut j_3: libc::c_int = 0 as libc::c_int;
        while j_3 < 32 as libc::c_int {
            if (*(*hashTable.offset(outHash as isize)).offset(j_3 as isize)).freq
                == 0 as libc::c_int
            {
                break;
            }
            if (*(*hashTable.offset(outHash as isize)).offset(j_3 as isize)).val
                == outVal
            {
                let ref mut fresh3 = (*(*hashTable.offset(outHash as isize))
                    .offset(j_3 as isize))
                    .freq;
                *fresh3 -= 1;
                *fresh3;
                break;
            } else {
                j_3 += 1;
                j_3;
            }
        }
        let mut inHash: libc::c_uint = (inVal as libc::c_uint)
            .wrapping_rem(1031 as libc::c_int as libc::c_uint);
        let mut found_0: bool = 0 as libc::c_int != 0;
        let mut j_4: libc::c_int = 0 as libc::c_int;
        while j_4 < 32 as libc::c_int {
            if (*(*hashTable.offset(inHash as isize)).offset(j_4 as isize)).freq
                == 0 as libc::c_int
            {
                break;
            }
            if (*(*hashTable.offset(inHash as isize)).offset(j_4 as isize)).val == inVal
            {
                let ref mut fresh4 = (*(*hashTable.offset(inHash as isize))
                    .offset(j_4 as isize))
                    .freq;
                *fresh4 += 1;
                *fresh4;
                found_0 = 1 as libc::c_int != 0;
                break;
            } else {
                j_4 += 1;
                j_4;
            }
        }
        if !found_0 {
            let mut j_5: libc::c_int = 0 as libc::c_int;
            while j_5 < 32 as libc::c_int {
                if (*(*hashTable.offset(inHash as isize)).offset(j_5 as isize)).freq
                    == 0 as libc::c_int
                {
                    (*(*hashTable.offset(inHash as isize)).offset(j_5 as isize))
                        .val = inVal;
                    (*(*hashTable.offset(inHash as isize)).offset(j_5 as isize))
                        .freq = 1 as libc::c_int;
                    break;
                } else {
                    j_5 += 1;
                    j_5;
                }
            }
        }
        activeCount = 0 as libc::c_int;
        let mut h: libc::c_int = 0 as libc::c_int;
        while h < 1031 as libc::c_int {
            let mut j_6: libc::c_int = 0 as libc::c_int;
            while j_6 < 32 as libc::c_int {
                if (*(*hashTable.offset(h as isize)).offset(j_6 as isize)).freq
                    > 0 as libc::c_int
                {
                    let fresh5 = activeCount;
                    activeCount = activeCount + 1;
                    *activeElements
                        .offset(
                            fresh5 as isize,
                        ) = *(*hashTable.offset(h as isize)).offset(j_6 as isize);
                }
                if activeCount >= k {
                    break;
                }
                j_6 += 1;
                j_6;
            }
            if activeCount >= k {
                break;
            }
            h += 1;
            h;
        }
        qsort(
            activeElements as *mut libc::c_void,
            activeCount as size_t,
            ::core::mem::size_of::<Pair>() as libc::c_ulong,
            Some(
                cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        sum = 0 as libc::c_int as libc::c_longlong;
        countToSum = if activeCount < x { activeCount } else { x };
        let mut j_7: libc::c_int = 0 as libc::c_int;
        while j_7 < countToSum {
            sum
                += (*activeElements.offset(j_7 as isize)).val as libc::c_longlong
                    * (*activeElements.offset(j_7 as isize)).freq as libc::c_longlong;
            j_7 += 1;
            j_7;
        }
        *result.offset(i_3 as isize) = sum;
        i_3 += 1;
        i_3;
    }
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < 1031 as libc::c_int {
        free(*hashTable.offset(i_4 as isize) as *mut libc::c_void);
        i_4 += 1;
        i_4;
    }
    free(hashTable as *mut libc::c_void);
    free(activeElements as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut k as *mut libc::c_int,
        &mut x as *mut libc::c_int,
    );
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
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_longlong = findXSum(
        nums,
        numsSize,
        k,
        x,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%lld \0" as *const u8 as *const libc::c_char,
            *result.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(nums as *mut libc::c_void);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
