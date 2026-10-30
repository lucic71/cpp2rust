extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct MoveOnly {
    pub v: (),
}
impl MoveOnly {
    pub unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut MoveOnly) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut MoveOnly) -> *mut MoveOnly {
        if ((self as *mut MoveOnly) == (o)) {
            return &mut (*(self as *mut MoveOnly));
        }
        self.v = (*o).v;
        (*o).v = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ConstMoveAssign {
    pub mark: (),
}
impl ConstMoveAssign {
    pub unsafe fn new() -> Self {
        let mut this = Self { mark: 0_() };
        this
    }
    pub unsafe fn operator_assign_2(&mut self, o: *mut ConstMoveAssign) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (1_()));
        return &mut (*(self as *mut ConstMoveAssign));
    }
    pub unsafe fn operator_assign_3(&mut self, o: *const ConstMoveAssign) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (10_()));
        return &mut (*(self as *mut ConstMoveAssign));
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        unsafe { ConstMoveAssign::new() }
    }
}
pub unsafe fn make_0(mut v: ()) -> MoveOnly {
    let mut m: MoveOnly = MoveOnly::new({ v });
    return MoveOnly::move_from({ &mut m });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut a: MoveOnly = MoveOnly::new({ 1_() });
    let mut b: MoveOnly = MoveOnly::new({ 2_() });
    let mut c: MoveOnly = MoveOnly::new({ 3_() });
    (unsafe { MoveOnly::move_assign(&mut a, &mut b) });
    unimplemented!();
    unimplemented!();
    (unsafe {
        let mut _o: MoveOnly = MoveOnly::new({ 3_() });
        MoveOnly::move_assign(&mut b, &mut _o)
    });
    (unsafe {
        MoveOnly::move_assign(
            &mut c,
            &mut (*(unsafe { MoveOnly::move_assign(&mut a, &mut b) })),
        )
    });
    unimplemented!();
    (unsafe {
        let mut _o: MoveOnly = MoveOnly::new({ 5_() });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    unimplemented!();
    (unsafe {
        let mut _o: MoveOnly = (unsafe { make_0(6_()) });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    unimplemented!();
    (unsafe {
        let _o: *mut MoveOnly = &mut a;
        MoveOnly::move_assign(&mut a, _o)
    });
    unimplemented!();
    let mut vec_: () = unimplemented!();
    unimplemented!();
    let mut d: MoveOnly = MoveOnly::new({ 8_() });
    (unsafe { MoveOnly::move_assign(&mut (*unimplemented!()), &mut d) });
    unimplemented!();
    unimplemented!();
    let mut m: ConstMoveAssign = ConstMoveAssign::new();
    let mut m1: ConstMoveAssign = ConstMoveAssign::new();
    let mut m2: ConstMoveAssign = ConstMoveAssign::new();
    let cm: ConstMoveAssign = ConstMoveAssign::new();
    (unsafe { ConstMoveAssign::operator_assign_2(&mut m1, &mut m) });
    (unsafe { ConstMoveAssign::operator_assign_3(&mut m2, &cm) });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
