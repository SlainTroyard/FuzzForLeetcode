use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Edge {
    pub to: libc::c_int,
    pub weight: libc::c_int,
    pub next: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StackNode {
    pub node: libc::c_int,
    pub parent: libc::c_int,
    pub processed: libc::c_int,
}
#[no_mangle]
pub static mut edgeList: [Edge; 200010] = [Edge { to: 0, weight: 0, next: 0 }; 200010];
#[no_mangle]
pub static mut headList: [libc::c_int; 100005] = [0; 100005];
#[no_mangle]
pub static mut edgeCount: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut dp0_arr: [libc::c_longlong; 100005] = [0; 100005];
#[no_mangle]
pub static mut dp1_arr: [libc::c_longlong; 100005] = [0; 100005];
#[no_mangle]
pub unsafe extern "C" fn cmp_desc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut aa: libc::c_longlong = *(a as *mut libc::c_longlong);
    let mut bb: libc::c_longlong = *(b as *mut libc::c_longlong);
    if aa < bb {
        return 1 as libc::c_int;
    }
    if aa > bb {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn add_edge(
    mut u: libc::c_int,
    mut v: libc::c_int,
    mut w: libc::c_int,
) {
    edgeList[edgeCount as usize].to = v;
    edgeList[edgeCount as usize].weight = w;
    edgeList[edgeCount as usize].next = headList[u as usize];
    let fresh0 = edgeCount;
    edgeCount = edgeCount + 1;
    headList[u as usize] = fresh0;
    edgeList[edgeCount as usize].to = u;
    edgeList[edgeCount as usize].weight = w;
    edgeList[edgeCount as usize].next = headList[v as usize];
    let fresh1 = edgeCount;
    edgeCount = edgeCount + 1;
    headList[v as usize] = fresh1;
}
#[no_mangle]
pub unsafe extern "C" fn maximizeSumOfWeights(
    mut edges: *mut *mut libc::c_int,
    mut edgesSize: libc::c_int,
    mut edgesColSize: *mut libc::c_int,
    mut k: libc::c_int,
) -> libc::c_longlong {
    let mut n: libc::c_int = edgesSize + 1 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < n {
        headList[i as usize] = -(1 as libc::c_int);
        i += 1;
        i;
    }
    edgeCount = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < edgesSize {
        let mut u: libc::c_int = *(*edges.offset(i_0 as isize))
            .offset(0 as libc::c_int as isize);
        let mut v: libc::c_int = *(*edges.offset(i_0 as isize))
            .offset(1 as libc::c_int as isize);
        let mut w: libc::c_int = *(*edges.offset(i_0 as isize))
            .offset(2 as libc::c_int as isize);
        add_edge(u, v, w);
        i_0 += 1;
        i_0;
    }
    let mut stack: [StackNode; 100005] = [StackNode {
        node: 0,
        parent: 0,
        processed: 0,
    }; 100005];
    let mut top: libc::c_int = 0 as libc::c_int;
    stack[top as usize].node = 0 as libc::c_int;
    stack[top as usize].parent = -(1 as libc::c_int);
    stack[top as usize].processed = 0 as libc::c_int;
    top += 1;
    top;
    while top > 0 as libc::c_int {
        top -= 1;
        let mut current: StackNode = stack[top as usize];
        let mut node: libc::c_int = current.node;
        let mut parent: libc::c_int = current.parent;
        if current.processed == 0 {
            stack[top as usize].node = node;
            stack[top as usize].parent = parent;
            stack[top as usize].processed = 1 as libc::c_int;
            top += 1;
            top;
            let mut edge_idx: libc::c_int = headList[node as usize];
            while edge_idx != -(1 as libc::c_int) {
                let mut child: libc::c_int = edgeList[edge_idx as usize].to;
                if child != parent {
                    stack[top as usize].node = child;
                    stack[top as usize].parent = node;
                    stack[top as usize].processed = 0 as libc::c_int;
                    top += 1;
                    top;
                }
                edge_idx = edgeList[edge_idx as usize].next;
            }
        } else {
            let mut children_count: libc::c_int = 0 as libc::c_int;
            let mut edge_idx_0: libc::c_int = headList[node as usize];
            while edge_idx_0 != -(1 as libc::c_int) {
                let mut child_0: libc::c_int = edgeList[edge_idx_0 as usize].to;
                if child_0 != parent {
                    children_count += 1;
                    children_count;
                }
                edge_idx_0 = edgeList[edge_idx_0 as usize].next;
            }
            let mut gains: *mut libc::c_longlong = malloc(
                (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong)
                    .wrapping_mul(children_count as libc::c_ulong),
            ) as *mut libc::c_longlong;
            if gains.is_null() {
                exit(1 as libc::c_int);
            }
            let mut idx: libc::c_int = 0 as libc::c_int;
            edge_idx_0 = headList[node as usize];
            let mut sum_dp0: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
            while edge_idx_0 != -(1 as libc::c_int) {
                let mut child_1: libc::c_int = edgeList[edge_idx_0 as usize].to;
                let mut weight: libc::c_int = edgeList[edge_idx_0 as usize].weight;
                if child_1 != parent {
                    *gains
                        .offset(
                            idx as isize,
                        ) = weight as libc::c_longlong + dp1_arr[child_1 as usize]
                        - dp0_arr[child_1 as usize];
                    sum_dp0 += dp0_arr[child_1 as usize];
                    idx += 1;
                    idx;
                }
                edge_idx_0 = edgeList[edge_idx_0 as usize].next;
            }
            qsort(
                gains as *mut libc::c_void,
                children_count as size_t,
                ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong,
                Some(
                    cmp_desc
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            let mut sum0: libc::c_longlong = sum_dp0;
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < children_count && i_1 < k {
                if !(*gains.offset(i_1 as isize) > 0 as libc::c_int as libc::c_longlong)
                {
                    break;
                }
                sum0 += *gains.offset(i_1 as isize);
                i_1 += 1;
                i_1;
            }
            dp0_arr[node as usize] = sum0;
            if (k - 1 as libc::c_int) < 0 as libc::c_int {
                dp1_arr[node as usize] = 0 as libc::c_int as libc::c_longlong;
            } else {
                let mut sum1: libc::c_longlong = sum_dp0;
                let mut i_2: libc::c_int = 0 as libc::c_int;
                while i_2 < children_count && i_2 < k - 1 as libc::c_int {
                    if !(*gains.offset(i_2 as isize)
                        > 0 as libc::c_int as libc::c_longlong)
                    {
                        break;
                    }
                    sum1 += *gains.offset(i_2 as isize);
                    i_2 += 1;
                    i_2;
                }
                dp1_arr[node as usize] = sum1;
            }
            free(gains as *mut libc::c_void);
        }
    }
    return dp0_arr[0 as libc::c_int as usize];
}
unsafe fn main_0() -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut n as *mut libc::c_int,
        &mut k as *mut libc::c_int,
    );
    let mut edgesSize: libc::c_int = n - 1 as libc::c_int;
    let mut edges: *mut *mut libc::c_int = malloc(
        (edgesSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < edgesSize {
        let ref mut fresh2 = *edges.offset(i as isize);
        *fresh2 = malloc(
            (3 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        i += 1;
        i;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < edgesSize {
        scanf(
            b"%d %d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*edges.offset(i_0 as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*edges.offset(i_0 as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*edges.offset(i_0 as isize)).offset(2 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i_0 += 1;
        i_0;
    }
    let mut edgesColSize: *mut libc::c_int = malloc(
        (edgesSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < edgesSize {
        *edgesColSize.offset(i_1 as isize) = 3 as libc::c_int;
        i_1 += 1;
        i_1;
    }
    let mut result: libc::c_longlong = maximizeSumOfWeights(
        edges,
        edgesSize,
        edgesColSize,
        k,
    );
    printf(b"%lld\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < edgesSize {
        free(*edges.offset(i_2 as isize) as *mut libc::c_void);
        i_2 += 1;
        i_2;
    }
    free(edges as *mut libc::c_void);
    free(edgesColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
