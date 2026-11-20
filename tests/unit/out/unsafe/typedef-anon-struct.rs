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
pub struct Outer_RunInfo {
    pub block_idx: i32,
    pub num_extra_zero_runs: i32,
}
impl Outer_RunInfo {
    pub unsafe fn new_1(this: *mut Outer_RunInfo) {}
    pub unsafe fn copy_from(this: *mut Outer_RunInfo, _a0: *const Outer_RunInfo) {
        (&raw mut (*this).block_idx).write((*_a0).block_idx);
        (&raw mut (*this).num_extra_zero_runs).write((*_a0).num_extra_zero_runs);
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub runs: Vec<Outer_RunInfo>,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).runs).write((*_a0).runs.clone());
    }
    pub unsafe fn move_from(this: *mut Outer, _a0: *mut Outer) {
        (&raw mut (*this).runs).write(std::mem::take(&mut (*_a0).runs));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        {
            self.runs = std::mem::take(&mut (*_a0).runs);
        };
        return &mut (*(self as *mut Outer));
    }
    pub unsafe fn new_1(this: *mut Outer) {
        (&raw mut (*this).runs).write(Vec::new());
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
    Outer::new_1(&raw mut o);
    let mut info: Outer_RunInfo = ::std::mem::MaybeUninit::uninit().assume_init();
    Outer_RunInfo::new_1(&raw mut info);
    info.block_idx = 1;
    info.num_extra_zero_runs = 2;
    {
        let a0_clone = info.clone();
        o.runs.push(a0_clone)
    };
    assert!(((o.runs.len()) == (1_usize)));
    assert!(((o.runs[(0_usize)].block_idx) == (1)));
    assert!(((o.runs[(0_usize)].num_extra_zero_runs) == (2)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
