use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn constructTransformedArray(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut result: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *returnSize = numsSize;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        if *nums.offset(i as isize) == 0 as libc::c_int {
            *result.offset(i as isize) = *nums.offset(i as isize);
        } else {
            let mut steps: libc::c_int = *nums.offset(i as isize);
            let mut targetIndex: libc::c_int = (i + steps) % numsSize;
            if targetIndex < 0 as libc::c_int {
                targetIndex += numsSize;
            }
            *result.offset(i as isize) = *nums.offset(targetIndex as isize);
        }
        i += 1;
        i;
    }
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
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
    let mut returnSize: libc::c_int = 0;
    let mut transformedArray: *mut libc::c_int = constructTransformedArray(
        nums,
        numsSize,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *transformedArray.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(nums as *mut libc::c_void);
    free(transformedArray as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
