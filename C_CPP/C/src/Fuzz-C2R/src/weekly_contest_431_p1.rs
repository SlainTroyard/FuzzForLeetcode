use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gcd(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_longlong {
    while b != 0 as libc::c_int {
        let mut temp: libc::c_int = b;
        b = a % b;
        a = temp;
    }
    return a as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn lcm(
    mut a: libc::c_int,
    mut b: libc::c_int,
) -> libc::c_longlong {
    return a as libc::c_longlong / gcd(a, b) * b as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn maxLength(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut maxLength_0: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        let mut prod: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
        let mut g: libc::c_longlong = *nums.offset(i as isize) as libc::c_longlong;
        let mut l: libc::c_longlong = *nums.offset(i as isize) as libc::c_longlong;
        let mut j: libc::c_int = i;
        while j < numsSize {
            if prod
                > 9223372036854775807 as libc::c_longlong
                    / *nums.offset(j as isize) as libc::c_longlong
            {
                break;
            }
            prod *= *nums.offset(j as isize) as libc::c_longlong;
            g = gcd(g as libc::c_int, *nums.offset(j as isize));
            l = lcm(l as libc::c_int, *nums.offset(j as isize));
            if prod == l * g {
                let mut length: libc::c_int = j - i + 1 as libc::c_int;
                if length > maxLength_0 {
                    maxLength_0 = length;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return maxLength_0;
}
unsafe fn main_0() -> libc::c_int {
    let mut numSize: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut numSize as *mut libc::c_int);
    let mut nums: *mut libc::c_int = malloc(
        (numSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = maxLength(nums, numSize);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
