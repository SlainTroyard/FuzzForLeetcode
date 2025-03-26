use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
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
pub struct DpNode {
    pub max: [libc::c_longlong; 4],
    pub idx: [libc::c_ulonglong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PriorityQueue {
    pub arr: *mut libc::c_longlong,
    pub arrSize: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn maximumWeight(
    mut intervals: *mut *mut libc::c_int,
    mut intervalsSize: libc::c_int,
    mut intervalsColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut edge: libc::c_int = 0 as libc::c_int;
    let mut prev: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut max: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut idx: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut sel: libc::c_ulonglong = 0xffffffffffffffff as libc::c_ulong
        as libc::c_ulonglong;
    let vla = intervalsSize as usize;
    let mut arr1: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla);
    let vla_0 = intervalsSize as usize;
    let mut arr2: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla_0);
    let mut last: DpNode = DpNode { max: [0; 4], idx: [0; 4] };
    let vla_1 = intervalsSize as usize;
    let mut dp: Vec::<DpNode> = ::std::vec::from_elem(
        DpNode { max: [0; 4], idx: [0; 4] },
        vla_1,
    );
    let mut leftQueue: PriorityQueue = PriorityQueue {
        arr: 0 as *mut libc::c_longlong,
        arrSize: 0,
    };
    let mut rightQueue: PriorityQueue = PriorityQueue {
        arr: 0 as *mut libc::c_longlong,
        arrSize: 0,
    };
    let mut result: *mut libc::c_int = 0 as *mut libc::c_int;
    memset(
        &mut last as *mut DpNode as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<DpNode>() as libc::c_ulong,
    );
    leftQueue.arr = arr1.as_mut_ptr();
    leftQueue.arrSize = 0 as libc::c_int;
    rightQueue.arr = arr2.as_mut_ptr();
    rightQueue.arrSize = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while intervalsSize > i {
        queuePush(
            &mut leftQueue,
            (*(*intervals.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_longlong) << 32 as libc::c_int | i as libc::c_longlong,
        );
        queuePush(
            &mut rightQueue,
            (*(*intervals.offset(i as isize)).offset(1 as libc::c_int as isize)
                as libc::c_longlong) << 32 as libc::c_int | i as libc::c_longlong,
        );
        i += 1;
        i;
    }
    while (0 as libc::c_int) < leftQueue.arrSize {
        i = (*(leftQueue.arr).offset(0 as libc::c_int as isize)
            & 0xffffffff as libc::c_uint as libc::c_longlong) as libc::c_int;
        edge = (*(leftQueue.arr).offset(0 as libc::c_int as isize) >> 32 as libc::c_int)
            as libc::c_int;
        while (0 as libc::c_int) < rightQueue.arrSize
            && edge as libc::c_longlong
                > *(rightQueue.arr).offset(0 as libc::c_int as isize)
                    >> 32 as libc::c_int
        {
            j = (*(rightQueue.arr).offset(0 as libc::c_int as isize)
                & 0xffffffff as libc::c_uint as libc::c_longlong) as libc::c_int;
            k = 0 as libc::c_int;
            while 3 as libc::c_int > k {
                if last.max[k as usize]
                    < (*dp.as_mut_ptr().offset(j as isize)).max[k as usize]
                    || last.max[k as usize]
                        == (*dp.as_mut_ptr().offset(j as isize)).max[k as usize]
                        && last.idx[k as usize]
                            > (*dp.as_mut_ptr().offset(j as isize)).idx[k as usize]
                {
                    last
                        .max[k
                        as usize] = (*dp.as_mut_ptr().offset(j as isize))
                        .max[k as usize];
                    last
                        .idx[k
                        as usize] = (*dp.as_mut_ptr().offset(j as isize))
                        .idx[k as usize];
                }
                k += 1;
                k;
            }
            queuePop(&mut rightQueue);
        }
        memcpy(
            &mut *dp.as_mut_ptr().offset(i as isize) as *mut DpNode as *mut libc::c_void,
            &mut last as *mut DpNode as *const libc::c_void,
            ::core::mem::size_of::<DpNode>() as libc::c_ulong,
        );
        k = 0 as libc::c_int;
        while 4 as libc::c_int > k {
            prev = if 0 as libc::c_int == k {
                0 as libc::c_int as libc::c_longlong
            } else {
                last.max[(k - 1 as libc::c_int) as usize]
            };
            if 0 as libc::c_int == k
                || (0 as libc::c_int) < k
                    && (0 as libc::c_int as libc::c_longlong) < prev
            {
                idx = if 0 as libc::c_int == k {
                    (i as libc::c_ulonglong) << 48 as libc::c_int
                } else {
                    insertIndex(last.idx[(k - 1 as libc::c_int) as usize], i, k)
                };
                if (*dp.as_mut_ptr().offset(i as isize)).max[k as usize]
                    < *(*intervals.offset(i as isize)).offset(2 as libc::c_int as isize)
                        as libc::c_longlong + prev
                    || (*dp.as_mut_ptr().offset(i as isize)).max[k as usize]
                        == *(*intervals.offset(i as isize))
                            .offset(2 as libc::c_int as isize) as libc::c_longlong + prev
                        && (*dp.as_mut_ptr().offset(i as isize)).idx[k as usize] > idx
                {
                    (*dp.as_mut_ptr().offset(i as isize))
                        .max[k
                        as usize] = *(*intervals.offset(i as isize))
                        .offset(2 as libc::c_int as isize) as libc::c_longlong + prev;
                    (*dp.as_mut_ptr().offset(i as isize)).idx[k as usize] = idx;
                }
                if max < (*dp.as_mut_ptr().offset(i as isize)).max[k as usize]
                    || max == (*dp.as_mut_ptr().offset(i as isize)).max[k as usize]
                        && sel > (*dp.as_mut_ptr().offset(i as isize)).idx[k as usize]
                {
                    *returnSize = k + 1 as libc::c_int;
                    max = (*dp.as_mut_ptr().offset(i as isize)).max[k as usize];
                    sel = (*dp.as_mut_ptr().offset(i as isize)).idx[k as usize];
                }
            }
            k += 1;
            k;
        }
        queuePop(&mut leftQueue);
    }
    result = malloc(
        (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(*returnSize as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while *returnSize > i {
        *result
            .offset(
                i as isize,
            ) = (sel >> (3 as libc::c_int - i << 4 as libc::c_int)
            & 0xffff as libc::c_int as libc::c_ulonglong) as libc::c_int;
        i += 1;
        i;
    }
    return result;
}
unsafe extern "C" fn queuePush(
    mut queue: *mut PriorityQueue,
    mut value: libc::c_longlong,
) {
    let mut son: libc::c_int = (*queue).arrSize;
    let mut father: libc::c_int = if 0 as libc::c_int == son {
        -(1 as libc::c_int)
    } else {
        son - 1 as libc::c_int >> 1 as libc::c_int
    };
    (*queue).arrSize += 1;
    (*queue).arrSize;
    while -(1 as libc::c_int) != father
        && value < *((*queue).arr).offset(father as isize)
    {
        *((*queue).arr).offset(son as isize) = *((*queue).arr).offset(father as isize);
        son = father;
        father = if 0 as libc::c_int == son {
            -(1 as libc::c_int)
        } else {
            son - 1 as libc::c_int >> 1 as libc::c_int
        };
    }
    *((*queue).arr).offset(son as isize) = value;
}
unsafe extern "C" fn queuePop(mut queue: *mut PriorityQueue) {
    let mut father: libc::c_int = 0 as libc::c_int;
    let mut left: libc::c_int = (father << 1 as libc::c_int) + 1 as libc::c_int;
    let mut right: libc::c_int = (father << 1 as libc::c_int) + 2 as libc::c_int;
    let mut son: libc::c_int = 0 as libc::c_int;
    (*queue).arrSize -= 1;
    (*queue).arrSize;
    while (*queue).arrSize > left
        && *((*queue).arr).offset((*queue).arrSize as isize)
            > *((*queue).arr).offset(left as isize)
        || (*queue).arrSize > right
            && *((*queue).arr).offset((*queue).arrSize as isize)
                > *((*queue).arr).offset(right as isize)
    {
        son = if (*queue).arrSize > right
            && *((*queue).arr).offset(left as isize)
                > *((*queue).arr).offset(right as isize)
        {
            right
        } else {
            left
        };
        *((*queue).arr).offset(father as isize) = *((*queue).arr).offset(son as isize);
        father = son;
        left = (father << 1 as libc::c_int) + 1 as libc::c_int;
        right = (father << 1 as libc::c_int) + 2 as libc::c_int;
    }
    *((*queue).arr)
        .offset(father as isize) = *((*queue).arr).offset((*queue).arrSize as isize);
}
unsafe extern "C" fn insertIndex(
    mut idx: libc::c_ulonglong,
    mut i: libc::c_int,
    mut count: libc::c_int,
) -> libc::c_ulonglong {
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut value: [libc::c_int; 4] = [0; 4];
    let mut result: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    value[0 as libc::c_int as usize] = (idx >> 48 as libc::c_int) as libc::c_int;
    value[1 as libc::c_int
        as usize] = (idx >> 32 as libc::c_int
        & 0xffff as libc::c_int as libc::c_ulonglong) as libc::c_int;
    value[2 as libc::c_int
        as usize] = (idx >> 16 as libc::c_int
        & 0xffff as libc::c_int as libc::c_ulonglong) as libc::c_int;
    value[count as usize] = i;
    x = count - 1 as libc::c_int;
    while -(1 as libc::c_int) < x && value[x as usize] > i {
        value[(x + 1 as libc::c_int) as usize] = value[x as usize];
        value[x as usize] = i;
        x -= 1;
        x;
    }
    result = (value[0 as libc::c_int as usize] as libc::c_ulonglong) << 48 as libc::c_int
        | (value[1 as libc::c_int as usize] as libc::c_ulonglong) << 32 as libc::c_int
        | (value[2 as libc::c_int as usize] as libc::c_ulonglong) << 16 as libc::c_int
        | value[3 as libc::c_int as usize] as libc::c_ulonglong;
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let mut vec: *mut *mut libc::c_int = malloc(
        (::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(n as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh0 = *vec.offset(i as isize);
        *fresh0 = malloc(
            (::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(3 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_int;
        scanf(
            b"%d %d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*vec.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*vec.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*vec.offset(i as isize)).offset(2 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_int = maximumWeight(
        vec,
        n,
        0 as *mut libc::c_int,
        &mut returnSize,
    );
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *result.offset(i_0 as isize),
        );
        i_0 += 1;
        i_0;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    free(result as *mut libc::c_void);
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n {
        free(*vec.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(vec as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
