use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fmin(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn beautifulSplits(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let vla = numsSize as usize;
    let mut cnt0: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut kmpcnt: libc::c_int = 0 as libc::c_int;
    *cnt0.as_mut_ptr().offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < numsSize {
        while kmpcnt > 0 as libc::c_int
            && *nums.offset(i as isize) != *nums.offset(kmpcnt as isize)
        {
            kmpcnt = *cnt0.as_mut_ptr().offset((kmpcnt - 1 as libc::c_int) as isize);
        }
        if *nums.offset(i as isize) == *nums.offset(kmpcnt as isize) {
            kmpcnt += 1;
            kmpcnt;
        }
        *cnt0.as_mut_ptr().offset(i as isize) = kmpcnt;
        if i % 2 as libc::c_int != 0
            && 0 as libc::c_int
                == (i + 1 as libc::c_int) / 2 as libc::c_int
                    % (i + 1 as libc::c_int - kmpcnt)
        {
            res += numsSize - i - 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 < numsSize {
        let vla_0 = (numsSize - i_0) as usize;
        let mut cnt: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
        let mut end: libc::c_int = numsSize;
        kmpcnt = 0 as libc::c_int;
        *cnt.as_mut_ptr().offset(0 as libc::c_int as isize) = 0 as libc::c_int;
        if 2 as libc::c_int * i_0 < numsSize
            && 0 as libc::c_int
                == i_0
                    % (2 as libc::c_int * i_0
                        - *cnt0
                            .as_mut_ptr()
                            .offset(
                                (2 as libc::c_int * i_0 - 1 as libc::c_int) as isize,
                            ))
        {
            end = fmin(end as libc::c_double, (3 as libc::c_int * i_0) as libc::c_double)
                as libc::c_int;
        }
        let mut j: libc::c_int = i_0 + 1 as libc::c_int;
        while j < end {
            while kmpcnt > 0 as libc::c_int
                && *nums.offset(j as isize) != *nums.offset((i_0 + kmpcnt) as isize)
            {
                kmpcnt = *cnt.as_mut_ptr().offset((kmpcnt - 1 as libc::c_int) as isize);
            }
            if *nums.offset(j as isize) == *nums.offset((i_0 + kmpcnt) as isize) {
                kmpcnt += 1;
                kmpcnt;
            }
            *cnt.as_mut_ptr().offset((j - i_0) as isize) = kmpcnt;
            if 0 as libc::c_int == (j - i_0 + 1 as libc::c_int) % 2 as libc::c_int
                && 0 as libc::c_int
                    == (j - i_0 + 1 as libc::c_int) / 2 as libc::c_int
                        % (j - i_0 + 1 as libc::c_int - kmpcnt)
            {
                let mut len: libc::c_int = j - i_0 + 1 as libc::c_int;
                let mut div: libc::c_int = i_0 - 1 as libc::c_int
                    + len / 2 as libc::c_int;
                if len == i_0 * 2 as libc::c_int
                    && 0 as libc::c_int
                        == i_0
                            % (div + 1 as libc::c_int
                                - *cnt0.as_mut_ptr().offset(div as isize))
                {
                    break;
                }
                res += 1 as libc::c_int;
            }
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    return res;
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
    let mut result: libc::c_int = beautifulSplits(nums, n);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
