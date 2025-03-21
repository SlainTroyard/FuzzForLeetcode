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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
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
pub unsafe extern "C" fn popcount(mut n: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    while n != 0 {
        count += n & 1 as libc::c_int;
        n >>= 1 as libc::c_int;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn hasCycle(
    mut sub: libc::c_int,
    mut g: *mut [libc::c_int; 26],
    mut gSize: *mut libc::c_int,
) -> bool {
    let mut color: [libc::c_int; 26] = [
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
    let mut start: libc::c_int = 0 as libc::c_int;
    while start < 26 as libc::c_int {
        if !(sub >> start & 1 as libc::c_int == 0 as libc::c_int
            || color[start as usize] == 2 as libc::c_int)
        {
            let mut stack: [libc::c_int; 26] = [0; 26];
            let mut stackPos: [libc::c_int; 26] = [0; 26];
            let mut top: libc::c_int = 0 as libc::c_int;
            stack[top as usize] = start;
            stackPos[top as usize] = 0 as libc::c_int;
            color[start as usize] = 1 as libc::c_int;
            while top >= 0 as libc::c_int {
                let mut x: libc::c_int = stack[top as usize];
                if stackPos[top as usize] >= *gSize.offset(x as isize) {
                    color[x as usize] = 2 as libc::c_int;
                    top -= 1;
                    top;
                } else {
                    let fresh0 = stackPos[top as usize];
                    stackPos[top as usize] = stackPos[top as usize] + 1;
                    let mut y: libc::c_int = (*g.offset(x as isize))[fresh0 as usize];
                    if sub >> y & 1 as libc::c_int == 0 as libc::c_int {
                        continue;
                    }
                    if color[y as usize] == 1 as libc::c_int {
                        return 1 as libc::c_int != 0;
                    }
                    if color[y as usize] == 0 as libc::c_int {
                        color[y as usize] = 1 as libc::c_int;
                        top += 1;
                        stack[top as usize] = y;
                        stackPos[top as usize] = 0 as libc::c_int;
                    }
                }
            }
        }
        start += 1;
        start;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn supersequences(
    mut words: *mut *mut libc::c_char,
    mut wordsSize: libc::c_int,
    mut returnSize: *mut libc::c_int,
    mut returnColumnSizes: *mut *mut libc::c_int,
) -> *mut *mut libc::c_int {
    let mut all: libc::c_int = 0 as libc::c_int;
    let mut mask2: libc::c_int = 0 as libc::c_int;
    let mut g: [[libc::c_int; 26]; 26] = [[0; 26]; 26];
    let mut gSize: [libc::c_int; 26] = [
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
    while i < wordsSize {
        let mut x: libc::c_int = *(*words.offset(i as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int - 'a' as i32;
        let mut y: libc::c_int = *(*words.offset(i as isize))
            .offset(1 as libc::c_int as isize) as libc::c_int - 'a' as i32;
        all |= (1 as libc::c_int) << x | (1 as libc::c_int) << y;
        if x == y {
            mask2 |= (1 as libc::c_int) << x;
        }
        let fresh1 = gSize[x as usize];
        gSize[x as usize] = gSize[x as usize] + 1;
        g[x as usize][fresh1 as usize] = y;
        i += 1;
        i;
    }
    let mut mask1: libc::c_int = all ^ mask2;
    let mut validSubsets: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut validSubsetsCapacity: libc::c_int = 0 as libc::c_int;
    let mut validSubsetsCount: libc::c_int = 0 as libc::c_int;
    let mut maxSize: libc::c_int = 0 as libc::c_int;
    let mut sub: libc::c_int = mask1;
    loop {
        let mut size: libc::c_int = popcount(sub);
        if size >= maxSize && !hasCycle(sub, g.as_mut_ptr(), gSize.as_mut_ptr()) {
            if size > maxSize {
                maxSize = size;
                validSubsetsCount = 0 as libc::c_int;
            }
            if validSubsetsCount >= validSubsetsCapacity {
                validSubsetsCapacity = if validSubsetsCapacity == 0 as libc::c_int {
                    16 as libc::c_int
                } else {
                    validSubsetsCapacity * 2 as libc::c_int
                };
                let mut newArray: *mut libc::c_int = realloc(
                    validSubsets as *mut libc::c_void,
                    (validSubsetsCapacity as libc::c_ulong)
                        .wrapping_mul(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ),
                ) as *mut libc::c_int;
                if newArray.is_null() {
                    free(validSubsets as *mut libc::c_void);
                    *returnSize = 0 as libc::c_int;
                    return 0 as *mut *mut libc::c_int;
                }
                validSubsets = newArray;
            }
            let fresh2 = validSubsetsCount;
            validSubsetsCount = validSubsetsCount + 1;
            *validSubsets.offset(fresh2 as isize) = sub;
        }
        if sub == 0 as libc::c_int {
            break;
        }
        sub = sub - 1 as libc::c_int & mask1;
        if !(sub != mask1) {
            break;
        }
    }
    *returnSize = validSubsetsCount;
    let mut result: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    if validSubsetsCount > 0 as libc::c_int {
        result = malloc(
            (validSubsetsCount as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_int;
        if result.is_null() {
            free(validSubsets as *mut libc::c_void);
            *returnSize = 0 as libc::c_int;
            return 0 as *mut *mut libc::c_int;
        }
        *returnColumnSizes = malloc(
            (validSubsetsCount as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if (*returnColumnSizes).is_null() {
            free(validSubsets as *mut libc::c_void);
            free(result as *mut libc::c_void);
            *returnSize = 0 as libc::c_int;
            return 0 as *mut *mut libc::c_int;
        }
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < validSubsetsCount {
            let mut sub_0: libc::c_int = *validSubsets.offset(i_0 as isize);
            let ref mut fresh3 = *result.offset(i_0 as isize);
            *fresh3 = malloc(
                (26 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            if (*result.offset(i_0 as isize)).is_null() {
                let mut j: libc::c_int = 0 as libc::c_int;
                while j < i_0 {
                    free(*result.offset(j as isize) as *mut libc::c_void);
                    j += 1;
                    j;
                }
                free(*returnColumnSizes as *mut libc::c_void);
                free(result as *mut libc::c_void);
                free(validSubsets as *mut libc::c_void);
                *returnSize = 0 as libc::c_int;
                return 0 as *mut *mut libc::c_int;
            }
            *(*returnColumnSizes).offset(i_0 as isize) = 26 as libc::c_int;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < 26 as libc::c_int {
                *(*result.offset(i_0 as isize))
                    .offset(
                        j_0 as isize,
                    ) = (all >> j_0 & 1 as libc::c_int)
                    + ((all ^ sub_0) >> j_0 & 1 as libc::c_int);
                j_0 += 1;
                j_0;
            }
            i_0 += 1;
            i_0;
        }
    }
    free(validSubsets as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    if scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int)
        != 1 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error reading input for n\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut words: *mut *mut libc::c_char = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if words.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for words array\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh4 = *words.offset(i as isize);
        *fresh4 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
        ) as *mut libc::c_char;
        if (*words.offset(i as isize)).is_null() {
            fprintf(
                stderr,
                b"Memory allocation failed for words[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                free(*words.offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free(words as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        if scanf(b"%2s\0" as *const u8 as *const libc::c_char, *words.offset(i as isize))
            != 1 as libc::c_int
        {
            fprintf(
                stderr,
                b"Error reading input for words[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 <= i {
                free(*words.offset(j_0 as isize) as *mut libc::c_void);
                j_0 += 1;
                j_0;
            }
            free(words as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut returnColumnSizes: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut result: *mut *mut libc::c_int = supersequences(
        words,
        n,
        &mut returnSize,
        &mut returnColumnSizes,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < *returnColumnSizes.offset(i_0 as isize) {
            printf(
                b"%d \0" as *const u8 as *const libc::c_char,
                *(*result.offset(i_0 as isize)).offset(j_1 as isize),
            );
            j_1 += 1;
            j_1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < returnSize {
        free(*result.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(result as *mut libc::c_void);
    free(returnColumnSizes as *mut libc::c_void);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n {
        free(*words.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(words as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
