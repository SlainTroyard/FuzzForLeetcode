use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn subarraySum(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ans: libc::c_int = 0 as libc::c_int;
    let vla = (numsSize + 1 as libc::c_int) as usize;
    let mut sums: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    *sums.as_mut_ptr().offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < numsSize {
        *sums
            .as_mut_ptr()
            .offset(
                (i + 1 as libc::c_int) as isize,
            ) = *nums.offset(i as isize) + *sums.as_mut_ptr().offset(i as isize);
        ans
            += *sums.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
                - *sums
                    .as_mut_ptr()
                    .offset(
                        (if i - *nums.offset(i as isize) > 0 as libc::c_int {
                            i - *nums.offset(i as isize)
                        } else {
                            0 as libc::c_int
                        }) as isize,
                    );
        i += 1;
        i;
    }
    return ans;
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
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, subarraySum(nums, n));
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
