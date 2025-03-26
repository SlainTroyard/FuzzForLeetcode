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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DFSContext {
    pub memo: *mut *mut *mut libc::c_longlong,
    pub cost: *mut *mut libc::c_int,
    pub n: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn dfs(
    mut ctx: *mut DFSContext,
    mut i: libc::c_int,
    mut pre_j: libc::c_int,
    mut pre_k: libc::c_int,
) -> libc::c_longlong {
    if i < 0 as libc::c_int {
        return 0 as libc::c_int as libc::c_longlong;
    }
    if *(*(*((*ctx).memo).offset(i as isize)).offset(pre_j as isize))
        .offset(pre_k as isize) != -(1 as libc::c_int) as libc::c_longlong
    {
        return *(*(*((*ctx).memo).offset(i as isize)).offset(pre_j as isize))
            .offset(pre_k as isize);
    }
    let mut min_res: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if !(j == pre_j) {
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < 3 as libc::c_int {
                if k != pre_k && k != j {
                    let mut temp: libc::c_longlong = dfs(ctx, i - 1 as libc::c_int, j, k)
                        + *(*((*ctx).cost).offset(i as isize)).offset(j as isize)
                            as libc::c_longlong
                        + *(*((*ctx).cost)
                            .offset(((*ctx).n - 1 as libc::c_int - i) as isize))
                            .offset(k as isize) as libc::c_longlong;
                    if temp < min_res {
                        min_res = temp;
                    }
                }
                k += 1;
                k;
            }
        }
        j += 1;
        j;
    }
    *(*(*((*ctx).memo).offset(i as isize)).offset(pre_j as isize))
        .offset(pre_k as isize) = min_res;
    return min_res;
}
#[no_mangle]
pub unsafe extern "C" fn minCost(
    mut n: libc::c_int,
    mut cost: *mut *mut libc::c_int,
    mut costSize: libc::c_int,
    mut costColSize: *mut libc::c_int,
) -> libc::c_longlong {
    let mut memo: *mut *mut *mut libc::c_longlong = malloc(
        ((n / 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_longlong>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n / 2 as libc::c_int {
        let ref mut fresh0 = *memo.offset(i as isize);
        *fresh0 = malloc(
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_longlong>() as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_longlong;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            let ref mut fresh1 = *(*memo.offset(i as isize)).offset(j as isize);
            *fresh1 = malloc(
                (4 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                    ),
            ) as *mut libc::c_longlong;
            let mut k: libc::c_int = 0 as libc::c_int;
            while k < 4 as libc::c_int {
                *(*(*memo.offset(i as isize)).offset(j as isize))
                    .offset(k as isize) = -(1 as libc::c_int) as libc::c_longlong;
                k += 1;
                k;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    let mut ctx: DFSContext = DFSContext {
        memo: 0 as *mut *mut *mut libc::c_longlong,
        cost: 0 as *mut *mut libc::c_int,
        n: 0,
    };
    ctx.memo = memo;
    ctx.cost = cost;
    ctx.n = n;
    let mut result: libc::c_longlong = dfs(
        &mut ctx,
        n / 2 as libc::c_int - 1 as libc::c_int,
        3 as libc::c_int,
        3 as libc::c_int,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n / 2 as libc::c_int {
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 4 as libc::c_int {
            free(
                *(*memo.offset(i_0 as isize)).offset(j_0 as isize) as *mut libc::c_void,
            );
            j_0 += 1;
            j_0;
        }
        free(*memo.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(memo as *mut libc::c_void);
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
    let mut cost: *mut *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut costColSize: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if cost.is_null() || costColSize.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh2 = *cost.offset(i as isize);
        *fresh2 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *costColSize.offset(i as isize) = 3 as libc::c_int;
        if (*cost.offset(i as isize)).is_null() {
            fprintf(
                stderr,
                b"Memory allocation failed for cost[%d]\n\0" as *const u8
                    as *const libc::c_char,
                i,
            );
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < i {
                free(*cost.offset(j as isize) as *mut libc::c_void);
                j += 1;
                j;
            }
            free(cost as *mut libc::c_void);
            free(costColSize as *mut libc::c_void);
            return 1 as libc::c_int;
        }
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 3 as libc::c_int {
            if scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*cost.offset(i as isize)).offset(j_0 as isize) as *mut libc::c_int,
            ) != 1 as libc::c_int
            {
                fprintf(
                    stderr,
                    b"Error reading input for cost[%d][%d]\n\0" as *const u8
                        as *const libc::c_char,
                    i,
                    j_0,
                );
                let mut k: libc::c_int = 0 as libc::c_int;
                while k <= i {
                    free(*cost.offset(k as isize) as *mut libc::c_void);
                    k += 1;
                    k;
                }
                free(cost as *mut libc::c_void);
                free(costColSize as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            j_0 += 1;
            j_0;
        }
        i += 1;
        i;
    }
    let mut result: libc::c_longlong = minCost(n, cost, n, costColSize);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        free(*cost.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(cost as *mut libc::c_void);
    free(costColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
