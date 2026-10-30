extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn set_0(mut p: *mut (), mut v: ()) {
    (*p) = v;
}
pub unsafe fn inc_1(r: *mut ()) {
    (*r).prefix_inc();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: (),
    pub second: (),
}
pub unsafe fn total_2(v: *const ()) -> () {
    let mut s: () = 0_();
    'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut x: () = (*unimplemented!());
        s += x;
    }
    return s;
}
pub unsafe fn sum_3(mut arr: *const (), mut n: ()) -> () {
    let mut s: () = 0_();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        s += (*arr.offset((i) as isize));
        i.prefix_inc();
    }
    return s;
}
pub unsafe fn countdown_4(mut n: (), mut step: Option<()>) -> () {
    let mut step: () = step.unwrap_or_else(|| unsafe { 1_() });
    let mut steps: () = 0_();
    'loop_: while ((n) > (0_())) {
        n -= step;
        steps.postfix_inc();
    }
    return steps;
}
pub unsafe trait Shape {
    unsafe fn scale(&mut self, factor: ()) -> ();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Square {
    pub side: (),
}
impl Default for Square {
    fn default() -> Self {
        unsafe { Square { side: 2_() } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut plain: () = 1_();
    plain += 2_();
    plain.postfix_inc();
    unimplemented!();
    &(::std::mem::size_of::<()>());
    let mut addr: () = 0_();
    (unsafe { set_0((&mut addr as *mut ()), 5_()) });
    unimplemented!();
    let mut ref_: () = 1_();
    (unsafe { inc_1(&mut ref_) });
    let alias: *mut () = &mut ref_;
    (*alias).postfix_inc();
    unimplemented!();
    let mut captured: () = 7_();
    let mut get: FnPtr<fn() -> ()> = lambda_unsafe!(
        {
            let captured: *mut () = &mut captured;
        },
        || -> () {
            return (*captured);
        }
    );
    captured = 8_();
    unimplemented!();
    let mut arr: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    arr[(0_()) as usize] = ((arr[(3_()) as usize]) * (2_()));
    arr[(1_()) as usize].postfix_inc();
    unimplemented!();
    let mut elem: [(); 2] = [0_(), 0_()];
    (unsafe { set_0((&mut elem[(1_()) as usize] as *mut ()), 6_()) });
    unimplemented!();
    let mut decayed: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    let mut elem_ref: [(); 2] = [1_(), 1_()];
    (unsafe { inc_1(&mut elem_ref[(0_()) as usize]) });
    unimplemented!();
    let mut p: *mut () = (&mut addr as *mut ());
    p = (&mut elem[(0_()) as usize] as *mut ());
    (*p) = 9_();
    unimplemented!();
    let mut str: [(); 8] = std::mem::transmute(*b"abc\0\0\0\0\0");
    str[(0_()) as usize] = ('x' as ());
    unimplemented!();
    let mut zeros: [(); 16] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    zeros[(15_()) as usize] = 1.5_f64;
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut init: () = 3_();
    let mut pair: Pair = Pair {
        first: init,
        second: ((init) + (1_())),
    };
    let mut heap: *mut Pair = (Box::leak(Box::new(Pair {
        first: init,
        second: init,
    })) as *mut Pair);
    unimplemented!();
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut vec_: () = unimplemented!();
    let mut four: () = 4_();
    unimplemented!();
    (*unimplemented!()) = ((*unimplemented!()) + (1_()));
    (*unimplemented!()).postfix_inc();
    unimplemented!();
    let mut high: () = 3_();
    let mut low: () = (*unimplemented!());
    unimplemented!();
    let mut by_ref: () = unimplemented!();
    unimplemented!();
    let mut elem_ptr: () = unimplemented!();
    (unsafe { set_0((&mut (*unimplemented!()) as *mut ()), 7_()) });
    unimplemented!();
    let mut data: () = unimplemented!();
    (unsafe { set_0(unimplemented!(), 8_()) });
    unimplemented!();
    let mut square: Square = <Square>::default();
    let mut shape: *mut dyn Shape = (&mut square as *mut Square);
    unimplemented!();
    return 0_();
}
unsafe impl Shape for Square {
    unsafe fn scale(&mut self, factor: ()) -> () {
        return ((factor) * (self.side));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
