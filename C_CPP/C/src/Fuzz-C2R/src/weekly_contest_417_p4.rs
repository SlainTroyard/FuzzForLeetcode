use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn kchar_search(
    mut k: libc::c_longlong,
    mut operations: *mut libc::c_int,
    mut pos: libc::c_int,
) -> libc::c_char {
    let mut pow_sum: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    let mut tmp_pos: libc::c_int = 0 as libc::c_int;
    if pos == 0 || 1 as libc::c_int as libc::c_longlong == k {
        if *operations.offset(pos as isize) != 0 {
            return 'b' as i32 as libc::c_char;
        }
        return 'a' as i32 as libc::c_char;
    }
    while k > pow_sum {
        pow_sum *= 2 as libc::c_int as libc::c_longlong;
        tmp_pos += 1;
        tmp_pos;
    }
    if *operations.offset(pos as isize) != 0 {
        let mut kchar: libc::c_char = kchar_search(
            k - pow_sum / 2 as libc::c_int as libc::c_longlong,
            operations,
            tmp_pos - 1 as libc::c_int,
        );
        kchar += 1;
        if kchar as libc::c_int > 'z' as i32 {
            return 'a' as i32 as libc::c_char;
        }
        return kchar;
    }
    return kchar_search(
        k - pow_sum / 2 as libc::c_int as libc::c_longlong,
        operations,
        tmp_pos - 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn kthCharacter(
    mut k: libc::c_longlong,
    mut operations: *mut libc::c_int,
    mut operationsSize: libc::c_int,
) -> libc::c_char {
    let mut pow_sum: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
    let mut pos: libc::c_int = 0 as libc::c_int;
    if 1 as libc::c_int as libc::c_longlong == k {
        return 'a' as i32 as libc::c_char;
    }
    while pow_sum < k {
        pow_sum *= 2 as libc::c_int as libc::c_longlong;
        pos += 1;
        pos;
    }
    return kchar_search(
        k - pow_sum / 2 as libc::c_int as libc::c_longlong,
        operations,
        pos - 1 as libc::c_int,
    );
}
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_longlong = 0;
    let mut operationSize: libc::c_int = 0;
    scanf(
        b"%lld %d\0" as *const u8 as *const libc::c_char,
        &mut k as *mut libc::c_longlong,
        &mut operationSize as *mut libc::c_int,
    );
    let mut operations: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(operationSize as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < operationSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *operations.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    printf(
        b"%c\n\0" as *const u8 as *const libc::c_char,
        kthCharacter(k, operations, operationSize) as libc::c_int,
    );
    free(operations as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
