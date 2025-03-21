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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct Stack {
    pub data: *mut libc::c_int,
    pub top: libc::c_int,
    pub capacity: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn createStack(mut capacity: libc::c_int) -> *mut Stack {
    let mut stack: *mut Stack = malloc(::core::mem::size_of::<Stack>() as libc::c_ulong)
        as *mut Stack;
    if stack.is_null() {
        return 0 as *mut Stack;
    }
    (*stack)
        .data = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*stack).data).is_null() {
        free(stack as *mut libc::c_void);
        return 0 as *mut Stack;
    }
    (*stack).top = -(1 as libc::c_int);
    (*stack).capacity = capacity;
    return stack;
}
#[no_mangle]
pub unsafe extern "C" fn isEmpty(mut stack: *mut Stack) -> bool {
    return (*stack).top == -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn size(mut stack: *mut Stack) -> libc::c_int {
    return (*stack).top + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn push(mut stack: *mut Stack, mut value: libc::c_int) {
    if (*stack).top == (*stack).capacity - 1 as libc::c_int {
        return;
    }
    (*stack).top += 1;
    *((*stack).data).offset((*stack).top as isize) = value;
}
#[no_mangle]
pub unsafe extern "C" fn pop(mut stack: *mut Stack) -> libc::c_int {
    if isEmpty(stack) {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*stack).top;
    (*stack).top = (*stack).top - 1;
    return *((*stack).data).offset(fresh0 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn top(mut stack: *mut Stack) -> libc::c_int {
    if isEmpty(stack) {
        return -(1 as libc::c_int);
    }
    return *((*stack).data).offset((*stack).top as isize);
}
#[no_mangle]
pub unsafe extern "C" fn destroyStack(mut stack: *mut Stack) {
    free((*stack).data as *mut libc::c_void);
    free(stack as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn count(
    mut m: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    if m > k {
        return (m * 2 as libc::c_int - k + 1 as libc::c_int) as libc::c_longlong
            * k as libc::c_longlong / 2 as libc::c_int as libc::c_longlong
    } else {
        return (m + 1 as libc::c_int) as libc::c_longlong * m as libc::c_longlong
            / 2 as libc::c_int as libc::c_longlong
    };
}
#[no_mangle]
pub unsafe extern "C" fn sumSubarrayMins(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut res: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut stack: *mut Stack = createStack(numsSize + 1 as libc::c_int);
    push(stack, -(1 as libc::c_int));
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < numsSize {
        while size(stack) > 1 as libc::c_int
            && *nums.offset(top(stack) as isize) >= *nums.offset(r as isize)
        {
            let mut i: libc::c_int = pop(stack);
            let mut l: libc::c_int = top(stack);
            let mut cnt: libc::c_longlong = count(r - l - 1 as libc::c_int, k)
                - count(i - l - 1 as libc::c_int, k)
                - count(r - i - 1 as libc::c_int, k);
            res += *nums.offset(i as isize) as libc::c_longlong * cnt;
        }
        push(stack, r);
        r += 1;
        r;
    }
    destroyStack(stack);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn minMaxSubarraySum(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut temp: *mut libc::c_int = malloc(
        ((numsSize + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if temp.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    memcpy(
        temp as *mut libc::c_void,
        nums as *const libc::c_void,
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    *temp
        .offset(
            numsSize as isize,
        ) = (-(2147483647 as libc::c_int) - 1 as libc::c_int) / 2 as libc::c_int;
    let mut ans: libc::c_longlong = sumSubarrayMins(
        temp,
        numsSize + 1 as libc::c_int,
        k,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize + 1 as libc::c_int {
        *temp.offset(i as isize) = -*temp.offset(i as isize);
        i += 1;
        i;
    }
    *temp.offset(numsSize as isize) = -*temp.offset(numsSize as isize);
    ans -= sumSubarrayMins(temp, numsSize + 1 as libc::c_int, k);
    free(temp as *mut libc::c_void);
    return ans;
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
    let mut result: libc::c_longlong = minMaxSubarraySum(nums, n, k);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
