use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmax(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn gcd(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    let mut c: libc::c_longlong = a % b;
    while c != 0 {
        a = b;
        b = c;
        c = a % b;
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn lcm(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    return a / gcd(a, b) * b;
}
#[no_mangle]
pub unsafe extern "C" fn maxScore(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_longlong {
    if numsSize == 1 as libc::c_int {
        return (*nums.offset(0 as libc::c_int as isize)
            * *nums.offset(0 as libc::c_int as isize)) as libc::c_longlong;
    }
    let vla = numsSize as usize;
    let mut lcms: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla);
    let vla_0 = numsSize as usize;
    let mut gcds: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla_0);
    let ref mut fresh0 = *gcds
        .as_mut_ptr()
        .offset((numsSize - 1 as libc::c_int) as isize);
    *fresh0 = *nums.offset((numsSize - 1 as libc::c_int) as isize) as libc::c_longlong;
    *lcms.as_mut_ptr().offset((numsSize - 1 as libc::c_int) as isize) = *fresh0;
    let mut i: libc::c_int = numsSize - 2 as libc::c_int;
    while i >= 0 as libc::c_int {
        *lcms
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = lcm(
            *nums.offset(i as isize) as libc::c_longlong,
            *lcms.as_mut_ptr().offset((i + 1 as libc::c_int) as isize),
        );
        *gcds
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = gcd(
            *nums.offset(i as isize) as libc::c_longlong,
            *gcds.as_mut_ptr().offset((i + 1 as libc::c_int) as isize),
        );
        i -= 1;
        i;
    }
    let mut ans: libc::c_longlong = *lcms.as_mut_ptr().offset(0 as libc::c_int as isize)
        * *gcds.as_mut_ptr().offset(0 as libc::c_int as isize);
    ans = fmax(
        ans as libc::c_double,
        (*lcms.as_mut_ptr().offset(1 as libc::c_int as isize)
            * *gcds.as_mut_ptr().offset(1 as libc::c_int as isize)) as libc::c_double,
    ) as libc::c_longlong;
    let mut prelcm: libc::c_longlong = *nums.offset(0 as libc::c_int as isize)
        as libc::c_longlong;
    let mut pregcd: libc::c_longlong = *nums.offset(0 as libc::c_int as isize)
        as libc::c_longlong;
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 < numsSize - 1 as libc::c_int {
        ans = fmax(
            (gcd(pregcd, *gcds.as_mut_ptr().offset((i_0 + 1 as libc::c_int) as isize))
                * lcm(
                    prelcm,
                    *lcms.as_mut_ptr().offset((i_0 + 1 as libc::c_int) as isize),
                )) as libc::c_double,
            ans as libc::c_double,
        ) as libc::c_longlong;
        prelcm = lcm(prelcm, *nums.offset(i_0 as isize) as libc::c_longlong);
        pregcd = gcd(pregcd, *nums.offset(i_0 as isize) as libc::c_longlong);
        i_0 += 1;
        i_0;
    }
    ans = fmax(ans as libc::c_double, (prelcm * pregcd) as libc::c_double)
        as libc::c_longlong;
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let vla = numsSize as usize;
    let mut nums: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        maxScore(nums.as_mut_ptr(), numsSize),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
