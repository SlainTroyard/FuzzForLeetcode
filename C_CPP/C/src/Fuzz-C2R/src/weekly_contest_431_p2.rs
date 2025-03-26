use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Stack {
    pub data: *mut libc::c_int,
    pub size: libc::c_int,
    pub capacity: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn initStack(mut s: *mut Stack, mut capacity: libc::c_int) {
    (*s)
        .data = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*s).size = 0 as libc::c_int;
    (*s).capacity = capacity;
}
#[no_mangle]
pub unsafe extern "C" fn pushStack(mut s: *mut Stack, mut value: libc::c_int) {
    if (*s).size < (*s).capacity {
        let fresh0 = (*s).size;
        (*s).size = (*s).size + 1;
        *((*s).data).offset(fresh0 as isize) = value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn popStack(mut s: *mut Stack) -> libc::c_int {
    if (*s).size > 0 as libc::c_int {
        (*s).size -= 1;
        return *((*s).data).offset((*s).size as isize);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn topStack(mut s: *mut Stack) -> libc::c_int {
    if (*s).size > 0 as libc::c_int {
        return *((*s).data).offset(((*s).size - 1 as libc::c_int) as isize);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn isEmptyStack(mut s: *mut Stack) -> libc::c_int {
    return ((*s).size == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeStack(mut s: *mut Stack) {
    free((*s).data as *mut libc::c_void);
    (*s).size = 0 as libc::c_int;
    (*s).capacity = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn calculateScore(mut s: *mut libc::c_char) -> libc::c_longlong {
    let mut len: libc::c_int = strlen(s) as libc::c_int;
    let mut stacks: [Stack; 26] = [Stack {
        data: 0 as *mut libc::c_int,
        size: 0,
        capacity: 0,
    }; 26];
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 26 as libc::c_int {
        initStack(&mut *stacks.as_mut_ptr().offset(i as isize), len);
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < len {
        let mut c: libc::c_int = *s.offset(i_0 as isize) as libc::c_int - 'a' as i32;
        if isEmptyStack(
            &mut *stacks.as_mut_ptr().offset((25 as libc::c_int - c) as isize),
        ) == 0
        {
            ans
                += (i_0
                    - topStack(
                        &mut *stacks
                            .as_mut_ptr()
                            .offset((25 as libc::c_int - c) as isize),
                    )) as libc::c_longlong;
            popStack(&mut *stacks.as_mut_ptr().offset((25 as libc::c_int - c) as isize));
        } else {
            pushStack(&mut *stacks.as_mut_ptr().offset(c as isize), i_0);
        }
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < 26 as libc::c_int {
        freeStack(&mut *stacks.as_mut_ptr().offset(i_1 as isize));
        i_1 += 1;
        i_1;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100001] = [0; 100001];
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    printf(
        b"%lld\n\0" as *const u8 as *const libc::c_char,
        calculateScore(s.as_mut_ptr()),
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
