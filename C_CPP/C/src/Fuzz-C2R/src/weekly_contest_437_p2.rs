use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return *(b as *mut libc::c_int) - *(a as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn maxWeight(
    mut pizzas: *mut libc::c_int,
    mut pizzasSize: libc::c_int,
) -> libc::c_longlong {
    let mut day: libc::c_int = pizzasSize / 4 as libc::c_int;
    let mut even: libc::c_int = day / 2 as libc::c_int;
    let mut odd: libc::c_int = (day + 1 as libc::c_int) / 2 as libc::c_int;
    qsort(
        pizzas as *mut libc::c_void,
        pizzasSize as size_t,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        Some(
            cmp
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut total: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < odd {
        let fresh0 = index;
        index = index + 1;
        total += *pizzas.offset(fresh0 as isize) as libc::c_longlong;
        i += 1;
        i;
    }
    index += 1;
    index;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < even {
        total += *pizzas.offset(index as isize) as libc::c_longlong;
        index += 2 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    return total;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
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
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, maxWeight(arr, n));
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
