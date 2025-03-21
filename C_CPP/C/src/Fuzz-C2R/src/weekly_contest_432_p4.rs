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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stack {
    pub data: *mut libc::c_int,
    pub top: libc::c_int,
    pub capacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Deque {
    pub data: *mut libc::c_int,
    pub front: libc::c_int,
    pub rear: libc::c_int,
    pub size: libc::c_int,
    pub capacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector {
    pub data: *mut libc::c_int,
    pub size: libc::c_int,
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
pub unsafe extern "C" fn stackIsEmpty(mut stack: *mut Stack) -> bool {
    return (*stack).top == -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn stackPush(mut stack: *mut Stack, mut item: libc::c_int) {
    if (*stack).top == (*stack).capacity - 1 as libc::c_int {
        return;
    }
    (*stack).top += 1;
    *((*stack).data).offset((*stack).top as isize) = item;
}
#[no_mangle]
pub unsafe extern "C" fn stackPop(mut stack: *mut Stack) -> libc::c_int {
    if stackIsEmpty(stack) {
        return -(1 as libc::c_int);
    }
    let fresh0 = (*stack).top;
    (*stack).top = (*stack).top - 1;
    return *((*stack).data).offset(fresh0 as isize);
}
#[no_mangle]
pub unsafe extern "C" fn stackTop(mut stack: *mut Stack) -> libc::c_int {
    if stackIsEmpty(stack) {
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
pub unsafe extern "C" fn createDeque(mut capacity: libc::c_int) -> *mut Deque {
    let mut deque: *mut Deque = malloc(::core::mem::size_of::<Deque>() as libc::c_ulong)
        as *mut Deque;
    if deque.is_null() {
        return 0 as *mut Deque;
    }
    (*deque)
        .data = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*deque).data).is_null() {
        free(deque as *mut libc::c_void);
        return 0 as *mut Deque;
    }
    (*deque).front = 0 as libc::c_int;
    (*deque).rear = -(1 as libc::c_int);
    (*deque).size = 0 as libc::c_int;
    (*deque).capacity = capacity;
    return deque;
}
#[no_mangle]
pub unsafe extern "C" fn dequeIsEmpty(mut deque: *mut Deque) -> bool {
    return (*deque).size == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dequePushBack(mut deque: *mut Deque, mut item: libc::c_int) {
    if (*deque).size == (*deque).capacity {
        return;
    }
    (*deque).rear = ((*deque).rear + 1 as libc::c_int) % (*deque).capacity;
    *((*deque).data).offset((*deque).rear as isize) = item;
    (*deque).size += 1;
    (*deque).size;
}
#[no_mangle]
pub unsafe extern "C" fn dequePopBack(mut deque: *mut Deque) -> libc::c_int {
    if dequeIsEmpty(deque) {
        return -(1 as libc::c_int);
    }
    let mut item: libc::c_int = *((*deque).data).offset((*deque).rear as isize);
    (*deque)
        .rear = ((*deque).rear - 1 as libc::c_int + (*deque).capacity)
        % (*deque).capacity;
    (*deque).size -= 1;
    (*deque).size;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn dequePopFront(mut deque: *mut Deque) -> libc::c_int {
    if dequeIsEmpty(deque) {
        return -(1 as libc::c_int);
    }
    let mut item: libc::c_int = *((*deque).data).offset((*deque).front as isize);
    (*deque).front = ((*deque).front + 1 as libc::c_int) % (*deque).capacity;
    (*deque).size -= 1;
    (*deque).size;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn dequeFront(mut deque: *mut Deque) -> libc::c_int {
    if dequeIsEmpty(deque) {
        return -(1 as libc::c_int);
    }
    return *((*deque).data).offset((*deque).front as isize);
}
#[no_mangle]
pub unsafe extern "C" fn dequeBack(mut deque: *mut Deque) -> libc::c_int {
    if dequeIsEmpty(deque) {
        return -(1 as libc::c_int);
    }
    return *((*deque).data).offset((*deque).rear as isize);
}
#[no_mangle]
pub unsafe extern "C" fn destroyDeque(mut deque: *mut Deque) {
    free((*deque).data as *mut libc::c_void);
    free(deque as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn createVector(mut capacity: libc::c_int) -> *mut Vector {
    let mut vector: *mut Vector = malloc(
        ::core::mem::size_of::<Vector>() as libc::c_ulong,
    ) as *mut Vector;
    if vector.is_null() {
        return 0 as *mut Vector;
    }
    (*vector)
        .data = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*vector).data).is_null() {
        free(vector as *mut libc::c_void);
        return 0 as *mut Vector;
    }
    (*vector).size = 0 as libc::c_int;
    (*vector).capacity = capacity;
    return vector;
}
#[no_mangle]
pub unsafe extern "C" fn vectorPushBack(mut vector: *mut Vector, mut item: libc::c_int) {
    if (*vector).size == (*vector).capacity {
        let mut newCapacity: libc::c_int = (*vector).capacity * 2 as libc::c_int;
        let mut newData: *mut libc::c_int = realloc(
            (*vector).data as *mut libc::c_void,
            (newCapacity as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        if newData.is_null() {
            return;
        }
        (*vector).data = newData;
        (*vector).capacity = newCapacity;
    }
    let fresh1 = (*vector).size;
    (*vector).size = (*vector).size + 1;
    *((*vector).data).offset(fresh1 as isize) = item;
}
#[no_mangle]
pub unsafe extern "C" fn destroyVector(mut vector: *mut Vector) {
    free((*vector).data as *mut libc::c_void);
    free(vector as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn countNonDecreasingSubarrays(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut g: *mut *mut Vector = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Vector>() as libc::c_ulong),
    ) as *mut *mut Vector;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        let ref mut fresh2 = *g.offset(i as isize);
        *fresh2 = createVector(10 as libc::c_int);
        i += 1;
        i;
    }
    let mut pos_r: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < numsSize {
        *pos_r.offset(i_0 as isize) = numsSize;
        i_0 += 1;
        i_0;
    }
    let mut st: *mut Stack = createStack(numsSize);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < numsSize {
        let mut x: libc::c_int = *nums.offset(i_1 as isize);
        while !stackIsEmpty(st) && x >= *nums.offset(stackTop(st) as isize) {
            *pos_r.offset(stackTop(st) as isize) = i_1;
            stackPop(st);
        }
        if !stackIsEmpty(st) {
            vectorPushBack(*g.offset(stackTop(st) as isize), i_1);
        }
        stackPush(st, i_1);
        i_1 += 1;
        i_1;
    }
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut cnt: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0 as libc::c_int;
    let mut q: *mut Deque = createDeque(numsSize);
    let mut r: libc::c_int = 0 as libc::c_int;
    while r < numsSize {
        let mut x_0: libc::c_int = *nums.offset(r as isize);
        while !dequeIsEmpty(q) && *nums.offset(dequeBack(q) as isize) <= x_0 {
            dequePopBack(q);
        }
        dequePushBack(q, r);
        cnt += *nums.offset(dequeFront(q) as isize) - x_0;
        while cnt > k {
            let mut out: libc::c_int = *nums.offset(l as isize);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < (**g.offset(l as isize)).size {
                let mut i_2: libc::c_int = *((**g.offset(l as isize)).data)
                    .offset(j as isize);
                if i_2 > r {
                    break;
                }
                let mut min_pos: libc::c_int = if *pos_r.offset(i_2 as isize)
                    < r + 1 as libc::c_int
                {
                    *pos_r.offset(i_2 as isize)
                } else {
                    r + 1 as libc::c_int
                };
                cnt -= (out - *nums.offset(i_2 as isize)) * (min_pos - i_2);
                j += 1;
                j;
            }
            l += 1;
            l;
            if !dequeIsEmpty(q) && dequeFront(q) < l {
                dequePopFront(q);
            }
        }
        ans += (r - l + 1 as libc::c_int) as libc::c_longlong;
        r += 1;
        r;
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < numsSize {
        destroyVector(*g.offset(i_3 as isize));
        i_3 += 1;
        i_3;
    }
    free(g as *mut libc::c_void);
    free(pos_r as *mut libc::c_void);
    destroyStack(st);
    destroyDeque(q);
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut numsSize: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    if scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error reading input for numsSize and k\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut nums: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
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
    while i < numsSize {
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
    let mut result: libc::c_longlong = countNonDecreasingSubarrays(nums, numsSize, k);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
