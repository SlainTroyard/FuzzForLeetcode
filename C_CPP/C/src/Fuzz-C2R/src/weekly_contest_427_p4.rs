use ::libc;
extern "C" {
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
pub struct PriorityQueue {
    pub arr: *mut libc::c_longlong,
    pub arrSize: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BinaryTree {
    pub arr: *mut libc::c_int,
    pub highestBit: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn maxRectangleArea(
    mut xCoord: *mut libc::c_int,
    mut xCoordSize: libc::c_int,
    mut yCoord: *mut libc::c_int,
    mut yCoordSize: libc::c_int,
) -> libc::c_longlong {
    let n: libc::c_int = xCoordSize;
    let treeSize: libc::c_int = n * 3 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut k: libc::c_int = -(2147483647 as libc::c_int) - 1 as libc::c_int;
    let mut x: libc::c_int = 0 as libc::c_int;
    let mut y: libc::c_int = 0 as libc::c_int;
    let mut number: libc::c_int = 0 as libc::c_int;
    let mut yMapSize: libc::c_int = 0 as libc::c_int;
    let mut buffSize: libc::c_int = 0 as libc::c_int;
    let vla = n as usize;
    let mut xMap: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = n as usize;
    let mut yMap: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let vla_1 = n as usize;
    let mut listsSize: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_1);
    let vla_2 = n as usize;
    let mut listsBuff: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_2);
    let vla_3 = n as usize;
    let mut prefixBuff: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_3);
    let vla_4 = n as usize;
    let mut xLast: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_4);
    let vla_5 = treeSize as usize;
    let mut arr1: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_5);
    let vla_6 = n as usize;
    let mut lists: Vec::<*mut libc::c_int> = ::std::vec::from_elem(
        0 as *mut libc::c_int,
        vla_6,
    );
    let vla_7 = n as usize;
    let mut prefix: Vec::<*mut libc::c_int> = ::std::vec::from_elem(
        0 as *mut libc::c_int,
        vla_7,
    );
    let vla_8 = n as usize;
    let mut arr2: Vec::<libc::c_longlong> = ::std::vec::from_elem(0, vla_8);
    let mut tree: BinaryTree = BinaryTree {
        arr: 0 as *mut libc::c_int,
        highestBit: 0,
    };
    let mut queue: PriorityQueue = PriorityQueue {
        arr: 0 as *mut libc::c_longlong,
        arrSize: 0,
    };
    let mut t: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut result: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    memset(
        xLast.as_mut_ptr() as *mut libc::c_void,
        -(1 as libc::c_int),
        (vla_4 * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    memset(
        arr1.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (vla_5 * ::core::mem::size_of::<libc::c_int>()) as libc::c_ulong,
    );
    queue.arr = arr2.as_mut_ptr();
    queue.arrSize = 0 as libc::c_int;
    tree.arr = arr1.as_mut_ptr();
    treeHighestBit(&mut tree, n - 1 as libc::c_int);
    j = 0 as libc::c_int;
    while n > j {
        queuePush(&mut queue, *yCoord.offset(j as isize) as libc::c_longlong);
        j += 1;
        j;
    }
    while (0 as libc::c_int) < queue.arrSize {
        if (k as libc::c_longlong) < *(queue.arr).offset(0 as libc::c_int as isize) {
            k = *(queue.arr).offset(0 as libc::c_int as isize) as libc::c_int;
            *yMap.as_mut_ptr().offset(yMapSize as isize) = k;
            yMapSize += 1;
            yMapSize;
        }
        queuePop(&mut queue);
    }
    j = 0 as libc::c_int;
    while n > j {
        y = binarySearch(yMap.as_mut_ptr(), yMapSize, *yCoord.offset(j as isize));
        queuePush(
            &mut queue,
            (*xCoord.offset(j as isize) as libc::c_longlong) << 32 as libc::c_int
                | y as libc::c_longlong,
        );
        j += 1;
        j;
    }
    while (0 as libc::c_int) < queue.arrSize {
        j = 0 as libc::c_int;
        let ref mut fresh0 = *lists.as_mut_ptr().offset(i as isize);
        *fresh0 = &mut *listsBuff.as_mut_ptr().offset(buffSize as isize)
            as *mut libc::c_int;
        let ref mut fresh1 = *prefix.as_mut_ptr().offset(i as isize);
        *fresh1 = &mut *prefixBuff.as_mut_ptr().offset(buffSize as isize)
            as *mut libc::c_int;
        *xMap
            .as_mut_ptr()
            .offset(
                i as isize,
            ) = (*(queue.arr).offset(0 as libc::c_int as isize) >> 32 as libc::c_int)
            as libc::c_int;
        while (0 as libc::c_int) < queue.arrSize
            && *xMap.as_mut_ptr().offset(i as isize) as libc::c_longlong
                == *(queue.arr).offset(0 as libc::c_int as isize) >> 32 as libc::c_int
        {
            *(*lists.as_mut_ptr().offset(i as isize))
                .offset(
                    j as isize,
                ) = (*(queue.arr).offset(0 as libc::c_int as isize)
                & 0xffffffff as libc::c_uint as libc::c_longlong) as libc::c_int;
            *(*prefix.as_mut_ptr().offset(i as isize))
                .offset(
                    j as isize,
                ) = treePushCount(
                &mut tree,
                *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize),
            );
            if (0 as libc::c_int) < j
                && -(1 as libc::c_int)
                    != *xLast
                        .as_mut_ptr()
                        .offset(
                            *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize)
                                as isize,
                        )
                && *xLast
                    .as_mut_ptr()
                    .offset(
                        *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize)
                            as isize,
                    ) == k
            {
                x = binarySearch(
                    *lists.as_mut_ptr().offset(k as isize),
                    *listsSize.as_mut_ptr().offset(k as isize),
                    *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize),
                );
                y = binarySearch(
                    *lists.as_mut_ptr().offset(k as isize),
                    *listsSize.as_mut_ptr().offset(k as isize),
                    *(*lists.as_mut_ptr().offset(i as isize))
                        .offset((j - 1 as libc::c_int) as isize),
                );
                number = *(*prefix.as_mut_ptr().offset(i as isize)).offset(j as isize)
                    - *(*prefix.as_mut_ptr().offset(i as isize))
                        .offset((j - 1 as libc::c_int) as isize)
                    - *(*prefix.as_mut_ptr().offset(k as isize)).offset(x as isize)
                    + *(*prefix.as_mut_ptr().offset(k as isize)).offset(y as isize);
                if x - 1 as libc::c_int == y && 1 as libc::c_int == number {
                    t = (*xMap.as_mut_ptr().offset(i as isize)
                        - *xMap.as_mut_ptr().offset(k as isize)) as libc::c_longlong
                        * (*yMap
                            .as_mut_ptr()
                            .offset(
                                *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize)
                                    as isize,
                            )
                            - *yMap
                                .as_mut_ptr()
                                .offset(
                                    *(*lists.as_mut_ptr().offset(i as isize))
                                        .offset((j - 1 as libc::c_int) as isize) as isize,
                                )) as libc::c_longlong;
                    result = if result > t { result } else { t };
                }
            }
            k = *xLast
                .as_mut_ptr()
                .offset(
                    *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize) as isize,
                );
            *xLast
                .as_mut_ptr()
                .offset(
                    *(*lists.as_mut_ptr().offset(i as isize)).offset(j as isize) as isize,
                ) = i;
            queuePop(&mut queue);
            j += 1;
            j;
        }
        *listsSize.as_mut_ptr().offset(i as isize) = j;
        buffSize += j;
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
unsafe extern "C" fn binarySearch(
    mut map: *mut libc::c_int,
    mut mapSize: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut mid: libc::c_int = -(1 as libc::c_int);
    let mut left: libc::c_int = 0 as libc::c_int;
    let mut right: libc::c_int = mapSize - 1 as libc::c_int;
    if value < *map.offset(left as isize) {
        return mid;
    }
    while left < right {
        mid = left + right + 1 as libc::c_int >> 1 as libc::c_int;
        if value < *map.offset(mid as isize) {
            right = mid - 1 as libc::c_int;
        } else {
            left = mid;
        }
    }
    return left;
}
unsafe extern "C" fn treeHighestBit(mut tree: *mut BinaryTree, mut max: libc::c_int) {
    let mut i: libc::c_int = 1 as libc::c_int;
    if 0 as libc::c_int != max {
        while 0 as libc::c_int != max {
            i += 1;
            i;
            max = max >> 1 as libc::c_int;
        }
        i = (1 as libc::c_int) << i - 2 as libc::c_int;
    }
    (*tree).highestBit = i;
}
unsafe extern "C" fn treePushCount(
    mut tree: *mut BinaryTree,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut bit: libc::c_int = (*tree).highestBit;
    let mut result: libc::c_int = 0 as libc::c_int;
    while 0 as libc::c_int != bit {
        if bit & value != 0 {
            result
                += *((*tree).arr)
                    .offset(((i << 1 as libc::c_int) + 1 as libc::c_int) as isize);
            i = (i << 1 as libc::c_int) + 2 as libc::c_int;
        } else {
            i = (i << 1 as libc::c_int) + 1 as libc::c_int;
        }
        let ref mut fresh2 = *((*tree).arr).offset(i as isize);
        *fresh2 += 1;
        *fresh2;
        bit = bit >> 1 as libc::c_int;
    }
    result += *((*tree).arr).offset(i as isize);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n as *mut libc::c_int);
    let vla = n as usize;
    let mut xCoord: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla);
    let vla_0 = n as usize;
    let mut yCoord: Vec::<libc::c_int> = ::std::vec::from_elem(0, vla_0);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *xCoord.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
            &mut *yCoord.as_mut_ptr().offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut maxArea: libc::c_longlong = maxRectangleArea(
        xCoord.as_mut_ptr(),
        n,
        yCoord.as_mut_ptr(),
        n,
    );
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, maxArea);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
