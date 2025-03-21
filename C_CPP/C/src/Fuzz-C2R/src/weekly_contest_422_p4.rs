use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub static mut n: libc::c_int = 0;
#[no_mangle]
pub static mut cnt: [libc::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut left_s: [libc::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut left_c: [libc::c_int; 10] = [0; 10];
#[no_mangle]
pub static mut dp: [[[libc::c_long; 81]; 721]; 10] = [[[0; 81]; 721]; 10];
#[no_mangle]
pub static mut r1: [libc::c_long; 11] = [0; 11];
#[no_mangle]
pub static mut cb: [[libc::c_int; 81]; 81] = [[0; 81]; 81];
#[no_mangle]
pub unsafe extern "C" fn pascal() {
    memset(
        cb.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[libc::c_int; 81]; 81]>() as libc::c_ulong,
    );
    cb[0 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i <= 80 as libc::c_int {
        cb[i as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
        cb[i as usize][i as usize] = 1 as libc::c_int;
        let mut j: libc::c_int = 1 as libc::c_int;
        while j < i {
            cb[i
                as usize][j
                as usize] = (cb[(i - 1 as libc::c_int)
                as usize][(j - 1 as libc::c_int) as usize]
                + cb[(i - 1 as libc::c_int) as usize][j as usize])
                % 1000000007 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn dfs(
    mut i: libc::c_int,
    mut s: libc::c_int,
    mut c: libc::c_int,
) -> libc::c_long {
    if s == 0 as libc::c_int && c == 0 as libc::c_int {
        return r1[i as usize];
    }
    if i == 10 as libc::c_int {
        return 0 as libc::c_int as libc::c_long;
    }
    if s > left_s[i as usize] || c > left_c[i as usize] {
        return 0 as libc::c_int as libc::c_long;
    }
    if dp[i as usize][s as usize][c as usize] != -(1 as libc::c_int) as libc::c_long {
        return dp[i as usize][s as usize][c as usize];
    }
    let mut res: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut a: libc::c_int = s;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = cnt[i as usize];
    while x <= cnt[i as usize] && a >= 0 as libc::c_int && c >= x {
        if !(y > left_c[i as usize] - c) {
            let mut b: libc::c_long = dfs(i + 1 as libc::c_int, a, c - x)
                * cb[c as usize][x as usize] as libc::c_long
                % 1000000007 as libc::c_int as libc::c_long;
            res = (res
                + b * cb[(left_c[i as usize] - c) as usize][y as usize] as libc::c_long)
                % 1000000007 as libc::c_int as libc::c_long;
        }
        x += 1;
        x;
        y -= 1;
        y;
        a -= i;
    }
    dp[i as usize][s as usize][c as usize] = res;
    return dp[i as usize][s as usize][c as usize];
}
#[no_mangle]
pub unsafe extern "C" fn countBalancedPermutations(
    mut num: *mut libc::c_char,
) -> libc::c_int {
    let mut s: libc::c_int = 0 as libc::c_int;
    memset(
        cnt.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 10]>() as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while *num.offset(i as isize) as libc::c_int != '\0' as i32 {
        let mut digit: libc::c_int = *num.offset(i as isize) as libc::c_int - '0' as i32;
        s += digit;
        cnt[digit as usize] += 1;
        cnt[digit as usize];
        i += 1;
        i;
    }
    if s % 2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    pascal();
    r1[10 as libc::c_int as usize] = 1 as libc::c_int as libc::c_long;
    let mut ls: libc::c_int = 0 as libc::c_int;
    let mut lc: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 9 as libc::c_int;
    while i_0 >= 0 as libc::c_int {
        ls += i_0 * cnt[i_0 as usize];
        lc += cnt[i_0 as usize];
        left_s[i_0 as usize] = ls;
        left_c[i_0 as usize] = lc;
        r1[i_0
            as usize] = r1[(i_0 + 1 as libc::c_int) as usize]
            * cb[left_c[i_0 as usize] as usize][cnt[i_0 as usize] as usize]
                as libc::c_long % 1000000007 as libc::c_int as libc::c_long;
        i_0 -= 1;
        i_0;
    }
    n = strlen(num) as libc::c_int;
    memset(
        dp.as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        ::core::mem::size_of::<[[[libc::c_long; 81]; 721]; 10]>() as libc::c_ulong,
    );
    return dfs(0 as libc::c_int, s / 2 as libc::c_int, n / 2 as libc::c_int)
        as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut num: [libc::c_char; 82] = [0; 82];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, num.as_mut_ptr());
    if strlen(num.as_mut_ptr()) > (81 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
    {
        printf(
            b"Input too long, maximum allowed length is %d\n\0" as *const u8
                as *const libc::c_char,
            81 as libc::c_int - 1 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    let mut result: libc::c_int = countBalancedPermutations(num.as_mut_ptr());
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
