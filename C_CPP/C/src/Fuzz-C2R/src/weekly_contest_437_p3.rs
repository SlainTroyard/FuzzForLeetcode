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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Interval {
    pub right: libc::c_int,
    pub left: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn min(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn compare_intervals(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (*(a as *mut Interval)).right - (*(b as *mut Interval)).right;
}
#[no_mangle]
pub unsafe extern "C" fn lower_bound(
    mut arr: *mut libc::c_int,
    mut size: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut low: libc::c_int = 0 as libc::c_int;
    let mut high: libc::c_int = size;
    while low < high {
        let mut mid: libc::c_int = low + (high - low) / 2 as libc::c_int;
        if *arr.offset(mid as isize) < val {
            low = mid + 1 as libc::c_int;
        } else {
            high = mid;
        }
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C" fn upper_bound(
    mut arr: *mut libc::c_int,
    mut size: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut low: libc::c_int = 0 as libc::c_int;
    let mut high: libc::c_int = size;
    while low < high {
        let mut mid: libc::c_int = low + (high - low) / 2 as libc::c_int;
        if *arr.offset(mid as isize) <= val {
            low = mid + 1 as libc::c_int;
        } else {
            high = mid;
        }
    }
    return low;
}
#[no_mangle]
pub unsafe extern "C" fn maxSubstringLength(
    mut s: *mut libc::c_char,
    mut k: libc::c_int,
) -> bool {
    let mut n: libc::c_int = strlen(s) as libc::c_int;
    let mut pos: [*mut libc::c_int; 26] = [0 as *mut libc::c_int; 26];
    let mut pos_sizes: [libc::c_int; 26] = [
        0 as libc::c_int,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        pos[i
            as usize] = malloc(
            (n as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if (pos[i as usize]).is_null() {
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                free(pos[j as usize] as *mut libc::c_void);
                j += 1;
                j;
            }
            return 0 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let mut c: libc::c_int = *s.offset(i_0 as isize) as libc::c_int - 'a' as i32;
        let fresh0 = pos_sizes[c as usize];
        pos_sizes[c as usize] = pos_sizes[c as usize] + 1;
        *(pos[c as usize]).offset(fresh0 as isize) = i_0;
        i_0 += 1;
        i_0;
    }
    let mut intervals: *mut Interval = malloc(
        (26 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<Interval>() as libc::c_ulong),
    ) as *mut Interval;
    if intervals.is_null() {
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 26 as libc::c_int {
            free(pos[i_1 as usize] as *mut libc::c_void);
            i_1 += 1;
            i_1;
        }
        return 0 as libc::c_int != 0;
    }
    let mut interval_count: libc::c_int = 0 as libc::c_int;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 26 as libc::c_int {
        if pos_sizes[i_2 as usize] > 0 as libc::c_int {
            let mut l: libc::c_int = *(pos[i_2 as usize])
                .offset(0 as libc::c_int as isize);
            let mut r: libc::c_int = *(pos[i_2 as usize])
                .offset((pos_sizes[i_2 as usize] - 1 as libc::c_int) as isize);
            let mut flag: bool = 1 as libc::c_int != 0;
            while flag {
                flag = 0 as libc::c_int != 0;
                let mut j_0: libc::c_int = 0 as libc::c_int;
                while j_0 < 26 as libc::c_int {
                    if pos_sizes[j_0 as usize] > 0 as libc::c_int {
                        let mut low_idx: libc::c_int = lower_bound(
                            pos[j_0 as usize],
                            pos_sizes[j_0 as usize],
                            l,
                        );
                        let mut up_idx: libc::c_int = upper_bound(
                            pos[j_0 as usize],
                            pos_sizes[j_0 as usize],
                            r,
                        );
                        let mut cnt: libc::c_int = up_idx - low_idx;
                        if cnt > 0 as libc::c_int && cnt < pos_sizes[j_0 as usize] {
                            l = min(
                                l,
                                *(pos[j_0 as usize]).offset(0 as libc::c_int as isize),
                            );
                            r = max(
                                r,
                                *(pos[j_0 as usize])
                                    .offset(
                                        (pos_sizes[j_0 as usize] - 1 as libc::c_int) as isize,
                                    ),
                            );
                            flag = 1 as libc::c_int != 0;
                        }
                    }
                    j_0 += 1;
                    j_0;
                }
            }
            if l > 0 as libc::c_int || r < n - 1 as libc::c_int {
                (*intervals.offset(interval_count as isize)).right = r;
                (*intervals.offset(interval_count as isize)).left = l;
                interval_count += 1;
                interval_count;
            }
        }
        i_2 += 1;
        i_2;
    }
    qsort(
        intervals as *mut libc::c_void,
        interval_count as size_t,
        ::core::mem::size_of::<Interval>() as libc::c_ulong,
        Some(
            compare_intervals
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut R: libc::c_int = -(1 as libc::c_int);
    let mut cnt_0: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < interval_count {
        if (*intervals.offset(i_3 as isize)).left > R {
            R = (*intervals.offset(i_3 as isize)).right;
            cnt_0 += 1;
            cnt_0;
        }
        i_3 += 1;
        i_3;
    }
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < 26 as libc::c_int {
        free(pos[i_4 as usize] as *mut libc::c_void);
        i_4 += 1;
        i_4;
    }
    free(intervals as *mut libc::c_void);
    return cnt_0 >= k;
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
    let mut result: bool = maxSubstringLength(s.as_mut_ptr(), k);
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
