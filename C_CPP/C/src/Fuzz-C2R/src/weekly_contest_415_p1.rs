use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getSneakyNumbers(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = calloc(
        2 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        let mut j: libc::c_int = i + 1 as libc::c_int;
        while j < numsSize {
            if *nums.offset(i as isize) == *nums.offset(j as isize) {
                *result.offset(count as isize) = *nums.offset(i as isize);
                count += 1;
                count;
                break;
            } else {
                j += 1;
                j;
            }
        }
        if count == 2 as libc::c_int {
            break;
        }
        i += 1;
        i;
    }
    *returnSize = 2 as libc::c_int;
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut numSize: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut numSize as *mut libc::c_int);
    numSize += 2 as libc::c_int;
    let mut nums: *mut libc::c_int = calloc(
        numSize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
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
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_int = getSneakyNumbers(nums, numSize, &mut returnSize);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *result.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    free(nums as *mut libc::c_void);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
