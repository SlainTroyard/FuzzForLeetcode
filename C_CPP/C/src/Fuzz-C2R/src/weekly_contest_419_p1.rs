use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Element {
    pub value: libc::c_int,
    pub count: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn cmp(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut ea: *mut Element = a as *mut Element;
    let mut eb: *mut Element = b as *mut Element;
    if (*ea).count == (*eb).count {
        return (*eb).value - (*ea).value;
    }
    return (*eb).count - (*ea).count;
}
#[no_mangle]
pub unsafe extern "C" fn findXSum(
    mut nums: *mut libc::c_int,
    mut numsSize: libc::c_int,
    mut k: libc::c_int,
    mut x: libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut numResults: libc::c_int = numsSize - k + 1 as libc::c_int;
    let mut answer: *mut libc::c_int = malloc(
        (numResults as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    *returnSize = numResults;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numResults {
        let mut hash: [libc::c_int; 51] = [
            0 as libc::c_int,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < k {
            hash[*nums.offset((i + j) as isize) as usize] += 1;
            hash[*nums.offset((i + j) as isize) as usize];
            j += 1;
            j;
        }
        let mut elements: [Element; 51] = [Element { value: 0, count: 0 }; 51];
        let mut elementCount: libc::c_int = 0 as libc::c_int;
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < 51 as libc::c_int {
            if hash[j_0 as usize] > 0 as libc::c_int {
                elements[elementCount as usize].value = j_0;
                elements[elementCount as usize].count = hash[j_0 as usize];
                elementCount += 1;
                elementCount;
            }
            j_0 += 1;
            j_0;
        }
        qsort(
            elements.as_mut_ptr() as *mut libc::c_void,
            elementCount as size_t,
            ::core::mem::size_of::<Element>() as libc::c_ulong,
            Some(
                cmp
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        let mut sum: libc::c_int = 0 as libc::c_int;
        let mut elementsToSum: libc::c_int = if elementCount < x {
            elementCount
        } else {
            x
        };
        let mut j_1: libc::c_int = 0 as libc::c_int;
        while j_1 < elementsToSum {
            sum += elements[j_1 as usize].value * elements[j_1 as usize].count;
            j_1 += 1;
            j_1;
        }
        *answer.offset(i as isize) = sum;
        i += 1;
        i;
    }
    return answer;
}
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut k as *mut libc::c_int,
        &mut x as *mut libc::c_int,
    );
    let mut numsSize: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numsSize as *mut libc::c_int,
    );
    let mut nums: *mut libc::c_int = malloc(
        (numsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numsSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *nums.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut answer: *mut libc::c_int = findXSum(nums, numsSize, k, x, &mut returnSize);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *answer.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(nums as *mut libc::c_void);
    free(answer as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
