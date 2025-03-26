use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub vertex: libc::c_int,
    pub next: *mut Node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub data: *mut libc::c_int,
    pub front: libc::c_int,
    pub rear: libc::c_int,
    pub capacity: libc::c_int,
    pub size: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn createQueue(mut capacity: libc::c_int) -> *mut Queue {
    let mut queue: *mut Queue = malloc(::core::mem::size_of::<Queue>() as libc::c_ulong)
        as *mut Queue;
    if queue.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for queue\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*queue)
        .data = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if ((*queue).data).is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for queue data\n\0" as *const u8
                as *const libc::c_char,
        );
        free(queue as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    (*queue).front = 0 as libc::c_int;
    (*queue).rear = -(1 as libc::c_int);
    (*queue).capacity = capacity;
    (*queue).size = 0 as libc::c_int;
    return queue;
}
#[no_mangle]
pub unsafe extern "C" fn isEmpty(mut queue: *mut Queue) -> bool {
    return (*queue).size == 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut queue: *mut Queue, mut item: libc::c_int) {
    if (*queue).size == (*queue).capacity {
        return;
    }
    (*queue).rear = ((*queue).rear + 1 as libc::c_int) % (*queue).capacity;
    *((*queue).data).offset((*queue).rear as isize) = item;
    (*queue).size += 1;
    (*queue).size;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(mut queue: *mut Queue) -> libc::c_int {
    if isEmpty(queue) {
        return -(1 as libc::c_int);
    }
    let mut item: libc::c_int = *((*queue).data).offset((*queue).front as isize);
    (*queue).front = ((*queue).front + 1 as libc::c_int) % (*queue).capacity;
    (*queue).size -= 1;
    (*queue).size;
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn destroyQueue(mut queue: *mut Queue) {
    free((*queue).data as *mut libc::c_void);
    free(queue as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn addEdge(
    mut adjList: *mut *mut Node,
    mut src: libc::c_int,
    mut dest: libc::c_int,
) {
    let mut newNode: *mut Node = malloc(::core::mem::size_of::<Node>() as libc::c_ulong)
        as *mut Node;
    if newNode.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for new node\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    (*newNode).vertex = dest;
    (*newNode).next = *adjList.offset(src as isize);
    let ref mut fresh0 = *adjList.offset(src as isize);
    *fresh0 = newNode;
}
#[no_mangle]
pub unsafe extern "C" fn check(
    mut n: libc::c_int,
    mut edges: *mut *mut libc::c_int,
    mut edgesSize: libc::c_int,
    mut limit: libc::c_int,
) -> bool {
    let mut adjList: *mut *mut Node = malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut Node>() as libc::c_ulong),
    ) as *mut *mut Node;
    if adjList.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for adjacency list\n\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        let ref mut fresh1 = *adjList.offset(i as isize);
        *fresh1 = 0 as *mut Node;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < edgesSize {
        if *(*edges.offset(i_0 as isize)).offset(2 as libc::c_int as isize) <= limit {
            addEdge(
                adjList,
                *(*edges.offset(i_0 as isize)).offset(1 as libc::c_int as isize),
                *(*edges.offset(i_0 as isize)).offset(0 as libc::c_int as isize),
            );
        }
        i_0 += 1;
        i_0;
    }
    let mut visited: *mut bool = calloc(
        n as libc::c_ulong,
        ::core::mem::size_of::<bool>() as libc::c_ulong,
    ) as *mut bool;
    if visited.is_null() {
        fprintf(
            stderr,
            b"Memory allocation failed for visited array\n\0" as *const u8
                as *const libc::c_char,
        );
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < n {
            let mut current: *mut Node = *adjList.offset(i_1 as isize);
            while !current.is_null() {
                let mut temp: *mut Node = current;
                current = (*current).next;
                free(temp as *mut libc::c_void);
            }
            i_1 += 1;
            i_1;
        }
        free(adjList as *mut libc::c_void);
        exit(1 as libc::c_int);
    }
    let mut queue: *mut Queue = createQueue(n);
    *visited.offset(0 as libc::c_int as isize) = 1 as libc::c_int != 0;
    enqueue(queue, 0 as libc::c_int);
    while !isEmpty(queue) {
        let mut currentVertex: libc::c_int = dequeue(queue);
        let mut temp_0: *mut Node = *adjList.offset(currentVertex as isize);
        while !temp_0.is_null() {
            let mut adjVertex: libc::c_int = (*temp_0).vertex;
            if !*visited.offset(adjVertex as isize) {
                *visited.offset(adjVertex as isize) = 1 as libc::c_int != 0;
                enqueue(queue, adjVertex);
            }
            temp_0 = (*temp_0).next;
        }
    }
    let mut allVisited: bool = 1 as libc::c_int != 0;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < n {
        if !*visited.offset(i_2 as isize) {
            allVisited = 0 as libc::c_int != 0;
            break;
        } else {
            i_2 += 1;
            i_2;
        }
    }
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < n {
        let mut current_0: *mut Node = *adjList.offset(i_3 as isize);
        while !current_0.is_null() {
            let mut temp_1: *mut Node = current_0;
            current_0 = (*current_0).next;
            free(temp_1 as *mut libc::c_void);
        }
        i_3 += 1;
        i_3;
    }
    free(adjList as *mut libc::c_void);
    free(visited as *mut libc::c_void);
    destroyQueue(queue);
    return allVisited;
}
#[no_mangle]
pub unsafe extern "C" fn findMaxWeight(
    mut edges: *mut *mut libc::c_int,
    mut edgesSize: libc::c_int,
) -> libc::c_int {
    let mut maxWeight: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edgesSize {
        if *(*edges.offset(i as isize)).offset(2 as libc::c_int as isize) > maxWeight {
            maxWeight = *(*edges.offset(i as isize)).offset(2 as libc::c_int as isize);
        }
        i += 1;
        i;
    }
    return maxWeight;
}
#[no_mangle]
pub unsafe extern "C" fn minMaxWeight(
    mut n: libc::c_int,
    mut edges: *mut *mut libc::c_int,
    mut edgesSize: libc::c_int,
    mut edgesColSize: *mut libc::c_int,
    mut threshold: libc::c_int,
) -> libc::c_int {
    let mut maxWeight: libc::c_int = findMaxWeight(edges, edgesSize);
    if !check(n, edges, edgesSize, maxWeight) {
        return -(1 as libc::c_int);
    }
    let mut left: libc::c_int = 0 as libc::c_int;
    let mut right: libc::c_int = maxWeight;
    while left < right {
        let mut mid: libc::c_int = left + (right - left) / 2 as libc::c_int;
        if check(n, edges, edgesSize, mid) {
            right = mid;
        } else {
            left = mid + 1 as libc::c_int;
        }
    }
    return left;
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut threshold: libc::c_int = 0;
    if scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut threshold as *mut libc::c_int,
    ) != 2 as libc::c_int
    {
        fprintf(
            stderr,
            b"Error reading input for n and threshold\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut edgesSize: libc::c_int = 0 as libc::c_int;
    let mut capacity: libc::c_int = 100000 as libc::c_int;
    let mut edges: *mut *mut libc::c_int = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut edgesColSize: *mut libc::c_int = malloc(
        (capacity as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut src: libc::c_int = 0;
    let mut dest: libc::c_int = 0;
    let mut weight: libc::c_int = 0;
    while scanf(
        b"%d %d %d\0" as *const u8 as *const libc::c_char,
        &mut src as *mut libc::c_int,
        &mut dest as *mut libc::c_int,
        &mut weight as *mut libc::c_int,
    ) == 3 as libc::c_int
    {
        if edgesSize >= capacity {
            fprintf(
                stderr,
                b"Warning: Maximum edge capacity reached (%d edges)\n\0" as *const u8
                    as *const libc::c_char,
                capacity,
            );
            break;
        } else {
            let ref mut fresh2 = *edges.offset(edgesSize as isize);
            *fresh2 = malloc(
                (3 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            *(*edges.offset(edgesSize as isize)).offset(0 as libc::c_int as isize) = src;
            *(*edges.offset(edgesSize as isize))
                .offset(1 as libc::c_int as isize) = dest;
            *(*edges.offset(edgesSize as isize))
                .offset(2 as libc::c_int as isize) = weight;
            *edgesColSize.offset(edgesSize as isize) = 3 as libc::c_int;
            edgesSize += 1;
            edgesSize;
        }
    }
    let mut result: libc::c_int = minMaxWeight(
        n,
        edges,
        edgesSize,
        edgesColSize,
        threshold,
    );
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edgesSize {
        free(*edges.offset(i as isize) as *mut libc::c_void);
        i += 1;
        i;
    }
    free(edges as *mut libc::c_void);
    free(edgesColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
