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
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
pub unsafe extern "C" fn str_to_num(mut str: *mut libc::c_char) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_ulong) < strlen(str as *const libc::c_char) {
        if *str.offset(i as isize) as libc::c_int >= '0' as i32
            && *str.offset(i as isize) as libc::c_int <= '9' as i32
        {
            num *= 10 as libc::c_int;
            num += *str.offset(i as isize) as libc::c_int - '0' as i32;
        }
        i += 1;
        i;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn extract_id_number(
    mut id_str: *const libc::c_char,
) -> libc::c_int {
    let mut num_start: *const libc::c_char = id_str;
    while *num_start as libc::c_int != 0
        && !(*num_start as libc::c_int >= '0' as i32
            && *num_start as libc::c_int <= '9' as i32)
    {
        num_start = num_start.offset(1);
        num_start;
    }
    if *num_start as libc::c_int == '\0' as i32 {
        return -(1 as libc::c_int);
    }
    return str_to_num(num_start as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn countMentions(
    mut numberOfUsers: libc::c_int,
    mut events: *mut *mut *mut libc::c_char,
    mut eventsSize: libc::c_int,
    mut eventsColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let vla = eventsSize as usize;
    let mut time_arr: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < eventsSize {
        *time_arr
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = str_to_num(
            *(*events.offset(i as isize)).offset(1 as libc::c_int as isize),
        );
        i += 1;
        i;
    }
    let vla_0 = eventsSize as usize;
    let mut order: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < eventsSize {
        *order.as_mut_ptr().offset(i_0 as isize) = i_0;
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = eventsSize - 1 as libc::c_int;
    while i_1 > 0 as libc::c_int {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < i_1 {
            if *time_arr
                .as_mut_ptr()
                .offset(
                    *order.as_mut_ptr().offset((j + 1 as libc::c_int) as isize) as isize,
                )
                < *time_arr
                    .as_mut_ptr()
                    .offset(*order.as_mut_ptr().offset(j as isize) as isize)
                || *time_arr
                    .as_mut_ptr()
                    .offset(
                        *order.as_mut_ptr().offset((j + 1 as libc::c_int) as isize)
                            as isize,
                    )
                    == *time_arr
                        .as_mut_ptr()
                        .offset(*order.as_mut_ptr().offset(j as isize) as isize)
                    && *(*(*events
                        .offset(
                            *order.as_mut_ptr().offset((j + 1 as libc::c_int) as isize)
                                as isize,
                        ))
                        .offset(0 as libc::c_int as isize))
                        .offset(0 as libc::c_int as isize) as libc::c_int == 'O' as i32
            {
                let mut t: libc::c_int = *order.as_mut_ptr().offset(j as isize);
                *order
                    .as_mut_ptr()
                    .offset(
                        j as isize,
                    ) = *order.as_mut_ptr().offset((j + 1 as libc::c_int) as isize);
                *order.as_mut_ptr().offset((j + 1 as libc::c_int) as isize) = t;
            }
            j += 1;
            j;
        }
        i_1 -= 1;
        i_1;
    }
    let vla_1 = numberOfUsers as usize;
    let mut online: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    let mut mention: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numberOfUsers as libc::c_ulong),
    ) as *mut libc::c_int;
    if mention.is_null() {
        *returnSize = 0 as libc::c_int;
        return 0 as *mut libc::c_int;
    }
    memset(
        online.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla_1 * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    memset(
        mention as *mut libc::c_void,
        0 as libc::c_int,
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(numberOfUsers as libc::c_ulong),
    );
    *returnSize = numberOfUsers;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < eventsSize {
        let mut i_2: libc::c_int = *order.as_mut_ptr().offset(n as isize);
        if *(*(*events.offset(i_2 as isize)).offset(0 as libc::c_int as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int == 'M' as i32
        {
            if *(*(*events.offset(i_2 as isize)).offset(2 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == 'A' as i32
            {
                let mut j_0: libc::c_int = 0 as libc::c_int;
                while j_0 < numberOfUsers {
                    *mention.offset(j_0 as isize) += 1 as libc::c_int;
                    j_0 += 1;
                    j_0;
                }
            } else if *(*(*events.offset(i_2 as isize))
                .offset(2 as libc::c_int as isize))
                .offset(0 as libc::c_int as isize) as libc::c_int == 'H' as i32
            {
                let mut time: libc::c_int = str_to_num(
                    *(*events.offset(i_2 as isize)).offset(1 as libc::c_int as isize),
                );
                let mut j_1: libc::c_int = 0 as libc::c_int;
                while j_1 < numberOfUsers {
                    if *online.as_mut_ptr().offset(j_1 as isize) == 0 as libc::c_int {
                        *mention.offset(j_1 as isize) += 1 as libc::c_int;
                    } else if *online.as_mut_ptr().offset(j_1 as isize) <= time {
                        *online.as_mut_ptr().offset(j_1 as isize) = 0 as libc::c_int;
                        *mention.offset(j_1 as isize) += 1 as libc::c_int;
                    }
                    j_1 += 1;
                    j_1;
                }
            } else {
                let mut t_id: [libc::c_char; 100] = [0; 100];
                let mut prev: *mut libc::c_char = *(*events.offset(i_2 as isize))
                    .offset(2 as libc::c_int as isize);
                let mut space: *mut libc::c_char = strchr(prev, ' ' as i32);
                loop {
                    memset(
                        t_id.as_mut_ptr() as *mut libc::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                    );
                    if space.is_null() {
                        strcpy(t_id.as_mut_ptr(), prev);
                        let mut user_id: libc::c_int = extract_id_number(
                            t_id.as_mut_ptr(),
                        );
                        if user_id >= 0 as libc::c_int && user_id < numberOfUsers {
                            *mention.offset(user_id as isize) += 1 as libc::c_int;
                        }
                        break;
                    } else {
                        let mut len: libc::c_int = space.offset_from(prev)
                            as libc::c_long as libc::c_int;
                        if (len as libc::c_ulong)
                            < ::core::mem::size_of::<[libc::c_char; 100]>()
                                as libc::c_ulong
                        {
                            strncpy(t_id.as_mut_ptr(), prev, len as libc::c_ulong);
                            t_id[len as usize] = '\0' as i32 as libc::c_char;
                            let mut user_id_0: libc::c_int = extract_id_number(
                                t_id.as_mut_ptr(),
                            );
                            if user_id_0 >= 0 as libc::c_int && user_id_0 < numberOfUsers
                            {
                                *mention.offset(user_id_0 as isize) += 1 as libc::c_int;
                            }
                        }
                        prev = space.offset(1 as libc::c_int as isize);
                        space = strchr(prev, ' ' as i32);
                    }
                }
            }
        } else if *(*(*events.offset(i_2 as isize)).offset(0 as libc::c_int as isize))
            .offset(0 as libc::c_int as isize) as libc::c_int == 'O' as i32
        {
            let mut user_id_1: libc::c_int = extract_id_number(
                *(*events.offset(i_2 as isize)).offset(2 as libc::c_int as isize),
            );
            if user_id_1 >= 0 as libc::c_int && user_id_1 < numberOfUsers {
                *online
                    .as_mut_ptr()
                    .offset(
                        user_id_1 as isize,
                    ) = str_to_num(
                    *(*events.offset(i_2 as isize)).offset(1 as libc::c_int as isize),
                ) + 60 as libc::c_int;
            }
        }
        n += 1;
        n;
    }
    return mention;
}
unsafe fn main_0() -> libc::c_int {
    let mut numberOfUsers: libc::c_int = 0;
    let mut eventsSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut numberOfUsers as *mut libc::c_int,
        &mut eventsSize as *mut libc::c_int,
    );
    let mut events: *mut *mut *mut libc::c_char = malloc(
        (eventsSize as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_char;
    let mut eventsColSize: *mut libc::c_int = malloc(
        (eventsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if events.is_null() || eventsColSize.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < eventsSize {
        let ref mut fresh0 = *events.offset(i as isize);
        *fresh0 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_char;
        if (*events.offset(i as isize)).is_null() {
            fprintf(
                stderr,
                b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            let ref mut fresh1 = *(*events.offset(i as isize)).offset(j as isize);
            *fresh1 = malloc(
                (100 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_char;
            if (*(*events.offset(i as isize)).offset(j as isize)).is_null() {
                fprintf(
                    stderr,
                    b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
                );
                return 1 as libc::c_int;
            }
            scanf(
                b"%s\0" as *const u8 as *const libc::c_char,
                *(*events.offset(i as isize)).offset(j as isize),
            );
            j += 1;
            j;
        }
        *eventsColSize.offset(i as isize) = 3 as libc::c_int;
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_int = countMentions(
        numberOfUsers,
        events,
        eventsSize,
        eventsColSize,
        &mut returnSize,
    );
    if !result.is_null() {
        printf(b"Mentions: \0" as *const u8 as *const libc::c_char);
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < returnSize {
            printf(
                b"%d \0" as *const u8 as *const libc::c_char,
                *result.offset(i_0 as isize),
            );
            i_0 += 1;
            i_0;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        free(result as *mut libc::c_void);
    } else {
        printf(
            b"Error: Failed to compute mentions\n\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < eventsSize {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 3 as libc::c_int {
            free(
                *(*events.offset(i_1 as isize)).offset(j_0 as isize) as *mut libc::c_void,
            );
            j_0 += 1;
            j_0;
        }
        free(*events.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(events as *mut libc::c_void);
    free(eventsColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
