use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TreeNode {
    pub val: libc::c_int,
    pub left: *mut TreeNode,
    pub right: *mut TreeNode,
}
static mut arr: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut arr_size: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn tree_judge(
    mut root: *mut TreeNode,
    mut floor: libc::c_int,
) -> libc::c_int {
    let mut left_floor: libc::c_int = 0 as libc::c_int;
    let mut right_floor: libc::c_int = 0 as libc::c_int;
    if !((*root).left).is_null() && !((*root).right).is_null() {
        left_floor = tree_judge((*root).left, floor + 1 as libc::c_int);
        right_floor = tree_judge((*root).right, floor + 1 as libc::c_int);
    } else if !((*root).left).is_null() {
        left_floor = tree_judge((*root).left, floor + 1 as libc::c_int);
        right_floor = 0 as libc::c_int;
    } else if !((*root).right).is_null() {
        left_floor = 0 as libc::c_int;
        right_floor = tree_judge((*root).right, floor + 1 as libc::c_int);
    } else {
        left_floor = floor;
        right_floor = floor;
    }
    if left_floor == right_floor && right_floor >= floor {
        let fresh0 = arr_size;
        arr_size = arr_size + 1;
        *arr
            .offset(
                fresh0 as isize,
            ) = (pow(
            2 as libc::c_int as libc::c_double,
            (right_floor - floor + 1 as libc::c_int) as libc::c_double,
        ) - 1 as libc::c_int as libc::c_double) as libc::c_int;
        return left_floor;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quick_sort(mut l: libc::c_int, mut r: libc::c_int) {
    if l >= r {
        return;
    }
    let mut l_t: libc::c_int = l - 1 as libc::c_int;
    let mut r_t: libc::c_int = r + 1 as libc::c_int;
    let mut mid_val: libc::c_int = *arr.offset((l + r >> 1 as libc::c_int) as isize);
    while l_t < r_t {
        loop {
            l_t += 1;
            l_t;
            if !(*arr.offset(l_t as isize) < mid_val) {
                break;
            }
        }
        loop {
            r_t -= 1;
            r_t;
            if !(*arr.offset(r_t as isize) > mid_val) {
                break;
            }
        }
        if l_t < r_t {
            let mut tmp: libc::c_int = *arr.offset(l_t as isize);
            *arr.offset(l_t as isize) = *arr.offset(r_t as isize);
            *arr.offset(r_t as isize) = tmp;
        }
    }
    quick_sort(l, r_t);
    quick_sort(r_t + 1 as libc::c_int, r);
}
#[no_mangle]
pub unsafe extern "C" fn kthLargestPerfectSubtree(
    mut root: *mut TreeNode,
    mut k: libc::c_int,
) -> libc::c_int {
    arr = malloc(
        (10000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    arr_size = 0 as libc::c_int;
    tree_judge(root, 1 as libc::c_int);
    quick_sort(0 as libc::c_int, arr_size - 1 as libc::c_int);
    if arr_size - k < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    return *arr.offset((arr_size - k) as isize);
}
#[no_mangle]
pub unsafe extern "C" fn create_tree(
    mut arr_0: *mut libc::c_int,
    mut size: libc::c_int,
) -> *mut TreeNode {
    if size == 0 as libc::c_int {
        return 0 as *mut TreeNode;
    }
    let mut root: *mut TreeNode = malloc(
        ::core::mem::size_of::<TreeNode>() as libc::c_ulong,
    ) as *mut TreeNode;
    (*root).val = *arr_0.offset(0 as libc::c_int as isize);
    (*root).left = 0 as *mut TreeNode;
    (*root).right = 0 as *mut TreeNode;
    let mut queue: *mut *mut TreeNode = malloc(
        (size as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut TreeNode>() as libc::c_ulong),
    ) as *mut *mut TreeNode;
    let mut front: libc::c_int = 0 as libc::c_int;
    let mut rear: libc::c_int = 0 as libc::c_int;
    let fresh1 = rear;
    rear = rear + 1;
    let ref mut fresh2 = *queue.offset(fresh1 as isize);
    *fresh2 = root;
    let mut i: libc::c_int = 1 as libc::c_int;
    while i < size {
        let fresh3 = front;
        front = front + 1;
        let mut current: *mut TreeNode = *queue.offset(fresh3 as isize);
        if *arr_0.offset(i as isize) != 0 as libc::c_int {
            let mut leftNode: *mut TreeNode = malloc(
                ::core::mem::size_of::<TreeNode>() as libc::c_ulong,
            ) as *mut TreeNode;
            (*leftNode).val = *arr_0.offset(i as isize);
            (*leftNode).left = 0 as *mut TreeNode;
            (*leftNode).right = 0 as *mut TreeNode;
            (*current).left = leftNode;
            let fresh4 = rear;
            rear = rear + 1;
            let ref mut fresh5 = *queue.offset(fresh4 as isize);
            *fresh5 = leftNode;
        }
        if (i + 1 as libc::c_int) < size
            && *arr_0.offset((i + 1 as libc::c_int) as isize) != 0 as libc::c_int
        {
            let mut rightNode: *mut TreeNode = malloc(
                ::core::mem::size_of::<TreeNode>() as libc::c_ulong,
            ) as *mut TreeNode;
            (*rightNode).val = *arr_0.offset((i + 1 as libc::c_int) as isize);
            (*rightNode).left = 0 as *mut TreeNode;
            (*rightNode).right = 0 as *mut TreeNode;
            (*current).right = rightNode;
            let fresh6 = rear;
            rear = rear + 1;
            let ref mut fresh7 = *queue.offset(fresh6 as isize);
            *fresh7 = rightNode;
        }
        i += 2 as libc::c_int;
    }
    free(queue as *mut libc::c_void);
    return root;
}
#[no_mangle]
pub unsafe extern "C" fn free_tree(mut root: *mut TreeNode) {
    if root.is_null() {
        return;
    }
    free_tree((*root).left);
    free_tree((*root).right);
    free(root as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn print_tree(mut root: *mut TreeNode) {
    let mut queue: *mut *mut TreeNode = malloc(
        (10000 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut TreeNode>() as libc::c_ulong),
    ) as *mut *mut TreeNode;
    let mut front: libc::c_int = 0 as libc::c_int;
    let mut rear: libc::c_int = 0 as libc::c_int;
    let fresh8 = rear;
    rear = rear + 1;
    let ref mut fresh9 = *queue.offset(fresh8 as isize);
    *fresh9 = root;
    while front < rear {
        let fresh10 = front;
        front = front + 1;
        let mut node: *mut TreeNode = *queue.offset(fresh10 as isize);
        if node.is_null() {
            printf(b"null \0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"%d \0" as *const u8 as *const libc::c_char, (*node).val);
            let fresh11 = rear;
            rear = rear + 1;
            let ref mut fresh12 = *queue.offset(fresh11 as isize);
            *fresh12 = (*node).left;
            let fresh13 = rear;
            rear = rear + 1;
            let ref mut fresh14 = *queue.offset(fresh13 as isize);
            *fresh14 = (*node).right;
        }
    }
}
unsafe fn main_0() -> libc::c_int {
    let mut k: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut k as *mut libc::c_int);
    let mut arrSize: libc::c_int = 0;
    scanf(b"%d\0" as *const u8 as *const libc::c_char, &mut arrSize as *mut libc::c_int);
    let mut arr_0: *mut libc::c_int = malloc(
        (arrSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < arrSize {
        scanf(
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut *arr_0.offset(i as isize) as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut root: *mut TreeNode = create_tree(arr_0, arrSize);
    printf(
        b"%d\n\0" as *const u8 as *const libc::c_char,
        kthLargestPerfectSubtree(root, k),
    );
    free(arr_0 as *mut libc::c_void);
    free_tree(root);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
