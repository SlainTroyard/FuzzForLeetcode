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
pub struct SubNode {
    pub index: libc::c_int,
    pub next: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SectionNode {
    pub start: libc::c_int,
    pub end: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DfsNode {
    pub srcStr: *mut libc::c_char,
    pub dfsStr: *mut libc::c_char,
    pub strIndex: libc::c_int,
    pub hp: *mut libc::c_int,
    pub list: *mut SubNode,
    pub sec: *mut SectionNode,
}
#[no_mangle]
pub unsafe extern "C" fn findAnswer(
    mut parent: *mut libc::c_int,
    mut parentSize: libc::c_int,
    mut s: *mut libc::c_char,
    mut returnSize: *mut libc::c_int,
) -> *mut bool {
    let module: libc::c_int = 1000000007 as libc::c_int;
    let radix: libc::c_int = 26 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut z: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut node: DfsNode = DfsNode {
        srcStr: 0 as *mut libc::c_char,
        dfsStr: 0 as *mut libc::c_char,
        strIndex: 0,
        hp: 0 as *mut libc::c_int,
        list: 0 as *mut SubNode,
        sec: 0 as *mut SectionNode,
    };
    let vla = parentSize as usize;
    let mut dfsStr: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
    let vla_0 = parentSize as usize;
    let mut hp: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = parentSize as usize;
    let mut forward: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = parentSize as usize;
    let mut backward: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_2);
    let vla_3 = parentSize as usize;
    let mut list: Vec::<SubNode> = ::std::vec::from_elem(
        SubNode { index: 0, next: 0 },
        vla_3,
    );
    let vla_4 = parentSize as usize;
    let mut sec: Vec::<SectionNode> = ::std::vec::from_elem(
        SectionNode { start: 0, end: 0 },
        vla_4,
    );
    let mut result: *mut bool = 0 as *mut bool;
    memset(
        hp.as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        (vla_0 * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    result = malloc(
        (::core::mem::size_of::<bool>() as libc::c_ulong)
            .wrapping_mul(parentSize as libc::c_ulong),
    ) as *mut bool;
    if result.is_null() {
        *returnSize = 0 as libc::c_int;
        return result;
    }
    x = parentSize - 1 as libc::c_int;
    while (0 as libc::c_int) < x {
        (*list.as_mut_ptr().offset(y as isize)).index = x;
        (*list.as_mut_ptr().offset(y as isize))
            .next = *hp.as_mut_ptr().offset(*parent.offset(x as isize) as isize);
        *hp.as_mut_ptr().offset(*parent.offset(x as isize) as isize) = y;
        y += 1;
        y;
        x -= 1;
        x;
    }
    node.srcStr = s;
    node.dfsStr = dfsStr.as_mut_ptr();
    node.strIndex = 0 as libc::c_int;
    node.hp = hp.as_mut_ptr();
    node.list = list.as_mut_ptr();
    node.sec = sec.as_mut_ptr();
    dfsCalc(&mut node, 0 as libc::c_int);
    x = 0 as libc::c_int;
    while parentSize > x {
        y = parentSize - 1 as libc::c_int - x;
        *hp
            .as_mut_ptr()
            .offset(
                x as isize,
            ) = (if 0 as libc::c_int == x {
            1 as libc::c_int as libc::c_longlong
        } else {
            *hp.as_mut_ptr().offset((x - 1 as libc::c_int) as isize) as libc::c_longlong
                * radix as libc::c_longlong % module as libc::c_longlong
        }) as libc::c_int;
        *forward
            .as_mut_ptr()
            .offset(
                x as isize,
            ) = (if 0 as libc::c_int == x {
            (*dfsStr.as_mut_ptr().offset(x as isize) as libc::c_int - 'a' as i32)
                as libc::c_longlong
        } else {
            (*forward.as_mut_ptr().offset((x - 1 as libc::c_int) as isize)
                as libc::c_longlong * radix as libc::c_longlong
                + *dfsStr.as_mut_ptr().offset(x as isize) as libc::c_longlong
                - 'a' as i32 as libc::c_longlong) % module as libc::c_longlong
        }) as libc::c_int;
        *backward
            .as_mut_ptr()
            .offset(
                x as isize,
            ) = (if 0 as libc::c_int == x {
            (*dfsStr.as_mut_ptr().offset(y as isize) as libc::c_int - 'a' as i32)
                as libc::c_longlong
        } else {
            (*backward.as_mut_ptr().offset((x - 1 as libc::c_int) as isize)
                as libc::c_longlong * radix as libc::c_longlong
                + *dfsStr.as_mut_ptr().offset(y as isize) as libc::c_longlong
                - 'a' as i32 as libc::c_longlong) % module as libc::c_longlong
        }) as libc::c_int;
        x += 1;
        x;
    }
    x = 0 as libc::c_int;
    while parentSize > x {
        k = (*sec.as_mut_ptr().offset(x as isize)).end
            - (*sec.as_mut_ptr().offset(x as isize)).start + 1 as libc::c_int;
        i = (*sec.as_mut_ptr().offset(x as isize)).start;
        j = (*sec.as_mut_ptr().offset(x as isize)).end;
        y = (if 0 as libc::c_int == i {
            *forward.as_mut_ptr().offset(j as isize) as libc::c_longlong
        } else {
            ((*forward.as_mut_ptr().offset(j as isize) as libc::c_longlong
                - *forward.as_mut_ptr().offset((i - 1 as libc::c_int) as isize)
                    as libc::c_longlong
                    * *hp.as_mut_ptr().offset(k as isize) as libc::c_longlong)
                % module as libc::c_longlong + module as libc::c_longlong)
                % module as libc::c_longlong
        }) as libc::c_int;
        i = parentSize - 1 as libc::c_int - (*sec.as_mut_ptr().offset(x as isize)).end;
        j = parentSize - 1 as libc::c_int - (*sec.as_mut_ptr().offset(x as isize)).start;
        z = (if 0 as libc::c_int == i {
            *backward.as_mut_ptr().offset(j as isize) as libc::c_longlong
        } else {
            ((*backward.as_mut_ptr().offset(j as isize) as libc::c_longlong
                - *backward.as_mut_ptr().offset((i - 1 as libc::c_int) as isize)
                    as libc::c_longlong
                    * *hp.as_mut_ptr().offset(k as isize) as libc::c_longlong)
                % module as libc::c_longlong + module as libc::c_longlong)
                % module as libc::c_longlong
        }) as libc::c_int;
        *result
            .offset(
                x as isize,
            ) = if y == z { 1 as libc::c_int } else { 0 as libc::c_int } != 0;
        x += 1;
        x;
    }
    *returnSize = parentSize;
    return result;
}
unsafe extern "C" fn dfsCalc(mut node: *mut DfsNode, mut root: libc::c_int) {
    let mut x: libc::c_int = 0 as libc::c_int;
    (*((*node).sec).offset(root as isize)).start = (*node).strIndex;
    x = *((*node).hp).offset(root as isize);
    while -(1 as libc::c_int) != x {
        dfsCalc(node, (*((*node).list).offset(x as isize)).index);
        x = (*((*node).list).offset(x as isize)).next;
    }
    *((*node).dfsStr)
        .offset((*node).strIndex as isize) = *((*node).srcStr).offset(root as isize);
    (*((*node).sec).offset(root as isize)).end = (*node).strIndex;
    (*node).strIndex += 1;
    (*node).strIndex;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut parent: *mut libc::c_int = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *parent.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut s: *mut libc::c_char = malloc((n + 1 as libc::c_int) as libc::c_ulong)
        as *mut libc::c_char;
    scanf(b"%s\0" as *const u8 as *const libc::c_char, s);
    let mut returnSize: libc::c_int = 0 as libc::c_int;
    let mut ans: *mut bool = findAnswer(parent, n, s, &mut returnSize);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        if *ans.offset(i_0 as isize) {
            printf(b"true \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"false \0" as *const u8 as *const libc::c_char);
        }
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(parent as *mut libc::c_void);
    free(s as *mut libc::c_void);
    free(ans as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
