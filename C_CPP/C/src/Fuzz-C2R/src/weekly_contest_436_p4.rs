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
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn min_element(
    mut arr: *mut libc::c_int,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut min_val: libc::c_int = *arr.offset(0 as libc::c_int as isize);
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        if *arr.offset(i as isize) < min_val {
            min_val = *arr.offset(i as isize);
        }
        i += 1;
        i;
    }
    return min_val;
}
#[no_mangle]
pub unsafe extern "C" fn check(
    mut points: *mut libc::c_int,
    mut pointsSize: libc::c_int,
    mut m: libc::c_int,
    mut low: libc::c_longlong,
) -> bool {
    let mut n: libc::c_int = pointsSize;
    let mut rem: libc::c_int = m;
    let mut pre: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let mut k: libc::c_int = ((low - 1 as libc::c_int as libc::c_longlong)
            / *points.offset(i as isize) as libc::c_longlong
            + 1 as libc::c_int as libc::c_longlong - pre as libc::c_longlong)
            as libc::c_int;
        if i == n - 1 as libc::c_int && k <= 0 as libc::c_int {
            break;
        }
        k = max(k, 1 as libc::c_int);
        rem -= k * 2 as libc::c_int - 1 as libc::c_int;
        if rem < 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
        pre = k - 1 as libc::c_int;
        i += 1;
        i;
    }
    return 1 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn maxScore(
    mut points: *mut libc::c_int,
    mut pointsSize: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_longlong {
    let mut left: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut right: libc::c_longlong = (m + 1 as libc::c_int) as libc::c_longlong
        / 2 as libc::c_int as libc::c_longlong
        * min_element(points, pointsSize) as libc::c_longlong
        + 1 as libc::c_int as libc::c_longlong;
    while (left + 1 as libc::c_int as libc::c_longlong) < right {
        let mut mid: libc::c_longlong = left
            + (right - left) / 2 as libc::c_int as libc::c_longlong;
        if check(points, pointsSize, m, mid) {
            left = mid;
        } else {
            right = mid;
        }
    }
    return left;
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
    let mut points: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if points.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *points.offset(i as isize) as *mut libc::c_int,
        ) != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error reading input for points[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            free(points as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut result: libc::c_longlong = maxScore(points, n, m);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    free(points as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
