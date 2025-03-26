use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe extern "C" fn dfs(
    mut x: libc::c_int,
    mut adj: *mut *mut libc::c_int,
    mut adjSize: *mut libc::c_int,
    mut visited: *mut libc::c_int,
) {
    *visited.offset(x as isize) = 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < *adjSize.offset(x as isize) {
        let mut nxt: libc::c_int = *(*adj.offset(x as isize)).offset(i as isize);
        if *visited.offset(nxt as isize) == 0 {
            dfs(nxt, adj, adjSize, visited);
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn remainingMethods(
    mut n: libc::c_int,
    mut k: libc::c_int,
    mut invocations: *mut *mut libc::c_int,
    mut invocationsSize: libc::c_int,
    mut invocationsColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut adj: *mut *mut libc::c_int = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
    ) as *mut *mut libc::c_int;
    let mut cap: *mut libc::c_int = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut adjSize: *mut libc::c_int = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut suspicious: *mut libc::c_int = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < invocationsSize {
        let mut a: libc::c_int = *(*invocations.offset(i as isize))
            .offset(0 as libc::c_int as isize);
        let mut b: libc::c_int = *(*invocations.offset(i as isize))
            .offset(1 as libc::c_int as isize);
        if *adjSize.offset(a as isize) == *cap.offset(a as isize) {
            *cap
                .offset(
                    a as isize,
                ) = *cap.offset(a as isize) * 2 as libc::c_int + 1 as libc::c_int;
            let ref mut fresh0 = *adj.offset(a as isize);
            *fresh0 = realloc(
                *adj.offset(a as isize) as *mut libc::c_void,
                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(*cap.offset(a as isize) as libc::c_ulong),
            ) as *mut libc::c_int;
        }
        let ref mut fresh1 = *adjSize.offset(a as isize);
        let fresh2 = *fresh1;
        *fresh1 = *fresh1 + 1;
        *(*adj.offset(a as isize)).offset(fresh2 as isize) = b;
        i += 1;
        i;
    }
    dfs(k, adj, adjSize, suspicious);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < invocationsSize {
        if *suspicious
            .offset(
                *(*invocations.offset(i_0 as isize)).offset(0 as libc::c_int as isize)
                    as isize,
            ) == 0
            && *suspicious
                .offset(
                    *(*invocations.offset(i_0 as isize))
                        .offset(1 as libc::c_int as isize) as isize,
                ) != 0
        {
            let mut ans: *mut libc::c_int = malloc(
                (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(n as libc::c_ulong),
            ) as *mut libc::c_int;
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < n {
                *ans.offset(j as isize) = j;
                j += 1;
                j;
            }
            *returnSize = n;
            return ans;
        }
        i_0 += 1;
        i_0;
    }
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n {
        if *suspicious.offset(i_1 as isize) == 0 {
            count += 1;
            count;
        }
        i_1 += 1;
        i_1;
    }
    let mut ans_0: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(count as libc::c_ulong),
    ) as *mut libc::c_int;
    *returnSize = count;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n {
        if *suspicious.offset(i_2 as isize) == 0 {
            let fresh3 = idx;
            idx = idx + 1;
            *ans_0.offset(fresh3 as isize) = i_2;
        }
        i_2 += 1;
        i_2;
    }
    free(adj as *mut libc::c_void);
    free(cap as *mut libc::c_void);
    free(adjSize as *mut libc::c_void);
    free(suspicious as *mut libc::c_void);
    return ans_0;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut invocationsSize: libc::c_int = 0;
    scanf(
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
        &mut invocationsSize as *mut libc::c_int,
    );
    let mut invocations: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(invocationsSize as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut invocationsColSize: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < invocationsSize {
        let ref mut fresh4 = *invocations.offset(i as isize);
        *fresh4 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*invocations.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*invocations.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut ans: *mut libc::c_int = remainingMethods(
        n,
        k,
        invocations,
        invocationsSize,
        invocationsColSize,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *ans.offset(i_0 as isize));
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(invocationsColSize as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < invocationsSize {
        free(*invocations.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(invocations as *mut libc::c_void);
    free(ans as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
