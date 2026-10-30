extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Elem {
    pub id: (),
    pub hist: [(); 4],
    pub coeffs: (),
}
impl Elem {
    pub unsafe fn move_from(_a0: *mut Elem) -> Self {
        let mut this = Self {
            id: (*_a0).id,
            hist: std::array::from_fn::<_, 4, _>(|__i: usize| (*_a0).hist[(__i) as usize]),
            coeffs: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Elem) -> *mut Elem {
        self.id = (*_a0).id;
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut Elem));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct State {
    pub count: (),
    pub grid: [[(); 3]; 2],
    pub arr: (),
    pub elems: (),
    pub rows: (),
}
impl State {
    pub unsafe fn move_from(_a0: *mut State) -> Self {
        let mut this = Self {
            count: (*_a0).count,
            grid: std::array::from_fn::<_, 2, _>(|__i: usize| {
                std::array::from_fn::<_, 3, _>(|__i2: usize| {
                    (*_a0).grid[(__i) as usize][(__i2) as usize]
                })
            }),
            arr: (*_a0).arr,
            elems: unimplemented!(),
            rows: unimplemented!(),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut State) -> *mut State {
        self.count = (*_a0).count;
        unimplemented!();
        self.arr = ((*_a0).arr).clone();
        unimplemented!();
        unimplemented!();
        return &mut (*(self as *mut State));
    }
}
impl Default for State {
    fn default() -> Self {
        unsafe {
            State {
                count: unimplemented!(),
                grid: [[unimplemented!(); 3]; 2],
                arr: unimplemented!(),
                elems: unimplemented!(),
                rows: unimplemented!(),
            }
        }
    }
}
pub unsafe fn sum_0(mut p: *const (), mut n: ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        s += (*p.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn fill_1(mut s: *mut State) {
    let mut i: () = 0_();
    'loop_: while ((i) < (2_())) {
        let mut j: () = 0_();
        'loop_: while ((j) < (3_())) {
            (*s).grid[(i) as usize][(j) as usize] = (((i) * (3_())) + (j));
            j.prefix_inc();
        }
        i.prefix_inc();
    }
    (*unimplemented!()) = 7_();
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (3_())) {
        (*unimplemented!()).id = i;
        (*unimplemented!()).hist[(i) as usize] = ((10_()) * (i));
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut s: State = <State>::default();
    (unsafe { fill_1((&mut s as *mut State)) });
    unimplemented!();
    unimplemented!();
    let mut e: *mut Elem = (&mut (*unimplemented!()) as *mut Elem);
    let mut h: *mut () = (*e).hist.as_mut_ptr();
    let mut c: *mut () = (&mut (*unimplemented!()) as *mut ());
    let mut id: *mut () = (&mut (*unimplemented!()).id as *mut ());
    let mut k: () = 0_();
    'loop_: while ((k) < (4_())) {
        (*h.offset((k) as isize)) += k;
        k.prefix_inc();
    }
    (*c) += 5_();
    (*id) = 11_();
    s.count = 3_();
    unimplemented!();
    unimplemented!();
    let mut row: *mut () = (&mut (*unimplemented!()) as *mut ());
    (*row.offset((2_()) as isize)) = 9_();
    unimplemented!();
    let mut t: State = s.clone();
    (*unimplemented!()).hist[(0_()) as usize] = 100_();
    t.grid[(0_()) as usize][(0_()) as usize] = 100_();
    (*unimplemented!()) = 100_();
    unimplemented!();
    unimplemented!();
    let mut p: *mut State = (&mut t as *mut State);
    (*unimplemented!()).hist[(3_()) as usize] =
        (((*p).grid[(1_()) as usize][(1_()) as usize]) + (*unimplemented!()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut elems: () = unimplemented!();
    let mut first: *mut Elem = (&mut (*unimplemented!()) as *mut Elem);
    unimplemented!();
    (*unimplemented!()) = 4_();
    (*unimplemented!()).hist[(2_()) as usize] = ((*unimplemented!()) + (1_()));
    unimplemented!();
    let mut moved: State = State::move_from({ &mut t });
    unimplemented!();
    unimplemented!();
    let mut assigned: State = <State>::default();
    (unsafe { State::move_assign(&mut assigned, &mut moved) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
