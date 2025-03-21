use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HeapNode {
    pub time: libc::c_int,
    pub row: libc::c_int,
    pub col: libc::c_int,
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
    mut row: libc::c_int,
    mut col: libc::c_int,
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
    (*((*minHeap).array).offset(i as isize)).row = row;
    (*((*minHeap).array).offset(i as isize)).col = col;
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
    let mut vis: *mut *mut libc::c_int = malloc(
        (rows as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < rows {
        let ref mut fresh0 = *vis.offset(i as isize);
        *fresh0 = calloc(
            cols as libc::c_ulong,
            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    let mut minHeap: *mut MinHeap = createMinHeap(rows * cols);
    insertMinHeap(minHeap, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int);
    *(*vis.offset(0 as libc::c_int as isize))
        .offset(0 as libc::c_int as isize) = 1 as libc::c_int;
    let mut drow: [libc::c_int; 4] = [
        -(1 as libc::c_int),
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ];
    let mut dcol: [libc::c_int; 4] = [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        -(1 as libc::c_int),
    ];
    let mut time: libc::c_int = 0 as libc::c_int;
    while isEmpty(minHeap) == 0 {
        let mut currentNode: HeapNode = extractMin(minHeap);
        time = currentNode.time;
        let mut r: libc::c_int = currentNode.row;
        let mut c: libc::c_int = currentNode.col;
        if r == rows - 1 as libc::c_int && c == cols - 1 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < rows {
                free(*vis.offset(i_0 as isize) as *mut libc::c_void);
                i_0 += 1;
                i_0;
            }
            free(vis as *mut libc::c_void);
            freeMinHeap(minHeap);
            return time;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < 4 as libc::c_int {
            let mut nrow: libc::c_int = r + drow[i_1 as usize];
            let mut ncol: libc::c_int = c + dcol[i_1 as usize];
            if nrow >= 0 as libc::c_int && nrow < rows && ncol >= 0 as libc::c_int
                && ncol < cols
                && *(*vis.offset(nrow as isize)).offset(ncol as isize)
                    == 0 as libc::c_int
            {
                let mut nextCost: libc::c_int = 2 as libc::c_int
                    - (nrow + ncol) % 2 as libc::c_int;
                let mut newTime: libc::c_int = 0;
                if *(*moveTime.offset(nrow as isize)).offset(ncol as isize) >= time {
                    newTime = *(*moveTime.offset(nrow as isize)).offset(ncol as isize)
                        + nextCost;
                } else {
                    newTime = time + nextCost;
                }
                insertMinHeap(minHeap, newTime, nrow, ncol);
                *(*vis.offset(nrow as isize)).offset(ncol as isize) = 1 as libc::c_int;
            }
            i_1 += 1;
            i_1;
        }
    }
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < rows {
        free(*vis.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(vis as *mut libc::c_void);
    freeMinHeap(minHeap);
    return time;
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
