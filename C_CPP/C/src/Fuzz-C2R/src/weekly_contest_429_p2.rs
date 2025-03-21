use ::libc;
extern "C" {
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
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[no_mangle]
pub unsafe extern "C" fn cmp_int_asc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(a as *mut libc::c_int) - *(b as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn maxDistinctElements(
    mut arr: *mut libc::c_int,
    mut arrSize: libc::c_int,
    mut diff: libc::c_int,
) -> libc::c_int {
    let mut prev: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut distinctCount: libc::c_int = 0 as libc::c_int;
    qsort(
        arr as *mut libc::c_void,
        arrSize as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            cmp_int_asc
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < arrSize {
        let mut x: libc::c_int = if prev + 1 as libc::c_int
            > *arr.offset(i as isize) - diff
        {
            prev + 1 as libc::c_int
        } else {
            *arr.offset(i as isize) - diff
        };
        if x <= *arr.offset(i as isize) + diff {
            distinctCount += 1;
            distinctCount;
            prev = x;
        }
        i += 1;
        i;
    }
    return distinctCount;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut diff: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut diff as *mut libc::c_int);
    let mut arr: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *arr.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = maxDistinctElements(arr, n, diff);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(arr as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
