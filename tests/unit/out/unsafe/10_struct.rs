extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct GraphNode {
    pub dst: u32,
    pub next: *mut GraphNode,
}
impl GraphNode {
    pub unsafe fn copy_from(this: *mut GraphNode, _a0: *const GraphNode) {
        (&raw mut (*this).dst).write((*_a0).dst);
        (&raw mut (*this).next).write((*_a0).next);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Graph {
    pub V: u32,
    pub adj: *mut *mut GraphNode,
}
impl Graph {
    pub unsafe fn push(&self, mut src: u32, mut dst: u32) {
        (*self.adj.offset((src) as isize)) = {
            let __p = Box::into_raw(Box::<GraphNode>::new_uninit()) as *mut GraphNode;
            {
                let __d = __p;
                (&raw mut (*__d).dst).write(dst);
                (&raw mut (*__d).next).write((*self.adj.offset((src) as isize)));
            };
            __p
        };
        (*self.adj.offset((dst) as isize)) = {
            let __p = Box::into_raw(Box::<GraphNode>::new_uninit()) as *mut GraphNode;
            {
                let __d = __p;
                (&raw mut (*__d).dst).write(src);
                (&raw mut (*__d).next).write((*self.adj.offset((dst) as isize)));
            };
            __p
        };
    }
    pub unsafe fn copy_from(this: *mut Graph, _a0: *const Graph) {
        (&raw mut (*this).V).write((*_a0).V);
        (&raw mut (*this).adj).write((*_a0).adj);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Partial {
    pub p: *mut i32,
}
impl Partial {
    pub unsafe fn new_1(this: *mut Partial) {
        (&raw mut (*this).p).write(std::ptr::null_mut());
    }
    pub unsafe fn new_2(this: *mut Partial, mut q: *mut i32) {
        (&raw mut (*this).p).write(q);
    }
    pub unsafe fn copy_from(this: *mut Partial, _a0: *const Partial) {
        (&raw mut (*this).p).write((*_a0).p);
    }
}
impl Default for Partial {
    fn default() -> Self {
        unsafe {
            Partial {
                p: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Declared {}
impl Declared {
    pub unsafe fn copy_from(this: *mut Declared, _a0: *const Declared) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub i: i32,
    pub d: *mut Declared,
}
impl S {
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).i).write((*_a0).i);
        (&raw mut (*this).d).write((*_a0).d);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut g: Graph = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut g;
        (&raw mut (*__d).V).write(5_u32);
        (&raw mut (*__d).adj).write(std::ptr::null_mut());
    };
    let mut arr: [i32; 3] = [3, 1, 4];
    let mut it: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
    Partial::new_2(&raw mut it, { arr.as_mut_ptr() });
    if ((it.p) != (arr.as_mut_ptr())) {
        return 1;
    }
    let mut def: Partial = ::std::mem::MaybeUninit::uninit().assume_init();
    Partial::new_1(&raw mut def);
    if !((def.p).is_null()) {
        return 1;
    }
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).i).write(7);
        (&raw mut (*__d).d).write(std::ptr::null_mut());
    };
    if ((s.i) != (7)) || (!((s.d).is_null())) {
        return 1;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
