use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Matrix {
    pub m: [[libc::c_int; 26]; 26],
}
#[no_mangle]
pub unsafe extern "C" fn lengthAfterTransformations(
    mut s: *mut libc::c_char,
    mut t: libc::c_int,
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
) -> libc::c_int {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut digitsSize: libc::c_int = 0 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut src: [libc::c_int; 26] = [0; 26];
    let mut digits: [libc::c_int; 32] = [0; 32];
    let mut init: Matrix = Matrix { m: [[0; 26]; 26] };
    let mut dp: [Matrix; 2] = [Matrix { m: [[0; 26]; 26] }; 2];
    memset(
        src.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[libc::c_int; 26]>() as libc::c_ulong,
    );
    memset(
        &mut init as *mut Matrix as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<Matrix>() as libc::c_ulong,
    );
    memset(
        dp.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[Matrix; 2]>() as libc::c_ulong,
    );
    x = 0 as libc::c_int;
    while 26 as libc::c_int > x {
        dp[0 as libc::c_int as usize].m[x as usize][x as usize] = 1 as libc::c_int;
        y = 1 as libc::c_int;
        while *nums.offset(x as isize) >= y {
            z = if 26 as libc::c_int > x + y {
                x + y
            } else {
                x + y - 26 as libc::c_int
            };
            init.m[z as usize][x as usize] = 1 as libc::c_int;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    x = 0 as libc::c_int;
    while '\0' as i32 != *s.offset(x as isize) as libc::c_int {
        src[(*s.offset(x as isize) as libc::c_int - 'a' as i32) as usize] += 1;
        src[(*s.offset(x as isize) as libc::c_int - 'a' as i32) as usize];
        x += 1;
        x;
    }
    x = t;
    while 0 as libc::c_int != x {
        digits[digitsSize as usize] = x & 1 as libc::c_int;
        digitsSize += 1;
        digitsSize;
        x = x >> 1 as libc::c_int;
    }
    z = 0 as libc::c_int;
    x = digitsSize - 1 as libc::c_int;
    while 0 as libc::c_int <= x {
        matrixMultiply(
            &mut *dp.as_mut_ptr().offset(z as isize),
            &mut *dp.as_mut_ptr().offset(z as isize),
            &mut *dp.as_mut_ptr().offset((1 as libc::c_int - z) as isize),
        );
        if 1 as libc::c_int == digits[x as usize] {
            matrixMultiply(
                &mut *dp.as_mut_ptr().offset((1 as libc::c_int - z) as isize),
                &mut init,
                &mut *dp.as_mut_ptr().offset(z as isize),
            );
        } else {
            z = 1 as libc::c_int - z;
        }
        x -= 1;
        x;
    }
    x = 0 as libc::c_int;
    while 26 as libc::c_int > x {
        y = 0 as libc::c_int;
        while 26 as libc::c_int > y {
            result = ((dp[z as usize].m[x as usize][y as usize] as libc::c_longlong
                * src[y as usize] as libc::c_longlong + result as libc::c_longlong)
                % 1000000007 as libc::c_int as libc::c_longlong) as libc::c_int;
            y += 1;
            y;
        }
        x += 1;
        x;
    }
    return result;
}
unsafe extern "C" fn matrixMultiply(
    mut a: *mut Matrix,
    mut b: *mut Matrix,
    mut result: *mut Matrix,
) {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = 0 as libc::c_int;
    x = 0 as libc::c_int;
    while 26 as libc::c_int > x {
        y = 0 as libc::c_int;
        while 26 as libc::c_int > y {
            (*result).m[x as usize][y as usize] = 0 as libc::c_int;
            z = 0 as libc::c_int;
            while 26 as libc::c_int > z {
                (*result)
                    .m[x
                    as usize][y
                    as usize] = (((*a).m[x as usize][z as usize] as libc::c_longlong
                    * (*b).m[z as usize][y as usize] as libc::c_longlong
                    + (*result).m[x as usize][y as usize] as libc::c_longlong)
                    % 1000000007 as libc::c_int as libc::c_longlong) as libc::c_int;
                z += 1;
                z;
            }
            y += 1;
            y;
        }
        x += 1;
        x;
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut s_len: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut nums_size: libc::c_int = 0;
    nums_size = 26 as libc::c_int;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut s_len as *mut libc::c_int);
    let mut s: *mut libc::c_char = malloc(
        ((s_len + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s);
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut t as *mut libc::c_int);
    let mut nums: *mut libc::c_int = malloc(
        (nums_size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < nums_size {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = lengthAfterTransformations(s, t, nums, nums_size);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    free(s as *mut libc::c_void);
    free(nums as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
