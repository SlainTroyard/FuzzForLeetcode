use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn merge(
    mut intervals: *mut [libc::c_int; 2],
    mut size: libc::c_int,
    mut len: libc::c_int,
    mut merged: *mut [libc::c_int; 2],
) -> libc::c_int {
    if size == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut mergedSize: libc::c_int = 0 as libc::c_int;
    let mut currentStart: libc::c_int = (*intervals
        .offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    let mut currentEnd: libc::c_int = (*intervals
        .offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        let mut start: libc::c_int = (*intervals
            .offset(i as isize))[0 as libc::c_int as usize];
        let mut end: libc::c_int = (*intervals
            .offset(i as isize))[1 as libc::c_int as usize];
        if start <= currentEnd && start - currentStart + 1 as libc::c_int <= len {
            currentEnd = if currentEnd > end { currentEnd } else { end };
        } else {
            (*merged
                .offset(mergedSize as isize))[0 as libc::c_int as usize] = currentStart;
            (*merged
                .offset(mergedSize as isize))[1 as libc::c_int as usize] = currentEnd;
            mergedSize += 1;
            mergedSize;
            currentStart = start;
            currentEnd = end;
        }
        i += 1;
        i;
    }
    (*merged.offset(mergedSize as isize))[0 as libc::c_int as usize] = currentStart;
    (*merged.offset(mergedSize as isize))[1 as libc::c_int as usize] = currentEnd;
    return mergedSize + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isPoss(
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut op: libc::c_int,
    mut mid: libc::c_int,
) -> bool {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut zero: libc::c_int = 0 as libc::c_int;
    let mut one: libc::c_int = 0 as libc::c_int;
    let mut intervals: [[libc::c_int; 2]; 100000] = [[0; 2]; 100000];
    let mut size: libc::c_int = 0 as libc::c_int;
    while j < n {
        if *s.offset(j as isize) as libc::c_int == '0' as i32 {
            zero += 1;
            zero;
        } else {
            one += 1;
            one;
        }
        while j - i + 1 as libc::c_int > mid {
            if *s.offset(i as isize) as libc::c_int == '0' as i32 {
                zero -= 1;
                zero;
            } else {
                one -= 1;
                one;
            }
            i += 1;
            i;
        }
        if j - i + 1 as libc::c_int == mid {
            if zero == 0 as libc::c_int || one == 0 as libc::c_int {
                intervals[size as usize][0 as libc::c_int as usize] = i;
                intervals[size as usize][1 as libc::c_int as usize] = j;
                size += 1;
                size;
            }
        }
        j += 1;
        j;
    }
    let mut merged: [[libc::c_int; 2]; 100000] = [[0; 2]; 100000];
    let mut mergedSize: libc::c_int = merge(
        intervals.as_mut_ptr(),
        size,
        mid,
        merged.as_mut_ptr(),
    );
    return mergedSize <= op;
}
#[no_mangle]
pub unsafe extern "C" fn getMini(
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
    mut even: libc::c_char,
    mut odd: libc::c_char,
) -> libc::c_int {
    let mut op: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        if i % 2 as libc::c_int == 0 as libc::c_int
            && *s.offset(i as isize) as libc::c_int != even as libc::c_int
        {
            op += 1;
            op;
        } else if i % 2 as libc::c_int == 1 as libc::c_int
            && *s.offset(i as isize) as libc::c_int != odd as libc::c_int
        {
            op += 1;
            op;
        }
        i += 1;
        i;
    }
    return op;
}
#[no_mangle]
pub unsafe extern "C" fn minLength(
    mut s: *mut libc::c_char,
    mut numOps: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_int = strlen(s) as libc::c_int;
    let mut start: libc::c_int = 3 as libc::c_int;
    let mut end: libc::c_int = n;
    let mut res: libc::c_int = 2 as libc::c_int;
    let mut op: libc::c_int = 2147483647 as libc::c_int;
    let mut opEvenOdd: libc::c_int = getMini(
        s,
        n,
        '0' as i32 as libc::c_char,
        '1' as i32 as libc::c_char,
    );
    let mut opOddEven: libc::c_int = getMini(
        s,
        n,
        '1' as i32 as libc::c_char,
        '0' as i32 as libc::c_char,
    );
    op = if opEvenOdd < op { opEvenOdd } else { op };
    op = if opOddEven < op { opOddEven } else { op };
    if op <= numOps {
        return 1 as libc::c_int;
    }
    while start <= end {
        let mut mid: libc::c_int = start + (end - start) / 2 as libc::c_int;
        if isPoss(s, n, numOps, mid) {
            end = mid - 1 as libc::c_int;
        } else {
            res = mid;
            start = mid + 1 as libc::c_int;
        }
    }
    return res;
}
unsafe fn main_0() -> libc::c_int {
    let mut s: [libc::c_char; 100001] = [0; 100001];
    let mut numOps: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s.as_mut_ptr());
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut numOps as *mut libc::c_int);
    let mut result: libc::c_int = minLength(s.as_mut_ptr(), numOps);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
