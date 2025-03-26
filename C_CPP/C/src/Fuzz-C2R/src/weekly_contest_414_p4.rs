use ::libc;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn scanf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Position {
    pub x: libc::c_int,
    pub y: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Solution {
    pub pos: [Position; 51],
    pub dist: [[libc::c_int; 50]; 51],
    pub n: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Queue {
    pub queue: [Position; 2500],
    pub front: libc::c_int,
    pub rear: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn initQueue(mut q: *mut Queue) {
    (*q).rear = 0 as libc::c_int;
    (*q).front = (*q).rear;
}
#[no_mangle]
pub unsafe extern "C" fn enqueue(mut q: *mut Queue, mut p: Position) {
    let fresh0 = (*q).rear;
    (*q).rear = (*q).rear + 1;
    (*q).queue[fresh0 as usize] = p;
}
#[no_mangle]
pub unsafe extern "C" fn dequeue(mut q: *mut Queue) -> Position {
    let fresh1 = (*q).front;
    (*q).front = (*q).front + 1;
    return (*q).queue[fresh1 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn isEmpty(mut q: *mut Queue) -> bool {
    return (*q).front == (*q).rear;
}
#[no_mangle]
pub unsafe extern "C" fn isValid(mut x: libc::c_int, mut y: libc::c_int) -> bool {
    return x >= 0 as libc::c_int && x < 50 as libc::c_int && y >= 0 as libc::c_int
        && y < 50 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn calculateDistances(mut sol: *mut Solution) {
    let mut directions: [[libc::c_int; 2]; 8] = [
        [-(2 as libc::c_int), -(1 as libc::c_int)],
        [-(2 as libc::c_int), 1 as libc::c_int],
        [-(1 as libc::c_int), -(2 as libc::c_int)],
        [-(1 as libc::c_int), 2 as libc::c_int],
        [1 as libc::c_int, -(2 as libc::c_int)],
        [1 as libc::c_int, 2 as libc::c_int],
        [2 as libc::c_int, -(1 as libc::c_int)],
        [2 as libc::c_int, 1 as libc::c_int],
    ];
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= (*sol).n {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < (*sol).n {
            if !(i == j) {
                let mut queue: Queue = Queue {
                    queue: [Position { x: 0, y: 0 }; 2500],
                    front: 0,
                    rear: 0,
                };
                initQueue(&mut queue);
                enqueue(&mut queue, (*sol).pos[i as usize]);
                let mut seen: [[bool; 50]; 50] = [
                    [
                        0 as libc::c_int != 0,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                        false,
                    ],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                    [false; 50],
                ];
                seen[(*sol).pos[i as usize].x
                    as usize][(*sol).pos[i as usize].y as usize] = 1 as libc::c_int != 0;
                let mut steps: libc::c_int = 0 as libc::c_int;
                's_39: while !isEmpty(&mut queue) {
                    let mut size: libc::c_int = queue.rear - queue.front;
                    let mut s: libc::c_int = 0 as libc::c_int;
                    while s < size {
                        let mut current: Position = dequeue(&mut queue);
                        if current.x == (*sol).pos[j as usize].x
                            && current.y == (*sol).pos[j as usize].y
                        {
                            (*sol).dist[i as usize][j as usize] = steps;
                            break 's_39;
                        } else {
                            let mut d: libc::c_int = 0 as libc::c_int;
                            while d < 8 as libc::c_int {
                                let mut nx: libc::c_int = current.x
                                    + directions[d as usize][0 as libc::c_int as usize];
                                let mut ny: libc::c_int = current.y
                                    + directions[d as usize][1 as libc::c_int as usize];
                                if isValid(nx, ny) as libc::c_int != 0
                                    && !seen[nx as usize][ny as usize]
                                {
                                    enqueue(
                                        &mut queue,
                                        {
                                            let mut init = Position { x: nx, y: ny };
                                            init
                                        },
                                    );
                                    seen[nx as usize][ny as usize] = 1 as libc::c_int != 0;
                                }
                                d += 1;
                                d;
                            }
                            s += 1;
                            s;
                        }
                    }
                    steps += 1;
                    steps;
                }
            }
            j += 1;
            j;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub static mut memo: *mut *mut *mut libc::c_int = 0 as *const *mut *mut libc::c_int
    as *mut *mut *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn dfs(
    mut sol: *mut Solution,
    mut i: libc::c_int,
    mut m: libc::c_longlong,
    mut turn: libc::c_int,
) -> libc::c_int {
    if m == ((1 as libc::c_longlong) << (*sol).n) - 1 as libc::c_int as libc::c_longlong
    {
        return 0 as libc::c_int;
    }
    if *(*(*memo.offset(i as isize)).offset(m as isize)).offset(turn as isize)
        != -(1 as libc::c_int)
    {
        return *(*(*memo.offset(i as isize)).offset(m as isize)).offset(turn as isize);
    }
    let mut ans: libc::c_int = if turn == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        2147483647 as libc::c_int
    };
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < (*sol).n {
        if m & (1 as libc::c_longlong) << k == 0 {
            let mut next_m: libc::c_longlong = m | (1 as libc::c_longlong) << k;
            let mut result: libc::c_int = (*sol).dist[i as usize][k as usize]
                + dfs(sol, k, next_m, 1 as libc::c_int - turn);
            if turn == 0 as libc::c_int {
                ans = if result > ans { result } else { ans };
            } else {
                ans = if result < ans { result } else { ans };
            }
        }
        k += 1;
        k;
    }
    let ref mut fresh2 = *(*(*memo.offset(i as isize)).offset(m as isize))
        .offset(turn as isize);
    *fresh2 = ans;
    return *fresh2;
}
#[no_mangle]
pub unsafe extern "C" fn maxMoves(
    mut kx: libc::c_int,
    mut ky: libc::c_int,
    mut positions: *mut *mut libc::c_int,
    mut positionsSize: libc::c_int,
    mut positionsColSize: *mut libc::c_int,
) -> libc::c_int {
    let mut sol: Solution = Solution {
        pos: [Position { x: 0, y: 0 }; 51],
        dist: [[0; 50]; 51],
        n: 0,
    };
    sol.n = positionsSize;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < positionsSize {
        sol
            .pos[i as usize]
            .x = *(*positions.offset(i as isize)).offset(0 as libc::c_int as isize);
        sol
            .pos[i as usize]
            .y = *(*positions.offset(i as isize)).offset(1 as libc::c_int as isize);
        i += 1;
        i;
    }
    sol
        .pos[positionsSize
        as usize] = {
        let mut init = Position { x: kx, y: ky };
        init
    };
    memset(
        (sol.dist).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[[libc::c_int; 50]; 51]>() as libc::c_ulong,
    );
    memo = malloc(
        ((50 as libc::c_int + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(
                ::core::mem::size_of::<*mut *mut libc::c_int>() as libc::c_ulong,
            ),
    ) as *mut *mut *mut libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 <= 50 as libc::c_int {
        let ref mut fresh3 = *memo.offset(i_0 as isize);
        *fresh3 = malloc(
            (((1 as libc::c_longlong) << positionsSize) as libc::c_ulonglong)
                .wrapping_mul(
                    ::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong
                        as libc::c_ulonglong,
                ) as libc::c_ulong,
        ) as *mut *mut libc::c_int;
        let mut j: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        while j < (1 as libc::c_longlong) << positionsSize {
            let ref mut fresh4 = *(*memo.offset(i_0 as isize)).offset(j as isize);
            *fresh4 = malloc(
                (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
            ) as *mut libc::c_int;
            let ref mut fresh5 = *(*(*memo.offset(i_0 as isize)).offset(j as isize))
                .offset(1 as libc::c_int as isize);
            *fresh5 = -(1 as libc::c_int);
            *(*(*memo.offset(i_0 as isize)).offset(j as isize))
                .offset(0 as libc::c_int as isize) = *fresh5;
            j += 1;
            j;
        }
        i_0 += 1;
        i_0;
    }
    calculateDistances(&mut sol);
    let mut result: libc::c_int = dfs(
        &mut sol,
        sol.n,
        0 as libc::c_int as libc::c_longlong,
        0 as libc::c_int,
    );
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 <= 50 as libc::c_int {
        let mut j_0: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        while j_0 < (1 as libc::c_longlong) << positionsSize {
            free(
                *(*memo.offset(i_1 as isize)).offset(j_0 as isize) as *mut libc::c_void,
            );
            j_0 += 1;
            j_0;
        }
        free(*memo.offset(i_1 as isize) as *mut libc::c_void);
        i_1 += 1;
        i_1;
    }
    free(memo as *mut libc::c_void);
    return result;
}
unsafe fn main_0() -> libc::c_int {
    let mut kx: libc::c_int = 0;
    let mut ky: libc::c_int = 0;
    let mut positionsSize: libc::c_int = 0;
    scanf(
        b"%d %d\0" as *const u8 as *const libc::c_char,
        &mut kx as *mut libc::c_int,
        &mut ky as *mut libc::c_int,
    );
    scanf(
        b"%d\0" as *const u8 as *const libc::c_char,
        &mut positionsSize as *mut libc::c_int,
    );
    let mut positions: *mut *mut libc::c_int = malloc(
        (positionsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<*mut libc::c_int>() as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    let mut positionsColSize: *mut libc::c_int = malloc(
        (positionsSize as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < positionsSize {
        let ref mut fresh6 = *positions.offset(i as isize);
        *fresh6 = malloc(
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        *positionsColSize.offset(i as isize) = 2 as libc::c_int;
        scanf(
            b"%d %d\0" as *const u8 as *const libc::c_char,
            &mut *(*positions.offset(i as isize)).offset(0 as libc::c_int as isize)
                as *mut libc::c_int,
            &mut *(*positions.offset(i as isize)).offset(1 as libc::c_int as isize)
                as *mut libc::c_int,
        );
        i += 1;
        i;
    }
    let mut result: libc::c_int = maxMoves(
        kx,
        ky,
        positions,
        positionsSize,
        positionsColSize,
    );
    printf(b"%d\n\0" as *const u8 as *const libc::c_char, result);
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < positionsSize {
        free(*positions.offset(i_0 as isize) as *mut libc::c_void);
        i_0 += 1;
        i_0;
    }
    free(positions as *mut libc::c_void);
    free(positionsColSize as *mut libc::c_void);
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
