use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_int) - *(b as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gcdValues(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut queries: *mut libc::c_longlong,
    mut queriesSize: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut mx: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        if *nums.offset(i as isize) > mx {
            mx = *nums.offset(i as isize);
        }
        i += 1;
        i;
    }
    let mut cnt_x: *mut libc::c_int = calloc(
        (mx + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numsSize {
        let ref mut fresh0 = *cnt_x.offset(*nums.offset(i_0 as isize) as isize);
        *fresh0 += 1;
        *fresh0;
        i_0 += 1;
        i_0;
    }
    let mut cnt_gcd: *mut libc::c_longlong = calloc(
        (mx + 1 as libc::c_int) as libc::c_ulong,
        ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
    ) as *mut libc::c_longlong;
    let mut i_1: libc::c_int = mx;
    while i_1 > 0 as libc::c_int {
        let mut c: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = i_1;
        while j <= mx {
            c += *cnt_x.offset(j as isize);
            *cnt_gcd.offset(i_1 as isize) -= *cnt_gcd.offset(j as isize);
            j += i_1;
        }
        *cnt_gcd.offset(i_1 as isize)
            += c as libc::c_longlong * (c - 1 as libc::c_int) as libc::c_longlong
                / 2 as libc::c_int as libc::c_longlong;
        i_1 -= 1;
        i_1;
    }
    let mut i_2: libc::c_int = 1 as libc::c_int;
    while i_2 <= mx {
        *cnt_gcd.offset(i_2 as isize)
            += *cnt_gcd.offset((i_2 - 1 as libc::c_int) as isize);
        i_2 += 1;
        i_2;
    }
    let mut ans: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < queriesSize {
        let mut query: libc::c_longlong = *queries.offset(i_3 as isize);
        let mut left: libc::c_int = 1 as libc::c_int;
        let mut right: libc::c_int = mx;
        while left < right {
            let mut mid: libc::c_int = (left + right) / 2 as libc::c_int;
            if *cnt_gcd.offset(mid as isize) <= query {
                left = mid + 1 as libc::c_int;
            } else {
                right = mid;
            }
        }
        *ans.offset(i_3 as isize) = left;
        i_3 += 1;
        i_3;
    }
    *returnSize = queriesSize;
    free(cnt_x as *mut libc::c_void);
    free(cnt_gcd as *mut libc::c_void);
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numsSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut queriesSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut queriesSize as *mut libc::c_int,
    );
    let mut queries: *mut libc::c_longlong = malloc(
        (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
            .wrapping_mul(queriesSize as libc::c_ulong),
    ) as *mut libc::c_longlong;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < queriesSize {
        scanf(
            b"%lld\0" as *const u8 as *const libc::c_char,
            &mut *queries.offset(i_0 as isize) as *mut libc::c_longlong,
        );
        i_0 += 1;
        i_0;
    }
    let mut returnSize: libc::c_int = 0;
    let mut ans: *mut libc::c_int = gcdValues(
        nums,
        numsSize,
        queries,
        queriesSize,
        &mut returnSize,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < returnSize {
        printf(b"%d \0" as *const u8 as *const libc::c_char, *ans.offset(i_1 as isize));
        i_1 += 1;
        i_1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(nums as *mut libc::c_void);
    free(queries as *mut libc::c_void);
    free(ans as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
