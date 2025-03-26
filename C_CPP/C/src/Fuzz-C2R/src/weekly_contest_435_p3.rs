use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub unsafe extern "C" fn gcd(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    while b != 0 {
        let mut temp: libc::c_longlong = b;
        b = a % b;
        a = temp;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn min(
    mut a: libc::c_longlong,
    mut b: libc::c_longlong,
) -> libc::c_longlong {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn minimumIncrements(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut target: *mut libc::c_int,
    mut targetSize: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = numsSize;
    let mut m: libc::c_int = targetSize;
    let mut g: *mut libc::c_longlong = malloc(
        (((1 as libc::c_int) << m) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong),
    ) as *mut libc::c_longlong;
    if g.is_null() {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (1 as libc::c_int) << m {
        *g.offset(i as isize) = 1 as libc::c_int as libc::c_longlong;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < m {
            if i >> j & 1 as libc::c_int != 0 {
                *g
                    .offset(
                        i as isize,
                    ) = *g.offset(i as isize)
                    / gcd(
                        *g.offset(i as isize),
                        *target.offset(j as isize) as libc::c_longlong,
                    ) * *target.offset(j as isize) as libc::c_longlong;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let INF: libc::c_longlong = 1e18f64 as libc::c_longlong;
    let mut f: *mut *mut libc::c_longlong = malloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut libc::c_longlong>() as libc::c_ulong,
            ),
    ) as *mut *mut libc::c_longlong;
    if f.is_null() {
        free(g as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 2 as libc::c_int {
        let ref mut fresh0 = *f.offset(i_0 as isize);
        *fresh0 = malloc(
            (((1 as libc::c_int) << m) as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                ),
        ) as *mut libc::c_longlong;
        if (*f.offset(i_0 as isize)).is_null() {
            if i_0 > 0 as libc::c_int {
                free(*f.offset(0 as libc::c_int as isize) as *mut libc::c_void);
            }
            free(f as *mut libc::c_void);
            free(g as *mut libc::c_void);
            return -(1 as libc::c_int);
        }
        i_0 += 1;
        i_0;
    }
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < (1 as libc::c_int) << m {
        *(*f.offset(0 as libc::c_int as isize)).offset(j_0 as isize) = INF;
        j_0 += 1;
        j_0;
    }
    *(*f.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_longlong;
    let mut i_1: libc::c_int = 1 as libc::c_int;
    while i_1 <= n {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < (1 as libc::c_int) << m {
            *(*f.offset((i_1 & 1 as libc::c_int) as isize))
                .offset(
                    j_1 as isize,
                ) = *(*f.offset((i_1 & 1 as libc::c_int ^ 1 as libc::c_int) as isize))
                .offset(j_1 as isize);
            j_1 += 1;
            j_1;
        }
        let mut j_2: libc::c_int = 0 as libc::c_int;
        while j_2 < (1 as libc::c_int) << m {
            let mut k: libc::c_int = j_2;
            while k > 0 as libc::c_int {
                let mut v: libc::c_longlong = (*nums
                    .offset((i_1 - 1 as libc::c_int) as isize) as libc::c_longlong
                    + *g.offset(k as isize) - 1 as libc::c_int as libc::c_longlong)
                    / *g.offset(k as isize) * *g.offset(k as isize)
                    - *nums.offset((i_1 - 1 as libc::c_int) as isize)
                        as libc::c_longlong;
                *(*f.offset((i_1 & 1 as libc::c_int) as isize))
                    .offset(
                        j_2 as isize,
                    ) = min(
                    *(*f.offset((i_1 & 1 as libc::c_int) as isize)).offset(j_2 as isize),
                    *(*f.offset((i_1 & 1 as libc::c_int ^ 1 as libc::c_int) as isize))
                        .offset((j_2 ^ k) as isize) + v,
                );
                k = k - 1 as libc::c_int & j_2;
            }
            j_2 += 1;
            j_2;
        }
        i_1 += 1;
        i_1;
    }
    let mut result: libc::c_int = *(*f.offset((n & 1 as libc::c_int) as isize))
        .offset((((1 as libc::c_int) << m) - 1 as libc::c_int) as isize) as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 2 as libc::c_int {
        free(*f.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(f as *mut libc::c_void);
    free(g as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    if scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut m as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error reading input for n and m\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut nums: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut target: *mut libc::c_int = malloc(
        (m as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if nums.is_null() || target.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        if !nums.is_null() {
            free(nums as *mut libc::c_void);
        }
        if !target.is_null() {
            free(target as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        ) != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error reading input for nums[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            free(nums as *mut libc::c_void);
            free(target as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < m {
        if scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *target.offset(i_0 as isize) as *mut libc::c_int,
        ) != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error reading input for target[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i_0,
            );
            free(nums as *mut libc::c_void);
            free(target as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i_0 += 1;
        i_0;
    }
    let mut result: libc::c_int = minimumIncrements(nums, n, target, m);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    free(target as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
