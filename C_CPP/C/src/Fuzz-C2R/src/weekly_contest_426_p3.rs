use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn linepots(
    mut k: libc::c_int,
    mut pots: *mut *mut libc::c_int,
    mut node: libc::c_int,
    mut length: *mut libc::c_int,
    mut visited: libc::c_int,
) -> libc::c_int {
    if k == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    if k == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut answer: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < *length.offset(node as isize) {
        if *(*pots.offset(node as isize)).offset(i as isize) != visited {
            answer
                += linepots(
                    k - 1 as libc::c_int,
                    pots,
                    *(*pots.offset(node as isize)).offset(i as isize),
                    length,
                    node,
                );
        }
        i += 1;
        i;
    }
    return answer;
}
#[no_mangle]
pub unsafe extern "C" fn maxTargetNodes(
    mut edges1: *mut *mut libc::c_int,
    mut edges1Size: libc::c_int,
    mut edges1ColSize: *mut libc::c_int,
    mut edges2: *mut *mut libc::c_int,
    mut edges2Size: libc::c_int,
    mut edges2ColSize: *mut libc::c_int,
    mut k: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut len1: libc::c_int = 0 as libc::c_int;
    let mut len2: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edges1Size {
        if *(*edges1.offset(i as isize)).offset(1 as libc::c_int as isize) > len1 {
            len1 = *(*edges1.offset(i as isize)).offset(1 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < edges2Size {
        if *(*edges2.offset(i_0 as isize)).offset(1 as libc::c_int as isize) > len2 {
            len2 = *(*edges2.offset(i_0 as isize)).offset(1 as libc::c_int as isize);
        }
        i_0 += 1;
        i_0;
    }
    let mut pots1: *mut *mut libc::c_int = malloc(
        ((len1 + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut pots2: *mut *mut libc::c_int = malloc(
        ((len2 + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut answer: *mut libc::c_int = calloc(
        (len1 + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut length1: *mut libc::c_int = calloc(
        (len1 + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut length2: *mut libc::c_int = calloc(
        (len2 + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 <= len1 {
        let mut add: libc::c_int = 0 as libc::c_int;
        let mut ccc: *mut libc::c_int = malloc(
            ((len1 + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < edges1Size {
            if *(*edges1.offset(j as isize)).offset(0 as libc::c_int as isize) == i_1 {
                let fresh0 = add;
                add = add + 1;
                *ccc
                    .offset(
                        fresh0 as isize,
                    ) = *(*edges1.offset(j as isize)).offset(1 as libc::c_int as isize);
            }
            if *(*edges1.offset(j as isize)).offset(1 as libc::c_int as isize) == i_1 {
                let fresh1 = add;
                add = add + 1;
                *ccc
                    .offset(
                        fresh1 as isize,
                    ) = *(*edges1.offset(j as isize)).offset(0 as libc::c_int as isize);
            }
            j += 1;
            j;
        }
        let ref mut fresh2 = *pots1.offset(i_1 as isize);
        *fresh2 = malloc(
            (add as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *length1.offset(i_1 as isize) = add;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < add {
            *(*pots1.offset(i_1 as isize))
                .offset(j_0 as isize) = *ccc.offset(j_0 as isize);
            j_0 += 1;
            j_0;
        }
        free(ccc as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 <= len2 {
        let mut add_0: libc::c_int = 0 as libc::c_int;
        let mut ccc_0: *mut libc::c_int = malloc(
            ((len2 + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < edges2Size {
            if *(*edges2.offset(j_1 as isize)).offset(0 as libc::c_int as isize) == i_2 {
                let fresh3 = add_0;
                add_0 = add_0 + 1;
                *ccc_0
                    .offset(
                        fresh3 as isize,
                    ) = *(*edges2.offset(j_1 as isize))
                    .offset(1 as libc::c_int as isize);
            }
            if *(*edges2.offset(j_1 as isize)).offset(1 as libc::c_int as isize) == i_2 {
                let fresh4 = add_0;
                add_0 = add_0 + 1;
                *ccc_0
                    .offset(
                        fresh4 as isize,
                    ) = *(*edges2.offset(j_1 as isize))
                    .offset(0 as libc::c_int as isize);
            }
            j_1 += 1;
            j_1;
        }
        let ref mut fresh5 = *pots2.offset(i_2 as isize);
        *fresh5 = malloc(
            (add_0 as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *length2.offset(i_2 as isize) = add_0;
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < add_0 {
            *(*pots2.offset(i_2 as isize))
                .offset(j_2 as isize) = *ccc_0.offset(j_2 as isize);
            j_2 += 1;
            j_2;
        }
        free(ccc_0 as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    let mut max: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 <= len2 {
        let mut temp: libc::c_int = linepots(
            k - 1 as libc::c_int,
            pots2,
            i_3,
            length2,
            -(1 as libc::c_int),
        );
        if temp > max {
            max = temp;
        }
        i_3 += 1;
        i_3;
    }
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 <= len1 {
        *answer
            .offset(
                i_4 as isize,
            ) = linepots(k, pots1, i_4, length1, -(1 as libc::c_int)) + max;
        i_4 += 1;
        i_4;
    }
    *returnSize = len1 + 1 as libc::c_int;
    return answer;
}
unsafe fn main_0() -> libc::c_int {
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n1 as *mut libc::c_int);
    let vla = n1 as usize;
    let mut edges1: Vec::<[libc::c_int; 2]> = ::std::vec::from_elem([0; 2], vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n1 {
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges1.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_int,
            &mut *(*edges1.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n2 as *mut libc::c_int);
    let vla_0 = n2 as usize;
    let mut edges2: Vec::<[libc::c_int; 2]> = ::std::vec::from_elem([0; 2], vla_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n2 {
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges2.as_mut_ptr().offset(i_0 as isize))
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_int,
            &mut *(*edges2.as_mut_ptr().offset(i_0 as isize))
                .as_mut_ptr()
                .offset(1 as libc::c_int as isize) as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    let mut edges1Ptr: *mut *mut libc::c_int = malloc(
        (n1 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut edges2Ptr: *mut *mut libc::c_int = malloc(
        (n2 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n1 {
        let ref mut fresh6 = *edges1Ptr.offset(i_1 as isize);
        *fresh6 = (*edges1.as_mut_ptr().offset(i_1 as isize)).as_mut_ptr();
        i_1 += 1;
        i_1;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n2 {
        let ref mut fresh7 = *edges2Ptr.offset(i_2 as isize);
        *fresh7 = (*edges2.as_mut_ptr().offset(i_2 as isize)).as_mut_ptr();
        i_2 += 1;
        i_2;
    }
    let mut returnSize: libc::c_int = 0 as libc::c_int;
    let mut result: *mut libc::c_int = maxTargetNodes(
        edges1Ptr,
        n1,
        0 as *mut libc::c_int,
        edges2Ptr,
        n2,
        0 as *mut libc::c_int,
        k,
        &mut returnSize,
    );
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *result.offset(i_3 as isize),
        );
        i_3 += 1;
        i_3;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(result as *mut libc::c_void);
    free(edges1Ptr as *mut libc::c_void);
    free(edges2Ptr as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
