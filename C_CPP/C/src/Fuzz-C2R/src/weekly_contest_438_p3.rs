use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
pub unsafe extern "C" fn binom_mod2(
    mut n: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    while n != 0 || k != 0 {
        if k & 1 as libc::c_int > n & 1 as libc::c_int {
            return 0 as libc::c_int;
        }
        n >>= 1 as libc::c_int;
        k >>= 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn binom_mod5(
    mut n: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    static mut table: [[libc::c_int; 5]; 5] = [
        [
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            1 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            1 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            1 as libc::c_int,
            3 as libc::c_int,
            3 as libc::c_int,
            1 as libc::c_int,
            0 as libc::c_int,
        ],
        [
            1 as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
            4 as libc::c_int,
            1 as libc::c_int,
        ],
    ];
    let mut res: libc::c_int = 1 as libc::c_int;
    while n != 0 || k != 0 {
        let mut n_i: libc::c_int = n % 5 as libc::c_int;
        let mut k_i: libc::c_int = k % 5 as libc::c_int;
        if k_i > n_i {
            return 0 as libc::c_int;
        }
        res = res * table[n_i as usize][k_i as usize] % 5 as libc::c_int;
        n /= 5 as libc::c_int;
        k /= 5 as libc::c_int;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn combine_mod10(
    mut r2: libc::c_int,
    mut r5: libc::c_int,
) -> libc::c_int {
    if r5 % 2 as libc::c_int == r2 {
        return r5 % 10 as libc::c_int
    } else {
        return (r5 + 5 as libc::c_int) % 10 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn binom_mod10(
    mut n: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut r2: libc::c_int = binom_mod2(n, k);
    let mut r5: libc::c_int = binom_mod5(n, k);
    return combine_mod10(r2, r5);
}
#[no_mangle]
pub unsafe extern "C" fn hasSameDigits(mut s: *mut libc::c_char) -> bool {
    let mut n: libc::c_int = strlen(s) as libc::c_int;
    let mut D: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < n - 1 as libc::c_int {
        let mut weight: libc::c_int = binom_mod10(n - 2 as libc::c_int, j);
        let mut d1: libc::c_int = *s.offset(j as isize) as libc::c_int - '0' as i32;
        let mut d2: libc::c_int = *s.offset((j + 1 as libc::c_int) as isize)
            as libc::c_int - '0' as i32;
        let mut diff: libc::c_int = d1 - d2;
        diff = (diff % 10 as libc::c_int + 10 as libc::c_int) % 10 as libc::c_int;
        let mut contrib: libc::c_int = weight * diff % 10 as libc::c_int;
        D = (D + contrib) % 10 as libc::c_int;
        j += 1;
        j;
    }
    return D % 10 as libc::c_int == 0 as libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100001] = [0; 100001];
    if scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr())
        != 1 as libc::c_int
    {
        fprintf(stderr, b"Error reading input\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut result: bool = hasSameDigits(s.as_mut_ptr());
    printf(
        b"%s\n\0" as *const u8 as *const libc::c_char,
        if result as libc::c_int != 0 {
            b"true\0" as *const u8 as *const libc::c_char
        } else {
            b"false\0" as *const u8 as *const libc::c_char
        },
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
