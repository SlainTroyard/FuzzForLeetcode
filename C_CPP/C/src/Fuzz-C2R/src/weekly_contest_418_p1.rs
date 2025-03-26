use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn maxGoodNumber(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut nums1: libc::c_int = 0 as libc::c_int;
    let mut num2: libc::c_int = 0 as libc::c_int;
    let mut num3: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 7 as libc::c_int {
            if i == 0 as libc::c_int {
                let mut s: libc::c_int = *nums.offset(i as isize) << j
                    & 64 as libc::c_int;
                if !(s == 0 as libc::c_int) {
                    break;
                }
                nums1 += 1;
                nums1;
            } else if i == 1 as libc::c_int {
                let mut s_0: libc::c_int = *nums.offset(i as isize) << j
                    & 64 as libc::c_int;
                if !(s_0 == 0 as libc::c_int) {
                    break;
                }
                num2 += 1;
                num2;
            } else if i == 2 as libc::c_int {
                let mut s_1: libc::c_int = *nums.offset(i as isize) << j
                    & 64 as libc::c_int;
                if !(s_1 == 0 as libc::c_int) {
                    break;
                }
                num3 += 1;
                num3;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut times: *mut libc::c_int = calloc(
        3 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *times.offset(0 as libc::c_int as isize) = nums1;
    *times.offset(1 as libc::c_int as isize) = num2;
    *times.offset(2 as libc::c_int as isize) = num3;
    let mut store: *mut libc::c_int = calloc(
        3 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    *store.offset(1 as libc::c_int as isize) = 1 as libc::c_int;
    *store.offset(2 as libc::c_int as isize) = 2 as libc::c_int;
    let mut i_0: libc::c_int = 1 as libc::c_int;
    while i_0 <= 2 as libc::c_int {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 <= 1 as libc::c_int {
            let mut value1: libc::c_int = (*nums
                .offset(*store.offset(j_0 as isize) as isize)
                << 7 as libc::c_int
                    - *times
                        .offset(
                            *store.offset((j_0 + 1 as libc::c_int) as isize) as isize,
                        ))
                + *nums
                    .offset(*store.offset((j_0 + 1 as libc::c_int) as isize) as isize);
            let mut value2: libc::c_int = (*nums
                .offset(*store.offset((j_0 + 1 as libc::c_int) as isize) as isize)
                << 7 as libc::c_int
                    - *times.offset(*store.offset(j_0 as isize) as isize))
                + *nums.offset(*store.offset(j_0 as isize) as isize);
            if value2 >= value1 {
                let mut temp: libc::c_int = *store.offset(j_0 as isize);
                *store
                    .offset(
                        j_0 as isize,
                    ) = *store.offset((j_0 + 1 as libc::c_int) as isize);
                *store.offset((j_0 + 1 as libc::c_int) as isize) = temp;
            }
            j_0 += 1;
            j_0;
        }
        i_0 += 1;
        i_0;
    }
    return (*nums.offset(*store.offset(0 as libc::c_int as isize) as isize)
        << 14 as libc::c_int
            - *times.offset(*store.offset(1 as libc::c_int as isize) as isize)
            - *times.offset(*store.offset(2 as libc::c_int as isize) as isize))
        + (*nums.offset(*store.offset(1 as libc::c_int as isize) as isize)
            << 7 as libc::c_int
                - *times.offset(*store.offset(2 as libc::c_int as isize) as isize))
        + *nums.offset(*store.offset(2 as libc::c_int as isize) as isize);
}
unsafe fn main_0() -> libc::c_int {
    let mut numSize: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut numSize as *mut libc::c_int);
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
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, maxGoodNumber(nums, numSize));
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
