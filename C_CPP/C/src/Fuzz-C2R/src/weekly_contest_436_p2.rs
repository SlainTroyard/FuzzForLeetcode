use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn assignElements(
    mut groups: *mut libc::c_int,
    mut groupsSize: libc::c_int,
    mut elements: *mut libc::c_int,
    mut elementsSize: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut index: [libc::c_int; 100005] = [0; 100005];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 100005 as libc::c_int {
        index[i as usize] = 2147483647 as libc::c_int;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < elementsSize {
        index[*elements.offset(i_0 as isize)
            as usize] = fmin(
            index[*elements.offset(i_0 as isize) as usize] as libc::c_double,
            i_0 as libc::c_double,
        ) as libc::c_int;
        i_0 += 1;
        i_0;
    }
    let mut ans: *mut libc::c_int = malloc(
        (groupsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *returnSize = groupsSize;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < groupsSize {
        *ans.offset(i_1 as isize) = 2147483647 as libc::c_int;
        let mut j: libc::c_int = 1 as libc::c_int;
        while j as libc::c_double <= sqrt(*groups.offset(i_1 as isize) as libc::c_double)
        {
            if *groups.offset(i_1 as isize) % j == 0 as libc::c_int
                && (index[j as usize] != 2147483647 as libc::c_int
                    || index[(*groups.offset(i_1 as isize) / j) as usize]
                        != 2147483647 as libc::c_int)
            {
                *ans
                    .offset(
                        i_1 as isize,
                    ) = fmin(
                    *ans.offset(i_1 as isize) as libc::c_double,
                    fmin(
                        index[(*groups.offset(i_1 as isize) / j) as usize]
                            as libc::c_double,
                        index[j as usize] as libc::c_double,
                    ),
                ) as libc::c_int;
            }
            j += 1;
            j;
        }
        if *ans.offset(i_1 as isize) == 2147483647 as libc::c_int {
            *ans.offset(i_1 as isize) = -(1 as libc::c_int);
        }
        i_1 += 1;
        i_1;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
    );
    let mut groups: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut elements: *mut libc::c_int = malloc(
        (m as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *groups.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < m {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *elements.offset(i_0 as isize) as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    let mut returnSize: libc::c_int = 0;
    let mut ans: *mut libc::c_int = assignElements(
        groups,
        n,
        elements,
        m,
        &mut returnSize,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < returnSize {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *ans.offset(i_1 as isize));
        i_1 += 1;
        i_1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(groups as *mut libc::c_void);
    free(elements as *mut libc::c_void);
    free(ans as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
