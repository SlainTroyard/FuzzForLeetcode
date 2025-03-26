use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapNode {
    pub time: libc::c_int,
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MinHeap {
    pub array: *mut HeapNode,
    pub capacity: libc::c_int,
    pub size: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn createMinHeap(mut capacity: libc::c_int) -> *mut MinHeap {
    let mut minHeap: *mut MinHeap = malloc(
        ::core::mem::size_of::<MinHeap>() as libc::c_ulong,
    ) as *mut MinHeap;
    (*minHeap).capacity = capacity;
    (*minHeap).size = 0 as libc::c_int;
    (*minHeap)
        .array = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<HeapNode>() as libc::c_ulong),
    ) as *mut HeapNode;
    return minHeap;
}
#[no_mangle]
pub unsafe extern "C" fn swapHeapNodes(mut a: *mut HeapNode, mut b: *mut HeapNode) {
    let mut temp: HeapNode = *a;
    *a = *b;
    *b = temp;
}
#[no_mangle]
pub unsafe extern "C" fn minHeapify(mut minHeap: *mut MinHeap, mut idx: libc::c_int) {
    let mut smallest: libc::c_int = idx;
    let mut left: libc::c_int = 2 as libc::c_int * idx + 1 as libc::c_int;
    let mut right: libc::c_int = 2 as libc::c_int * idx + 2 as libc::c_int;
    if left < (*minHeap).size
        && (*((*minHeap).array).offset(left as isize)).time
            < (*((*minHeap).array).offset(smallest as isize)).time
    {
        smallest = left;
    }
    if right < (*minHeap).size
        && (*((*minHeap).array).offset(right as isize)).time
            < (*((*minHeap).array).offset(smallest as isize)).time
    {
        smallest = right;
    }
    if smallest != idx {
        swapHeapNodes(
            &mut *((*minHeap).array).offset(idx as isize),
            &mut *((*minHeap).array).offset(smallest as isize),
        );
        minHeapify(minHeap, smallest);
    }
}
#[no_mangle]
pub unsafe extern "C" fn insertMinHeap(
    mut minHeap: *mut MinHeap,
    mut time: libc::c_int,
    mut x: libc::c_int,
    mut y: libc::c_int,
) {
    if (*minHeap).size == (*minHeap).capacity {
        (*minHeap).capacity *= 2 as libc::c_int;
        (*minHeap)
            .array = realloc(
            (*minHeap).array as *mut libc::c_void,
            ((*minHeap).capacity as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<HeapNode>() as libc::c_ulong),
        ) as *mut HeapNode;
    }
    let mut i: libc::c_int = (*minHeap).size;
    (*((*minHeap).array).offset(i as isize)).time = time;
    (*((*minHeap).array).offset(i as isize)).x = x;
    (*((*minHeap).array).offset(i as isize)).y = y;
    (*minHeap).size += 1;
    (*minHeap).size;
    while i != 0 as libc::c_int
        && (*((*minHeap).array)
            .offset(((i - 1 as libc::c_int) / 2 as libc::c_int) as isize))
            .time > (*((*minHeap).array).offset(i as isize)).time
    {
        swapHeapNodes(
            &mut *((*minHeap).array).offset(i as isize),
            &mut *((*minHeap).array)
                .offset(((i - 1 as libc::c_int) / 2 as libc::c_int) as isize),
        );
        i = (i - 1 as libc::c_int) / 2 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn extractMin(mut minHeap: *mut MinHeap) -> HeapNode {
    let mut minNode: HeapNode = *((*minHeap).array).offset(0 as libc::c_int as isize);
    *((*minHeap).array)
        .offset(
            0 as libc::c_int as isize,
        ) = *((*minHeap).array).offset(((*minHeap).size - 1 as libc::c_int) as isize);
    (*minHeap).size -= 1;
    (*minHeap).size;
    minHeapify(minHeap, 0 as libc::c_int);
    return minNode;
}
#[no_mangle]
pub unsafe extern "C" fn isEmpty(mut minHeap: *mut MinHeap) -> libc::c_int {
    return ((*minHeap).size == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeMinHeap(mut minHeap: *mut MinHeap) {
    free((*minHeap).array as *mut libc::c_void);
    free(minHeap as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn minTimeToReach(
    mut moveTime: *mut *mut libc::c_int,
    mut moveTimeSize: libc::c_int,
    mut moveTimeColSize: *mut libc::c_int,
) -> libc::c_int {
    if moveTimeSize == 0 as libc::c_int
        || *moveTimeColSize.offset(0 as libc::c_int as isize) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut rows: libc::c_int = moveTimeSize;
    let mut cols: libc::c_int = *moveTimeColSize.offset(0 as libc::c_int as isize);
    let mut minHeap: *mut MinHeap = createMinHeap(rows * cols);
    let mut time: *mut *mut libc::c_int = malloc(
        (rows as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows {
        let ref mut fresh0 = *time.offset(i as isize);
        *fresh0 = malloc(
            (cols as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < cols {
            *(*time.offset(i as isize)).offset(j as isize) = 2147483647 as libc::c_int;
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    insertMinHeap(minHeap, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    *(*time.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    let mut dx: [libc::c_int; 4] = [
        -(1 as libc::c_int),
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut dy: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
    ];
    while isEmpty(minHeap) == 0 {
        let mut currentNode: HeapNode = extractMin(minHeap);
        let mut currentTime: libc::c_int = currentNode.time;
        let mut x: libc::c_int = currentNode.x;
        let mut y: libc::c_int = currentNode.y;
        if x == rows - 1 as libc::c_int && y == cols - 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < rows {
                free(*time.offset(i_0 as isize) as *mut libc::c_void);
                i_0 += 1;
                i_0;
            }
            free(time as *mut libc::c_void);
            freeMinHeap(minHeap);
            return currentTime;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 4 as libc::c_int {
            let mut newX: libc::c_int = x + dx[i_1 as usize];
            let mut newY: libc::c_int = y + dy[i_1 as usize];
            if newX >= 0 as libc::c_int && newX < rows && newY >= 0 as libc::c_int
                && newY < cols
            {
                let mut waitTime: libc::c_int = if *(*moveTime.offset(newX as isize))
                    .offset(newY as isize) > currentTime
                {
                    *(*moveTime.offset(newX as isize)).offset(newY as isize)
                        - currentTime
                } else {
                    0 as libc::c_int
                };
                let mut newTime: libc::c_int = currentTime + 1 as libc::c_int + waitTime;
                if newTime < *(*time.offset(newX as isize)).offset(newY as isize) {
                    *(*time.offset(newX as isize)).offset(newY as isize) = newTime;
                    insertMinHeap(minHeap, newTime, newX, newY);
                }
            }
            i_1 += 1;
            i_1;
        }
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < rows {
        free(*time.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(time as *mut libc::c_void);
    freeMinHeap(minHeap);
    return -(1 as libc::c_int);
}
unsafe fn main_0() -> libc::c_int {
    let mut rows: libc::c_int = 0;
    let mut cols: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut rows as *mut libc::c_int,
        &mut cols as *mut libc::c_int,
    );
    let mut moveTime: *mut *mut libc::c_int = malloc(
        (rows as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows {
        let ref mut fresh1 = *moveTime.offset(i as isize);
        *fresh1 = malloc(
            (cols as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < rows {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < cols {
            scanf(
                b"%d\0" as *const u8 as *const libc::c_char,
                &mut *(*moveTime.offset(i_0 as isize)).offset(j as isize)
                    as *mut libc::c_int,
            );
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    let mut moveTimeColSize: *mut libc::c_int = malloc(
        (rows as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < rows {
        *moveTimeColSize.offset(i_1 as isize) = cols;
        i_1 += 1;
        i_1;
    }
    let mut result: libc::c_int = minTimeToReach(moveTime, rows, moveTimeColSize);
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < rows {
        free(*moveTime.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(moveTime as *mut libc::c_void);
    free(moveTimeColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
