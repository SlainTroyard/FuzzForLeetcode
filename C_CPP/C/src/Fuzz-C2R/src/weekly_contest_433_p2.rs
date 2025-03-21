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
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub static mut F: [libc::c_longlong; 100000] = [0; 100000];
#[no_mangle]
pub static mut INV_F: [libc::c_longlong; 100000] = [0; 100000];
#[no_mangle]
pub unsafe extern "C" fn compare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_int) - *(b as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn power(
    mut x: libc::c_longlong,
    mut n: libc::c_int,
) -> libc::c_longlong {
    let mut res: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    while n > 0 as libc::c_int {
        if n % 2 as libc::c_int == 1 as libc::c_int {
            res = res * x % 1000000007 as libc::c_int as libc::c_longlong;
        }
        x = x * x % 1000000007 as libc::c_int as libc::c_longlong;
        n /= 2 as libc::c_int;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn comb(
    mut n: libc::c_int,
    mut m: libc::c_int,
) -> libc::c_longlong {
    if m > n {
        return 0 as libc::c_int as libc::c_longlong;
    }
    return F[n as usize] * INV_F[m as usize]
        % 1000000007 as libc::c_int as libc::c_longlong * INV_F[(n - m) as usize]
        % 1000000007 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn initialize() {
    F[0 as libc::c_int as usize] = 1 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < 100000 as libc::c_int {
        F[i
            as usize] = F[(i - 1 as libc::c_int) as usize] * i as libc::c_longlong
            % 1000000007 as libc::c_int as libc::c_longlong;
        i += 1;
        i;
    }
    INV_F[(100000 as libc::c_int - 1 as libc::c_int)
        as usize] = power(
        F[(100000 as libc::c_int - 1 as libc::c_int) as usize],
        1000000007 as libc::c_int - 2 as libc::c_int,
    );
    let mut i_0: libc::c_int = 100000 as libc::c_int - 1 as libc::c_int;
    while i_0 > 0 as libc::c_int {
        INV_F[(i_0 - 1 as libc::c_int)
            as usize] = INV_F[i_0 as usize] * i_0 as libc::c_longlong
            % 1000000007 as libc::c_int as libc::c_longlong;
        i_0 -= 1;
        i_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn minMaxSums(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    static mut initialized: bool = 0 as libc::c_int != 0;
    if !initialized {
        initialize();
        initialized = 1 as libc::c_int != 0;
    }
    qsort(
        nums as *mut libc::c_void,
        numsSize as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            compare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut s: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        ans = (ans
            + s
                * (*nums.offset(i as isize)
                    + *nums.offset((numsSize - 1 as libc::c_int - i) as isize))
                    as libc::c_longlong) % 1000000007 as libc::c_int as libc::c_longlong;
        s = (s * 2 as libc::c_int as libc::c_longlong - comb(i, k - 1 as libc::c_int)
            + 1000000007 as libc::c_int as libc::c_longlong)
            % 1000000007 as libc::c_int as libc::c_longlong;
        i += 1;
        i;
    }
    return ans as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error reading input for n and k\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut nums: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if nums.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for nums array\n\0" as *const u8
                as *const libc::c_char,
        );
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
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut result: libc::c_int = minMaxSums(nums, n, k);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
