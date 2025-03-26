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
pub unsafe extern "C" fn countSubstrings(mut s: *mut libc::c_char) -> libc::c_longlong {
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut f: [[libc::c_int; 9]; 10] = [
        [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
    ];
    let mut len: libc::c_int = strlen(s) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        let mut d: libc::c_int = *s.offset(i as isize) as libc::c_int - '0' as i32;
        let mut m: libc::c_int = 1 as libc::c_int;
        while m < 10 as libc::c_int {
            let mut nf: [libc::c_int; 9] = [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0];
            nf[(d % m) as usize] = 1 as libc::c_int;
            let mut rem: libc::c_int = 0 as libc::c_int;
            while rem < m {
                nf[((rem * 10 as libc::c_int + d) % m) as usize]
                    += f[m as usize][rem as usize];
                rem += 1;
                rem;
            }
            let mut rem_0: libc::c_int = 0 as libc::c_int;
            while rem_0 < m {
                f[m as usize][rem_0 as usize] = nf[rem_0 as usize];
                rem_0 += 1;
                rem_0;
            }
            m += 1;
            m;
        }
        ans += f[d as usize][0 as libc::c_int as usize] as libc::c_longlong;
        i += 1;
        i;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100001] = [0; 100001];
    if scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr())
        != 1 as libc::c_int
    {
        fprintf(stderr, b"Error reading input\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut result: libc::c_longlong = countSubstrings(s.as_mut_ptr());
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
