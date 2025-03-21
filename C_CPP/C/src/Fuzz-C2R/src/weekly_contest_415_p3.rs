use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HashSet {
    pub data: *mut libc::c_int,
    pub size: libc::c_int,
    pub capacity: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn initHashSet(mut set: *mut HashSet) {
    (*set).data = 0 as *mut libc::c_int;
    (*set).size = 0 as libc::c_int;
    (*set).capacity = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addHash(mut set: *mut HashSet, mut hash: libc::c_int) {
    if (*set).size == (*set).capacity {
        (*set)
            .capacity = if (*set).capacity != 0 {
            (*set).capacity * 2 as libc::c_int
        } else {
            16 as libc::c_int
        };
        (*set)
            .data = realloc(
            (*set).data as *mut libc::c_void,
            ((*set).capacity as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if ((*set).data).is_null() {
            fprintf(
                stderr,
                b"Memory allocation failed.\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    let fresh0 = (*set).size;
    (*set).size = (*set).size + 1;
    *((*set).data).offset(fresh0 as isize) = hash;
}
#[no_mangle]
pub unsafe extern "C" fn cmp_int(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut x: libc::c_int = *(a as *const libc::c_int);
    let mut y: libc::c_int = *(b as *const libc::c_int);
    if x < y {
        return -(1 as libc::c_int);
    }
    if x > y {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sortAndUniqueHashSet(mut set: *mut HashSet) {
    if (*set).size == 0 as libc::c_int {
        return;
    }
    qsort(
        (*set).data as *mut libc::c_void,
        (*set).size as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            cmp_int
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut unique_size: libc::c_int = 1 as libc::c_int;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < (*set).size {
        if *((*set).data).offset(i as isize)
            != *((*set).data).offset((unique_size - 1 as libc::c_int) as isize)
        {
            let fresh1 = unique_size;
            unique_size = unique_size + 1;
            *((*set).data).offset(fresh1 as isize) = *((*set).data).offset(i as isize);
        }
        i += 1;
        i;
    }
    (*set).size = unique_size;
}
#[no_mangle]
pub unsafe extern "C" fn getRandomBase() -> libc::c_uint {
    return (800000000 as libc::c_int + rand() % 100000000 as libc::c_int)
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn minValidStrings(
    mut words: *mut *mut libc::c_char,
    mut wordsSize: libc::c_int,
    mut target: *mut libc::c_char,
) -> libc::c_int {
    srand(time(0 as *mut time_t) as libc::c_uint);
    let mut n: libc::c_int = 0 as libc::c_int;
    while *target.offset(n as isize) as libc::c_int != '\0' as i32 {
        n += 1;
        n;
    }
    let mut max_len: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < wordsSize {
        let mut len: libc::c_int = 0 as libc::c_int;
        while *(*words.offset(i as isize)).offset(len as isize) as libc::c_int
            != '\0' as i32
        {
            len += 1;
            len;
        }
        if len > max_len {
            max_len = len;
        }
        i += 1;
        i;
    }
    let mut base: libc::c_uint = getRandomBase();
    let mut pow_base: *mut libc::c_ulonglong = malloc(
        ((n + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong),
    ) as *mut libc::c_ulonglong;
    if pow_base.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for pow_base.\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    *pow_base.offset(0 as libc::c_int as isize) = 1 as libc::c_int as libc::c_ulonglong;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        *pow_base
            .offset(
                (i_0 + 1 as libc::c_int) as isize,
            ) = (*pow_base.offset(i_0 as isize))
            .wrapping_mul(base as libc::c_ulonglong)
            .wrapping_rem(1070777777 as libc::c_int as libc::c_ulonglong);
        i_0 += 1;
        i_0;
    }
    let mut pre_hash: *mut libc::c_ulonglong = malloc(
        ((n + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_ulonglong>() as libc::c_ulong),
    ) as *mut libc::c_ulonglong;
    if pre_hash.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for pre_hash.\n\0" as *const u8
                as *const libc::c_char,
        );
        free(pow_base as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    *pre_hash.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_ulonglong;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n {
        *pre_hash
            .offset(
                (i_1 + 1 as libc::c_int) as isize,
            ) = (*pre_hash.offset(i_1 as isize))
            .wrapping_mul(base as libc::c_ulonglong)
            .wrapping_add(
                *target.offset(i_1 as isize) as libc::c_uchar as libc::c_ulonglong,
            )
            .wrapping_rem(1070777777 as libc::c_int as libc::c_ulonglong);
        i_1 += 1;
        i_1;
    }
    let mut sets: *mut HashSet = malloc(
        (max_len as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<HashSet>() as libc::c_ulong),
    ) as *mut HashSet;
    if sets.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for sets.\n\0" as *const u8 as *const libc::c_char,
        );
        free(pow_base as *mut libc::c_void);
        free(pre_hash as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < max_len {
        initHashSet(&mut *sets.offset(j as isize));
        j += 1;
        j;
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < wordsSize {
        let mut len_0: libc::c_int = 0 as libc::c_int;
        while *(*words.offset(i_2 as isize)).offset(len_0 as isize) as libc::c_int
            != '\0' as i32 && len_0 < max_len
        {
            len_0 += 1;
            len_0;
        }
        let mut h: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < len_0 {
            h = h
                .wrapping_mul(base as libc::c_ulonglong)
                .wrapping_add(
                    *(*words.offset(i_2 as isize)).offset(j_0 as isize) as libc::c_uchar
                        as libc::c_ulonglong,
                )
                .wrapping_rem(1070777777 as libc::c_int as libc::c_ulonglong);
            addHash(&mut *sets.offset(j_0 as isize), h as libc::c_int);
            j_0 += 1;
            j_0;
        }
        i_2 += 1;
        i_2;
    }
    let mut j_1: libc::c_int = 0 as libc::c_int;
    while j_1 < max_len {
        sortAndUniqueHashSet(&mut *sets.offset(j_1 as isize));
        j_1 += 1;
        j_1;
    }
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut cur_r: libc::c_int = 0 as libc::c_int;
    let mut nxt_r: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < n {
        while nxt_r < n && nxt_r - i_3 < max_len {
            if nxt_r + 1 as libc::c_int > n {
                break;
            }
            let mut current_hash: libc::c_ulonglong = (*pre_hash
                .offset((nxt_r + 1 as libc::c_int) as isize))
                .wrapping_add(1070777777 as libc::c_int as libc::c_ulonglong)
                .wrapping_sub(
                    (*pre_hash.offset(i_3 as isize))
                        .wrapping_mul(
                            *pow_base.offset((nxt_r + 1 as libc::c_int - i_3) as isize),
                        )
                        .wrapping_rem(1070777777 as libc::c_int as libc::c_ulonglong),
                )
                .wrapping_rem(1070777777 as libc::c_int as libc::c_ulonglong);
            let mut sh: libc::c_int = current_hash as libc::c_int;
            let mut prefix_len: libc::c_int = nxt_r - i_3;
            if prefix_len < 0 as libc::c_int || prefix_len >= max_len {
                break;
            }
            let mut set: *mut HashSet = &mut *sets.offset(prefix_len as isize)
                as *mut HashSet;
            let mut left: libc::c_int = 0 as libc::c_int;
            let mut right: libc::c_int = (*set).size - 1 as libc::c_int;
            let mut found: libc::c_int = 0 as libc::c_int;
            while left <= right {
                let mut mid: libc::c_int = left + (right - left) / 2 as libc::c_int;
                if *((*set).data).offset(mid as isize) == sh {
                    found = 1 as libc::c_int;
                    break;
                } else if *((*set).data).offset(mid as isize) < sh {
                    left = mid + 1 as libc::c_int;
                } else {
                    right = mid - 1 as libc::c_int;
                }
            }
            if !(found != 0) {
                break;
            }
            nxt_r += 1;
            nxt_r;
        }
        if i_3 == cur_r {
            if i_3 == nxt_r {
                let mut j_2: libc::c_int = 0 as libc::c_int;
                while j_2 < max_len {
                    free((*sets.offset(j_2 as isize)).data as *mut libc::c_void);
                    j_2 += 1;
                    j_2;
                }
                free(sets as *mut libc::c_void);
                free(pow_base as *mut libc::c_void);
                free(pre_hash as *mut libc::c_void);
                return -(1 as libc::c_int);
            }
            cur_r = nxt_r;
            ans += 1;
            ans;
        }
        i_3 += 1;
        i_3;
    }
    let mut j_3: libc::c_int = 0 as libc::c_int;
    while j_3 < max_len {
        free((*sets.offset(j_3 as isize)).data as *mut libc::c_void);
        j_3 += 1;
        j_3;
    }
    free(sets as *mut libc::c_void);
    free(pow_base as *mut libc::c_void);
    free(pre_hash as *mut libc::c_void);
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut wordsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut wordsSize as *mut libc::c_int,
    );
    let mut words: *mut *mut libc::c_char = malloc(
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(wordsSize as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut wordsColSize: libc::c_int = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < wordsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut wordsColSize as *mut libc::c_int,
        );
        let ref mut fresh2 = *words.offset(i as isize);
        *fresh2 = malloc(
            (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul((wordsColSize + 1 as libc::c_int) as libc::c_ulong),
        ) as *mut libc::c_char;
        scanf(b"%s\0" as *const u8 as *const libc::c_char, *words.offset(i as isize));
        i += 1;
        i;
    }
    let mut targetSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut targetSize as *mut libc::c_int,
    );
    let mut target: *mut libc::c_char = malloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul((targetSize + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, target);
    let mut res: libc::c_int = minValidStrings(words, wordsSize, target);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, res);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < wordsSize {
        free(*words.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(words as *mut libc::c_void);
    free(target as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
