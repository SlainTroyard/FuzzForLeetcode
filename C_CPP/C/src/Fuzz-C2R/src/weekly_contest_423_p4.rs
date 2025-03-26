use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
static mut hasCalc: libc::c_int = 0 as libc::c_int;
static mut digitsCnt: [libc::c_int; 801] = [0; 801];
static mut reducibleCnt: [libc::c_int; 801] = [0; 801];
static mut combVal: [[libc::c_int; 801]; 801] = [[0; 801]; 801];
#[no_mangle]
pub unsafe extern "C" fn countKReducibleNumbers(
    mut s: *mut libc::c_char,
    mut k: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut m: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = 0 as libc::c_int;
    let mut one: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    if 0 as libc::c_int == hasCalc {
        preProcess();
        hasCalc = 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while '\0' as i32 != *s.offset(i as isize) as libc::c_int {
        if '1' as i32 == *s.offset(i as isize) as libc::c_int {
            one += 1;
            one;
        }
        i += 1;
        i;
    }
    len = i;
    i = len - 1 as libc::c_int;
    while 0 as libc::c_int <= i {
        if '1' as i32 == *s.offset(i as isize) as libc::c_int {
            one -= 1;
            one;
            j = len - i - 1 as libc::c_int;
            m = 0 as libc::c_int;
            while j >= m {
                if (0 as libc::c_int) < one + m && k > reducibleCnt[(one + m) as usize] {
                    result = (result + combVal[j as usize][m as usize])
                        % 1000000007 as libc::c_int;
                }
                m += 1;
                m;
            }
        }
        i -= 1;
        i;
    }
    return result;
}
unsafe extern "C" fn preProcess() {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    digitsCnt[0 as libc::c_int as usize] = 0 as libc::c_int;
    reducibleCnt[0 as libc::c_int as usize] = 0 as libc::c_int;
    digitsCnt[1 as libc::c_int as usize] = 1 as libc::c_int;
    reducibleCnt[1 as libc::c_int as usize] = 0 as libc::c_int;
    combVal[0 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    combVal[1 as libc::c_int as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
    combVal[1 as libc::c_int as usize][1 as libc::c_int as usize] = 1 as libc::c_int;
    i = 2 as libc::c_int;
    while 801 as libc::c_int > i {
        digitsCnt[i
            as usize] = digitsCnt[(i >> 1 as libc::c_int) as usize]
            + (i & 1 as libc::c_int);
        reducibleCnt[i
            as usize] = reducibleCnt[digitsCnt[i as usize] as usize] + 1 as libc::c_int;
        combVal[i as usize][0 as libc::c_int as usize] = 1 as libc::c_int;
        combVal[i as usize][i as usize] = 1 as libc::c_int;
        j = 1 as libc::c_int;
        while i > j {
            combVal[i
                as usize][j
                as usize] = (combVal[(i - 1 as libc::c_int) as usize][j as usize]
                + combVal[(i - 1 as libc::c_int)
                    as usize][(j - 1 as libc::c_int) as usize])
                % 1000000007 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 1001] = [0; 1001];
    let mut k: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    let mut result: libc::c_int = countKReducibleNumbers(s.as_mut_ptr(), k);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
