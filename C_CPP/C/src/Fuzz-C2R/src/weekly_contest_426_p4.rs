use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn buildAdjacencyList(
    mut edges: *mut *mut libc::c_int,
    mut edgesSize: libc::c_int,
    mut edgesColSize: *mut libc::c_int,
    mut n: libc::c_int,
    mut adjList: *mut *mut *mut libc::c_int,
    mut adjLen: *mut *mut libc::c_int,
) {
    *adjList = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    *adjLen = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edgesSize {
        let ref mut fresh0 = *(*adjLen)
            .offset(
                *(*edges.offset(i as isize)).offset(0 as libc::c_int as isize) as isize,
            );
        *fresh0 += 1;
        *fresh0;
        let ref mut fresh1 = *(*adjLen)
            .offset(
                *(*edges.offset(i as isize)).offset(1 as libc::c_int as isize) as isize,
            );
        *fresh1 += 1;
        *fresh1;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n {
        let ref mut fresh2 = *(*adjList).offset(i_0 as isize);
        *fresh2 = malloc(
            (*(*adjLen).offset(i_0 as isize) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *(*adjLen).offset(i_0 as isize) = 0 as libc::c_int;
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < edgesSize {
        let mut u: libc::c_int = *(*edges.offset(i_1 as isize))
            .offset(0 as libc::c_int as isize);
        let mut v: libc::c_int = *(*edges.offset(i_1 as isize))
            .offset(1 as libc::c_int as isize);
        let ref mut fresh3 = *(*adjLen).offset(u as isize);
        let fresh4 = *fresh3;
        *fresh3 = *fresh3 + 1;
        *(*(*adjList).offset(u as isize)).offset(fresh4 as isize) = v;
        let ref mut fresh5 = *(*adjLen).offset(v as isize);
        let fresh6 = *fresh5;
        *fresh5 = *fresh5 + 1;
        *(*(*adjList).offset(v as isize)).offset(fresh6 as isize) = u;
        i_1 += 1;
        i_1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn bfsCount(
    mut adjList: *mut *mut libc::c_int,
    mut adjLen: *mut libc::c_int,
    mut n: libc::c_int,
    mut colorCount: *mut libc::c_int,
    mut nodeColor: *mut libc::c_int,
) {
    let mut visited: *mut bool = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    ) as *mut bool;
    let mut queue: *mut libc::c_int = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut front: libc::c_int = 0 as libc::c_int;
    let mut rear: libc::c_int = 0 as libc::c_int;
    let fresh7 = rear;
    rear = rear + 1;
    *queue.offset(fresh7 as isize) = 0 as libc::c_int;
    *nodeColor.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    *visited.offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
    while front < rear {
        let fresh8 = front;
        front = front + 1;
        let mut curr: libc::c_int = *queue.offset(fresh8 as isize);
        let mut color: libc::c_int = *nodeColor.offset(curr as isize);
        let ref mut fresh9 = *colorCount.offset(color as isize);
        *fresh9 += 1;
        *fresh9;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < *adjLen.offset(curr as isize) {
            let mut neighbor: libc::c_int = *(*adjList.offset(curr as isize))
                .offset(i as isize);
            if !*visited.offset(neighbor as isize) {
                *visited.offset(neighbor as isize) = 1 as libc::c_int != 0;
                *nodeColor.offset(neighbor as isize) = 1 as libc::c_int - color;
                let fresh10 = rear;
                rear = rear + 1;
                *queue.offset(fresh10 as isize) = neighbor;
            }
            i += 1;
            i;
        }
    }
    free(visited as *mut libc::c_void);
    free(queue as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn maxTargetNodes(
    mut edges1: *mut *mut libc::c_int,
    mut edges1Size: libc::c_int,
    mut edges1ColSize: *mut libc::c_int,
    mut edges2: *mut *mut libc::c_int,
    mut edges2Size: libc::c_int,
    mut edges2ColSize: *mut libc::c_int,
    mut returnSize: *mut libc::c_int,
) -> *mut libc::c_int {
    let mut n1: libc::c_int = edges1Size + 1 as libc::c_int;
    let mut n2: libc::c_int = edges2Size + 1 as libc::c_int;
    let mut adjList1: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut adjList2: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut adjLen1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut adjLen2: *mut libc::c_int = 0 as *mut libc::c_int;
    buildAdjacencyList(
        edges1,
        edges1Size,
        edges1ColSize,
        n1,
        &mut adjList1,
        &mut adjLen1,
    );
    buildAdjacencyList(
        edges2,
        edges2Size,
        edges2ColSize,
        n2,
        &mut adjList2,
        &mut adjLen2,
    );
    let mut colorCount1: [libc::c_int; 2] = [0 as libc::c_int, 0];
    let mut colorCount2: [libc::c_int; 2] = [0 as libc::c_int, 0];
    let mut nodeColor1: *mut libc::c_int = calloc(
        n1 as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    let mut nodeColor2: *mut libc::c_int = calloc(
        n2 as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    bfsCount(adjList1, adjLen1, n1, colorCount1.as_mut_ptr(), nodeColor1);
    bfsCount(adjList2, adjLen2, n2, colorCount2.as_mut_ptr(), nodeColor2);
    let mut result: *mut libc::c_int = malloc(
        (n1 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut maxColorInTree2: libc::c_int = if colorCount2[0 as libc::c_int as usize]
        > colorCount2[1 as libc::c_int as usize]
    {
        colorCount2[0 as libc::c_int as usize]
    } else {
        colorCount2[1 as libc::c_int as usize]
    };
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n1 {
        *result
            .offset(
                i as isize,
            ) = colorCount1[*nodeColor1.offset(i as isize) as usize] + maxColorInTree2;
        i += 1;
        i;
    }
    *returnSize = n1;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n1 {
        free(*adjList1.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < n2 {
        free(*adjList2.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(adjList1 as *mut libc::c_void);
    free(adjList2 as *mut libc::c_void);
    free(adjLen1 as *mut libc::c_void);
    free(adjLen2 as *mut libc::c_void);
    free(nodeColor1 as *mut libc::c_void);
    free(nodeColor2 as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut n1: libc::c_int = 0;
    let mut n2: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n1 as *mut libc::c_int);
    let mut edges1: *mut *mut libc::c_int = malloc(
        (n1 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut edges1ColSize: *mut libc::c_int = malloc(
        (n1 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n1 {
        let ref mut fresh11 = *edges1.offset(i as isize);
        *fresh11 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *edges1ColSize.offset(i as isize) = 2 as libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges1.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*edges1.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut n2 as *mut libc::c_int);
    let mut edges2: *mut *mut libc::c_int = malloc(
        (n2 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut edges2ColSize: *mut libc::c_int = malloc(
        (n2 as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < n2 {
        let ref mut fresh12 = *edges2.offset(i_0 as isize);
        *fresh12 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *edges2ColSize.offset(i_0 as isize) = 2 as libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges2.offset(i_0 as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*edges2.offset(i_0 as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    let mut returnSize: libc::c_int = 0;
    let mut result: *mut libc::c_int = maxTargetNodes(
        edges1,
        n1,
        edges1ColSize,
        edges2,
        n2,
        edges2ColSize,
        &mut returnSize,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < returnSize {
        printf(
            b"%d \0" as *const u8 as *const libc::c_char,
            *result.offset(i_1 as isize),
        );
        i_1 += 1;
        i_1;
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n1 {
        free(*edges1.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < n2 {
        free(*edges2.offset(i_3 as isize) as *mut libc::c_void);
        i_3 += 1;
        i_3;
    }
    free(edges1 as *mut libc::c_void);
    free(edges2 as *mut libc::c_void);
    free(edges1ColSize as *mut libc::c_void);
    free(edges2ColSize as *mut libc::c_void);
    free(result as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
