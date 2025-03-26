use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn countOfSubstrings(
    mut word: *mut libc::c_char,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let VOWEL_MASK: libc::c_longlong = 1065233 as libc::c_int as libc::c_longlong;
    let mut ans: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut cnt_vowel1: [libc::c_int; 26] = [
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
    ];
    let mut cnt_vowel2: [libc::c_int; 26] = [
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
    ];
    let mut size_vowel1: libc::c_int = 0 as libc::c_int;
    let mut size_vowel2: libc::c_int = 0 as libc::c_int;
    let mut cnt_consonant1: libc::c_int = 0 as libc::c_int;
    let mut cnt_consonant2: libc::c_int = 0 as libc::c_int;
    let mut left1: libc::c_int = 0 as libc::c_int;
    let mut left2: libc::c_int = 0 as libc::c_int;
    let mut length: libc::c_int = strlen(word) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        let mut b: libc::c_int = *word.offset(i as isize) as libc::c_int - 'a' as i32;
        if VOWEL_MASK >> b & 1 as libc::c_int as libc::c_longlong != 0 {
            let fresh0 = cnt_vowel1[b as usize];
            cnt_vowel1[b as usize] = cnt_vowel1[b as usize] + 1;
            if fresh0 == 0 as libc::c_int {
                size_vowel1 += 1;
                size_vowel1;
            }
            let fresh1 = cnt_vowel2[b as usize];
            cnt_vowel2[b as usize] = cnt_vowel2[b as usize] + 1;
            if fresh1 == 0 as libc::c_int {
                size_vowel2 += 1;
                size_vowel2;
            }
        } else {
            cnt_consonant1 += 1;
            cnt_consonant1;
            cnt_consonant2 += 1;
            cnt_consonant2;
        }
        while size_vowel1 == 5 as libc::c_int && cnt_consonant1 >= k {
            let mut out: libc::c_int = *word.offset(left1 as isize) as libc::c_int
                - 'a' as i32;
            if VOWEL_MASK >> out & 1 as libc::c_int as libc::c_longlong != 0 {
                cnt_vowel1[out as usize] -= 1;
                if cnt_vowel1[out as usize] == 0 as libc::c_int {
                    size_vowel1 -= 1;
                    size_vowel1;
                }
            } else {
                cnt_consonant1 -= 1;
                cnt_consonant1;
            }
            left1 += 1;
            left1;
        }
        while size_vowel2 == 5 as libc::c_int && cnt_consonant2 > k {
            let mut out_0: libc::c_int = *word.offset(left2 as isize) as libc::c_int
                - 'a' as i32;
            if VOWEL_MASK >> out_0 & 1 as libc::c_int as libc::c_longlong != 0 {
                cnt_vowel2[out_0 as usize] -= 1;
                if cnt_vowel2[out_0 as usize] == 0 as libc::c_int {
                    size_vowel2 -= 1;
                    size_vowel2;
                }
            } else {
                cnt_consonant2 -= 1;
                cnt_consonant2;
            }
            left2 += 1;
            left2;
        }
        ans += (left1 - left2) as libc::c_longlong;
        i += 1;
        i;
    }
    return ans;
}
unsafe fn main_0() -> libc::c_int {
    let mut wordSize: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut wordSize as *mut libc::c_int,
    );
    let mut word: *mut libc::c_char = malloc(
        (wordSize + 1 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, word);
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, countOfSubstrings(word, k));
    free(word as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
