use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn hasIncreasingSubarrays(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
) -> bool {
    if k == 1 as libc::c_int {
        return 1 as libc::c_int != 0;
    }
    let mut a: libc::c_int = 0 as libc::c_int;
    let mut s: libc::c_int = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < numsSize - 2 as libc::c_int * k + 1 as libc::c_int {
            s = i + k;
            a = 0 as libc::c_int;
            let mut z: libc::c_int = 0 as libc::c_int;
            while z < k - 1 as libc::c_int {
                if *nums.offset((i + z) as isize)
                    < *nums.offset((i + z + 1 as libc::c_int) as isize)
                    && *nums.offset((s + z) as isize)
                        < *nums.offset((s + z + 1 as libc::c_int) as isize)
                {
                    a += 1 as libc::c_int;
                }
                z += 1;
                z;
            }
            if a == k - 1 as libc::c_int {
                return 1 as libc::c_int != 0;
            }
            i += 1;
            i;
        }
        j += 1;
        j;
    }
    return 0 as libc::c_int != 0;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let vla = n as usize;
    let mut nums: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    if hasIncreasingSubarrays(nums.as_mut_ptr(), n, k) {
        printf(b"true\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"false\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
