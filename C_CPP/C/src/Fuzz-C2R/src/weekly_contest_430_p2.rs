use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn answerString(
    mut word: *mut libc::c_char,
    mut numFriends: libc::c_int,
) -> *mut libc::c_char {
    if numFriends == 1 as libc::c_int {
        return word;
    }
    let mut len: libc::c_int = strlen(word) as libc::c_int;
    let mut n: libc::c_int = len - numFriends + 1 as libc::c_int;
    let mut ans: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < len {
        if strncmp(
            word.offset(i as isize),
            word.offset(ans as isize),
            n as libc::c_ulong,
        ) > 0 as libc::c_int
        {
            ans = i;
        }
        i += 1;
        i;
    }
    if ans + n < len {
        *word.offset(ans as isize).offset(n as isize) = '\0' as i32 as libc::c_char;
    }
    return word.offset(ans as isize);
}
unsafe fn main_0() -> libc::c_int {
    let mut word: [libc::c_char; 5001] = [0; 5001];
    let mut numFriends: libc::c_int = 0;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, word.as_mut_ptr());
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut numFriends as *mut libc::c_int,
    );
    let mut result: *mut libc::c_char = answerString(word.as_mut_ptr(), numFriends);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, result);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
