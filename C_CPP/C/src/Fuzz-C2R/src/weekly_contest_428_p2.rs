use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn maxAmount(
    mut initialCurrency: *mut libc::c_char,
    mut pairs1: *mut *mut *mut libc::c_char,
    mut pairs1Size: libc::c_int,
    mut pairs1ColSize: *mut libc::c_int,
    mut rates1: *mut libc::c_double,
    mut rates1Size: libc::c_int,
    mut pairs2: *mut *mut *mut libc::c_char,
    mut pairs2Size: libc::c_int,
    mut pairs2ColSize: *mut libc::c_int,
    mut rates2: *mut libc::c_double,
    mut rates2Size: libc::c_int,
) -> libc::c_double {
    let MAX_CURRENCIES: libc::c_int = 20 as libc::c_int;
    let vla = MAX_CURRENCIES as usize;
    let vla_0 = MAX_CURRENCIES as usize;
    let mut graph1: Vec::<libc::c_double> = ::std::vec::from_elem(0., vla * vla_0);
    let vla_1 = MAX_CURRENCIES as usize;
    let vla_2 = MAX_CURRENCIES as usize;
    let mut graph2: Vec::<libc::c_double> = ::std::vec::from_elem(0., vla_1 * vla_2);
    let vla_3 = MAX_CURRENCIES as usize;
    let mut currencies: Vec::<*mut libc::c_char> = ::std::vec::from_elem(
        0 as *mut libc::c_char,
        vla_3,
    );
    let mut currencyCount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < MAX_CURRENCIES {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < MAX_CURRENCIES {
            *graph1
                .as_mut_ptr()
                .offset(i as isize * vla_0 as isize)
                .offset(j as isize) = if i == j { 1.0f64 } else { 0.0f64 };
            *graph2
                .as_mut_ptr()
                .offset(i as isize * vla_2 as isize)
                .offset(j as isize) = if i == j { 1.0f64 } else { 0.0f64 };
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < pairs1Size {
        let mut from: libc::c_int = getCurrencyIndex(
            *(*pairs1.offset(i_0 as isize)).offset(0 as libc::c_int as isize),
        );
        let mut to: libc::c_int = getCurrencyIndex(
            *(*pairs1.offset(i_0 as isize)).offset(1 as libc::c_int as isize),
        );
        *graph1
            .as_mut_ptr()
            .offset(from as isize * vla_0 as isize)
            .offset(to as isize) = *rates1.offset(i_0 as isize);
        *graph1
            .as_mut_ptr()
            .offset(to as isize * vla_0 as isize)
            .offset(from as isize) = 1.0f64 / *rates1.offset(i_0 as isize);
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < pairs2Size {
        let mut from_0: libc::c_int = getCurrencyIndex(
            *(*pairs2.offset(i_1 as isize)).offset(0 as libc::c_int as isize),
        );
        let mut to_0: libc::c_int = getCurrencyIndex(
            *(*pairs2.offset(i_1 as isize)).offset(1 as libc::c_int as isize),
        );
        *graph2
            .as_mut_ptr()
            .offset(from_0 as isize * vla_2 as isize)
            .offset(to_0 as isize) = *rates2.offset(i_1 as isize);
        *graph2
            .as_mut_ptr()
            .offset(to_0 as isize * vla_2 as isize)
            .offset(from_0 as isize) = 1.0f64 / *rates2.offset(i_1 as isize);
        i_1 += 1;
        i_1;
    }
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < currencyCount {
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < currencyCount {
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < currencyCount {
                *graph1
                    .as_mut_ptr()
                    .offset(i_2 as isize * vla_0 as isize)
                    .offset(
                        j_0 as isize,
                    ) = fmax(
                    *graph1
                        .as_mut_ptr()
                        .offset(i_2 as isize * vla_0 as isize)
                        .offset(j_0 as isize),
                    *graph1
                        .as_mut_ptr()
                        .offset(i_2 as isize * vla_0 as isize)
                        .offset(k as isize)
                        * *graph1
                            .as_mut_ptr()
                            .offset(k as isize * vla_0 as isize)
                            .offset(j_0 as isize),
                );
                *graph2
                    .as_mut_ptr()
                    .offset(i_2 as isize * vla_2 as isize)
                    .offset(
                        j_0 as isize,
                    ) = fmax(
                    *graph2
                        .as_mut_ptr()
                        .offset(i_2 as isize * vla_2 as isize)
                        .offset(j_0 as isize),
                    *graph2
                        .as_mut_ptr()
                        .offset(i_2 as isize * vla_2 as isize)
                        .offset(k as isize)
                        * *graph2
                            .as_mut_ptr()
                            .offset(k as isize * vla_2 as isize)
                            .offset(j_0 as isize),
                );
                j_0 += 1;
                j_0;
            }
            i_2 += 1;
            i_2;
        }
        k += 1;
        k;
    }
    let mut startIndex: libc::c_int = getCurrencyIndex(initialCurrency);
    let mut maxAmount_0: libc::c_double = 1.0f64;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < currencyCount {
        let mut amountDay1: libc::c_double = *graph1
            .as_mut_ptr()
            .offset(startIndex as isize * vla_0 as isize)
            .offset(i_3 as isize);
        let mut amountDay2: libc::c_double = amountDay1
            * *graph2
                .as_mut_ptr()
                .offset(i_3 as isize * vla_2 as isize)
                .offset(startIndex as isize);
        maxAmount_0 = fmax(maxAmount_0, amountDay2);
        i_3 += 1;
        i_3;
    }
    return maxAmount_0;
}
unsafe fn main_0() -> libc::c_int {
    let mut initialCurrency: [libc::c_char; 4] = [0; 4];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, initialCurrency.as_mut_ptr());
    let mut pairs1Size: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut pairs1Size as *mut libc::c_int,
    );
    let mut pairs1: *mut *mut *mut libc::c_char = malloc(
        (pairs1Size as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_char;
    let mut rates1: *mut libc::c_double = malloc(
        (pairs1Size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut pairs1ColSize: *mut libc::c_int = malloc(
        (pairs1Size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < pairs1Size {
        let ref mut fresh0 = *pairs1.offset(i as isize);
        *fresh0 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh1 = *(*pairs1.offset(i as isize))
            .offset(0 as libc::c_int as isize);
        *fresh1 = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        let ref mut fresh2 = *(*pairs1.offset(i as isize))
            .offset(1 as libc::c_int as isize);
        *fresh2 = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        scanf(
            b"%s %s %lf\0" as *const u8 as *const libc::c_char,
            *(*pairs1.offset(i as isize)).offset(0 as libc::c_int as isize),
            *(*pairs1.offset(i as isize)).offset(1 as libc::c_int as isize),
            &mut *rates1.offset(i as isize) as *mut libc::c_double,
        );
        *pairs1ColSize.offset(i as isize) = 2 as libc::c_int;
        i += 1;
        i;
    }
    let mut pairs2Size: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut pairs2Size as *mut libc::c_int,
    );
    let mut pairs2: *mut *mut *mut libc::c_char = malloc(
        (pairs2Size as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_char;
    let mut rates2: *mut libc::c_double = malloc(
        (pairs2Size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    ) as *mut libc::c_double;
    let mut pairs2ColSize: *mut libc::c_int = malloc(
        (pairs2Size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < pairs2Size {
        let ref mut fresh3 = *pairs2.offset(i_0 as isize);
        *fresh3 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        let ref mut fresh4 = *(*pairs2.offset(i_0 as isize))
            .offset(0 as libc::c_int as isize);
        *fresh4 = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        let ref mut fresh5 = *(*pairs2.offset(i_0 as isize))
            .offset(1 as libc::c_int as isize);
        *fresh5 = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        scanf(
            b"%s %s %lf\0" as *const u8 as *const libc::c_char,
            *(*pairs2.offset(i_0 as isize)).offset(0 as libc::c_int as isize),
            *(*pairs2.offset(i_0 as isize)).offset(1 as libc::c_int as isize),
            &mut *rates2.offset(i_0 as isize) as *mut libc::c_double,
        );
        *pairs2ColSize.offset(i_0 as isize) = 2 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    let mut result: libc::c_double = maxAmount(
        initialCurrency.as_mut_ptr(),
        pairs1,
        pairs1Size,
        pairs1ColSize,
        rates1,
        pairs1Size,
        pairs2,
        pairs2Size,
        pairs2ColSize,
        rates2,
        pairs2Size,
    );
    printf(b"%.5lf\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < pairs1Size {
        free(
            *(*pairs1.offset(i_1 as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free(
            *(*pairs1.offset(i_1 as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free(*pairs1.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(pairs1 as *mut libc::c_void);
    free(rates1 as *mut libc::c_void);
    free(pairs1ColSize as *mut libc::c_void);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < pairs2Size {
        free(
            *(*pairs2.offset(i_2 as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free(
            *(*pairs2.offset(i_2 as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
        );
        free(*pairs2.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(pairs2 as *mut libc::c_void);
    free(rates2 as *mut libc::c_void);
    free(pairs2ColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
