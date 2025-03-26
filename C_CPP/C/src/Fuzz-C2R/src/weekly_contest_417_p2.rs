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
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut len: libc::c_int = strlen(word) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= len - 5 as libc::c_int {
        let mut arr: [libc::c_int; 5] = [0 as libc::c_int, 0, 0, 0, 0];
        let mut nonVowelCount: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = i;
        while j < len {
            if *word.offset(j as isize) as libc::c_int == 'a' as i32 {
                arr[0 as libc::c_int as usize] += 1;
                arr[0 as libc::c_int as usize];
            } else if *word.offset(j as isize) as libc::c_int == 'e' as i32 {
                arr[1 as libc::c_int as usize] += 1;
                arr[1 as libc::c_int as usize];
            } else if *word.offset(j as isize) as libc::c_int == 'i' as i32 {
                arr[2 as libc::c_int as usize] += 1;
                arr[2 as libc::c_int as usize];
            } else if *word.offset(j as isize) as libc::c_int == 'o' as i32 {
                arr[3 as libc::c_int as usize] += 1;
                arr[3 as libc::c_int as usize];
            } else if *word.offset(j as isize) as libc::c_int == 'u' as i32 {
                arr[4 as libc::c_int as usize] += 1;
                arr[4 as libc::c_int as usize];
            } else {
                nonVowelCount += 1;
                nonVowelCount;
            }
            if arr[0 as libc::c_int as usize] > 0 as libc::c_int
                && arr[1 as libc::c_int as usize] > 0 as libc::c_int
                && arr[2 as libc::c_int as usize] > 0 as libc::c_int
                && arr[3 as libc::c_int as usize] > 0 as libc::c_int
                && arr[4 as libc::c_int as usize] > 0 as libc::c_int
                && nonVowelCount == k
            {
                count += 1;
                count;
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    return count;
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
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, countOfSubstrings(word, k));
    free(word as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
