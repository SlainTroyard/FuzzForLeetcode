use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn gcd(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    while b != 0 {
        let mut temp: libc::c_int = a % b;
        a = b;
        b = temp;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn subsequencePairCount(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let MOD: libc::c_int = 1000000007 as libc::c_int;
    let MX: libc::c_int = 201 as libc::c_int;
    static mut initialized: libc::c_int = 0 as libc::c_int;
    static mut lcms: [[libc::c_int; 201]; 201] = [[0; 201]; 201];
    static mut pow2: [libc::c_int; 201] = [0; 201];
    static mut pow3: [libc::c_int; 201] = [0; 201];
    static mut mu: [libc::c_int; 201] = [0; 201];
    if initialized == 0 {
        let mut i: libc::c_int = 1 as libc::c_int;
        while i < MX {
            let mut j: libc::c_int = 1 as libc::c_int;
            while j < MX {
                let mut g: libc::c_int = gcd(i, j);
                lcms[i as usize][j as usize] = i * j / g;
                j += 1;
                j;
            }
            i += 1;
            i;
        }
        pow2[0 as libc::c_int as usize] = 1 as libc::c_int;
        pow3[0 as libc::c_int as usize] = 1 as libc::c_int;
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 < MX {
            pow2[i_0
                as usize] = pow2[(i_0 - 1 as libc::c_int) as usize] * 2 as libc::c_int
                % MOD;
            pow3[i_0
                as usize] = (pow3[(i_0 - 1 as libc::c_int) as usize] as libc::c_longlong
                * 3 as libc::c_longlong % MOD as libc::c_longlong) as libc::c_int;
            i_0 += 1;
            i_0;
        }
        memset(
            mu.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<[libc::c_int; 201]>() as libc::c_ulong,
        );
        mu[1 as libc::c_int as usize] = 1 as libc::c_int;
        let mut i_1: libc::c_int = 1 as libc::c_int;
        while i_1 < MX {
            let mut j_0: libc::c_int = 2 as libc::c_int * i_1;
            while j_0 < MX {
                mu[j_0 as usize] -= mu[i_1 as usize];
                j_0 += i_1;
            }
            i_1 += 1;
            i_1;
        }
        initialized = 1 as libc::c_int;
    }
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < numsSize {
        if *nums.offset(i_2 as isize) > m {
            m = *nums.offset(i_2 as isize);
        }
        i_2 += 1;
        i_2;
    }
    let mut cnt: *mut libc::c_int = calloc(
        (m + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < numsSize {
        let ref mut fresh0 = *cnt.offset(*nums.offset(i_3 as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        i_3 += 1;
        i_3;
    }
    let mut i_4: libc::c_int = 1 as libc::c_int;
    while i_4 <= m {
        let mut j_1: libc::c_int = 2 as libc::c_int * i_4;
        while j_1 <= m {
            *cnt.offset(i_4 as isize) += *cnt.offset(j_1 as isize);
            j_1 += i_4;
        }
        i_4 += 1;
        i_4;
    }
    let mut f: *mut *mut libc::c_int = malloc(
        ((m + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 <= m {
        let ref mut fresh1 = *f.offset(i_5 as isize);
        *fresh1 = malloc(
            ((m + 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        i_5 += 1;
        i_5;
    }
    let mut g1: libc::c_int = 1 as libc::c_int;
    while g1 <= m {
        let mut g2: libc::c_int = 1 as libc::c_int;
        while g2 <= m {
            let mut l: libc::c_int = lcms[g1 as usize][g2 as usize];
            let mut c: libc::c_int = if l <= m {
                *cnt.offset(l as isize)
            } else {
                0 as libc::c_int
            };
            let mut c1: libc::c_int = *cnt.offset(g1 as isize);
            let mut c2: libc::c_int = *cnt.offset(g2 as isize);
            let mut term1: libc::c_longlong = pow3[c as usize] as libc::c_longlong
                * pow2[(c1 + c2 - 2 as libc::c_int * c) as usize] as libc::c_longlong
                % MOD as libc::c_longlong;
            let mut term2: libc::c_longlong = (term1
                - pow2[c1 as usize] as libc::c_longlong
                - pow2[c2 as usize] as libc::c_longlong
                + 1 as libc::c_int as libc::c_longlong) % MOD as libc::c_longlong;
            *(*f.offset(g1 as isize))
                .offset(
                    g2 as isize,
                ) = ((term2 + MOD as libc::c_longlong) % MOD as libc::c_longlong)
                as libc::c_int;
            g2 += 1;
            g2;
        }
        g1 += 1;
        g1;
    }
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut i_6: libc::c_int = 1 as libc::c_int;
    while i_6 <= m {
        let mut max_jk: libc::c_int = m / i_6;
        let mut j_2: libc::c_int = 1 as libc::c_int;
        while j_2 <= max_jk {
            let mut k: libc::c_int = 1 as libc::c_int;
            while k <= max_jk {
                let mut gj: libc::c_int = j_2 * i_6;
                let mut gk: libc::c_int = k * i_6;
                ans
                    += mu[j_2 as usize] as libc::c_longlong
                        * mu[k as usize] as libc::c_longlong
                        * *(*f.offset(gj as isize)).offset(gk as isize)
                            as libc::c_longlong;
                k += 1;
                k;
            }
            j_2 += 1;
            j_2;
        }
        i_6 += 1;
        i_6;
    }
    ans = (ans % MOD as libc::c_longlong + MOD as libc::c_longlong)
        % MOD as libc::c_longlong;
    free(cnt as *mut libc::c_void);
    let mut i_7: libc::c_int = 0 as libc::c_int;
    while i_7 <= m {
        free(*f.offset(i_7 as isize) as *mut libc::c_void);
        i_7 += 1;
        i_7;
    }
    free(f as *mut libc::c_void);
    return ans as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut nums: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = subsequencePairCount(nums, n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
