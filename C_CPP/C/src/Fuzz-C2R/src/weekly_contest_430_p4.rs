use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mult(
    mut n: libc::c_longlong,
    mut m: libc::c_longlong,
) -> libc::c_longlong {
    return n * m % 1000000007 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn power(
    mut n: libc::c_longlong,
    mut m: libc::c_longlong,
) -> libc::c_longlong {
    let mut res: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    let mut base: libc::c_longlong = n;
    while m > 0 as libc::c_int as libc::c_longlong {
        if m & 1 as libc::c_int as libc::c_longlong != 0 {
            res = mult(res, base);
        }
        base = mult(base, base);
        m >>= 1 as libc::c_int;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn inv(mut n: libc::c_longlong) -> libc::c_longlong {
    return power(n, (1000000007 as libc::c_int - 2 as libc::c_int) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn factorial(mut n: libc::c_longlong) -> libc::c_longlong {
    let mut res: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_longlong = 2 as libc::c_int as libc::c_longlong;
    while i <= n {
        res = mult(res, i);
        i += 1;
        i;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn comb(
    mut n: libc::c_longlong,
    mut m: libc::c_longlong,
) -> libc::c_longlong {
    return mult(factorial(n), inv(mult(factorial(m), factorial(n - m))));
}
#[no_mangle]
pub unsafe extern "C" fn countGoodArrays(
    mut n: libc::c_int,
    mut m: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    return mult(
        mult(
            comb(
                (n - 1 as libc::c_int) as libc::c_longlong,
                (n - 1 as libc::c_int - k) as libc::c_longlong,
            ),
            m as libc::c_longlong,
        ),
        power(
            (m - 1 as libc::c_int) as libc::c_longlong,
            (n - 1 as libc::c_int - k) as libc::c_longlong,
        ),
    );
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut result: libc::c_longlong = countGoodArrays(n, m, k);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
