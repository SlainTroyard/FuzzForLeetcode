use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut coin1: *mut libc::c_int = *(a as *mut *mut libc::c_int);
    let mut coin2: *mut libc::c_int = *(b as *mut *mut libc::c_int);
    return *coin1.offset(0 as libc::c_int as isize)
        - *coin2.offset(0 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn max_ll(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn maximumCoins(
    mut coins: *mut *mut libc::c_int,
    mut coinsSize: libc::c_int,
    mut coinsColSize: *mut libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    qsort(
        coins as *mut libc::c_void,
        coinsSize as size_t,
        ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut presum: *mut libc::c_longlong = malloc(
        ((coinsSize + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong),
    ) as *mut libc::c_longlong;
    *presum.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= coinsSize {
        *presum
            .offset(
                i as isize,
            ) = *presum.offset((i - 1 as libc::c_int) as isize)
            + (*(*coins.offset((i - 1 as libc::c_int) as isize))
                .offset(1 as libc::c_int as isize)
                - *(*coins.offset((i - 1 as libc::c_int) as isize))
                    .offset(0 as libc::c_int as isize) + 1 as libc::c_int)
                as libc::c_longlong
                * *(*coins.offset((i - 1 as libc::c_int) as isize))
                    .offset(2 as libc::c_int as isize) as libc::c_longlong;
        i += 1;
        i;
    }
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut left: libc::c_int = 0 as libc::c_int;
    let mut right: libc::c_int = 0 as libc::c_int;
    while right < coinsSize && left < coinsSize {
        while left < coinsSize
            && *(*coins.offset(right as isize)).offset(1 as libc::c_int as isize)
                - *(*coins.offset(left as isize)).offset(0 as libc::c_int as isize)
                + 1 as libc::c_int > k
        {
            let mut tamp: libc::c_longlong = (k
                - (*(*coins.offset(right as isize)).offset(0 as libc::c_int as isize)
                    - *(*coins.offset(left as isize)).offset(0 as libc::c_int as isize)))
                as libc::c_longlong;
            ans = max_ll(
                ans,
                tamp
                    * *(*coins.offset(right as isize)).offset(2 as libc::c_int as isize)
                        as libc::c_longlong + *presum.offset(right as isize)
                    - *presum.offset(left as isize),
            );
            left += 1 as libc::c_int;
        }
        if left >= coinsSize {
            break;
        }
        ans = max_ll(
            ans,
            *presum.offset((right + 1 as libc::c_int) as isize)
                - *presum.offset(left as isize),
        );
        right += 1 as libc::c_int;
    }
    left = coinsSize - 1 as libc::c_int;
    right = coinsSize - 1 as libc::c_int;
    while right >= 0 as libc::c_int && left >= 0 as libc::c_int {
        while right >= 0 as libc::c_int
            && *(*coins.offset(right as isize)).offset(1 as libc::c_int as isize)
                - *(*coins.offset(left as isize)).offset(0 as libc::c_int as isize)
                + 1 as libc::c_int > k
        {
            let mut tamp_0: libc::c_longlong = (k
                - (*(*coins.offset(right as isize)).offset(1 as libc::c_int as isize)
                    - *(*coins.offset(left as isize)).offset(1 as libc::c_int as isize)))
                as libc::c_longlong;
            ans = max_ll(
                ans,
                tamp_0
                    * *(*coins.offset(left as isize)).offset(2 as libc::c_int as isize)
                        as libc::c_longlong
                    + *presum.offset((right + 1 as libc::c_int) as isize)
                    - *presum.offset((left + 1 as libc::c_int) as isize),
            );
            right -= 1 as libc::c_int;
        }
        if right < 0 as libc::c_int {
            break;
        }
        ans = max_ll(
            ans,
            *presum.offset((right + 1 as libc::c_int) as isize)
                - *presum.offset(left as isize),
        );
        left -= 1 as libc::c_int;
    }
    free(presum as *mut libc::c_void);
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut coins: *mut *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut coinsColSize: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *coins.offset(i as isize);
        *fresh0 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *coinsColSize.offset(i as isize) = 3 as libc::c_int;
        scanf(
            b"%d %d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*coins.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*coins.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*coins.offset(i as isize)).offset(2 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        maximumCoins(coins, n, coinsColSize, k),
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        free(*coins.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(coins as *mut libc::c_void);
    free(coinsColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
