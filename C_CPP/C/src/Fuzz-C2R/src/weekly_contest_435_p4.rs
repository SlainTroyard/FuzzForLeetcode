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
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn min(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn maxDifference(
    mut s: *mut libc::c_char,
    mut k: libc::c_int,
) -> libc::c_int {
    let inf: libc::c_int = 2147483647 as libc::c_int / 2 as libc::c_int;
    let mut ans: libc::c_int = -inf;
    let mut len: libc::c_int = strlen(s) as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    while x < 5 as libc::c_int {
        let mut y: libc::c_int = 0 as libc::c_int;
        while y < 5 as libc::c_int {
            if !(y == x) {
                let mut cur_s: [libc::c_int; 5] = [0 as libc::c_int, 0, 0, 0, 0];
                let mut pre_s: [libc::c_int; 5] = [0 as libc::c_int, 0, 0, 0, 0];
                let mut min_s: [[libc::c_int; 2]; 2] = [[inf, inf], [inf, inf]];
                let mut left: libc::c_int = 0 as libc::c_int;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < len {
                    cur_s[(*s.offset(i as isize) as libc::c_int - '0' as i32) as usize]
                        += 1;
                    cur_s[(*s.offset(i as isize) as libc::c_int - '0' as i32) as usize];
                    let mut r: libc::c_int = i + 1 as libc::c_int;
                    while r - left >= k && cur_s[x as usize] > pre_s[x as usize]
                        && cur_s[y as usize] > pre_s[y as usize]
                    {
                        let mut p: *mut libc::c_int = &mut *(*min_s
                            .as_mut_ptr()
                            .offset(
                                (*pre_s.as_mut_ptr().offset(x as isize) & 1 as libc::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr()
                            .offset(
                                (*pre_s.as_mut_ptr().offset(y as isize) & 1 as libc::c_int)
                                    as isize,
                            ) as *mut libc::c_int;
                        *p = min(*p, pre_s[x as usize] - pre_s[y as usize]);
                        pre_s[(*s.offset(left as isize) as libc::c_int - '0' as i32)
                            as usize] += 1;
                        pre_s[(*s.offset(left as isize) as libc::c_int - '0' as i32)
                            as usize];
                        left += 1;
                        left;
                    }
                    ans = max(
                        ans,
                        cur_s[x as usize] - cur_s[y as usize]
                            - min_s[(cur_s[x as usize] & 1 as libc::c_int
                                ^ 1 as libc::c_int)
                                as usize][(cur_s[y as usize] & 1 as libc::c_int) as usize],
                    );
                    i += 1;
                    i;
                }
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100001] = [0; 100001];
    let mut k: libc::c_int = 0;
    if scanf(
        b"%s %d\0" as *const u8 as *const libc::c_char,
        s.as_mut_ptr(),
        &mut k as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        fprintf(stderr, b"Error reading input\n\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    let mut result: libc::c_int = maxDifference(s.as_mut_ptr(), k);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
