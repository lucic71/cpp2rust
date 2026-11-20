extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MoveOnly {
    pub v: i32,
}
impl MoveOnly {
    pub unsafe fn new(this: *mut MoveOnly, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn move_from(this: *mut MoveOnly, o: *mut MoveOnly) {
        (&raw mut (*this).v).write((*o).v);
        (*o).v = 0;
    }
    pub unsafe fn move_assign(&mut self, o: *mut MoveOnly) -> *mut MoveOnly {
        if ((self as *mut MoveOnly) == (o)) {
            return &mut (*(self as *mut MoveOnly));
        }
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut MoveOnly));
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ConstMoveAssign {
    pub mark: i32,
}
impl ConstMoveAssign {
    pub unsafe fn new(this: *mut ConstMoveAssign) {
        (&raw mut (*this).mark).write(0);
    }
    pub unsafe fn operator_assign_2(&mut self, o: *mut ConstMoveAssign) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (1));
        return &mut (*(self as *mut ConstMoveAssign));
    }
    pub unsafe fn operator_assign_3(&mut self, o: *const ConstMoveAssign) -> *mut ConstMoveAssign {
        self.mark = (((*o).mark) + (10));
        return &mut (*(self as *mut ConstMoveAssign));
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: ConstMoveAssign = ::std::mem::MaybeUninit::uninit().assume_init();
                ConstMoveAssign::new(&raw mut __v);
                __v
            }
        }
    }
}
pub unsafe fn make_0(__ret: *mut MoveOnly, mut v: i32) {
    let mut m: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut m, { v });
    MoveOnly::move_from(__ret.clone(), { &mut m });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut a, { 1 });
    let mut b: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut b, { 2 });
    let mut c: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut c, { 3 });
    (unsafe { MoveOnly::move_assign(&mut a, &mut b) });
    assert!(((a.v) == (2)));
    assert!(((b.v) == (0)));
    (unsafe {
        let mut _o: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        MoveOnly::new(&raw mut _o, { 3 });
        MoveOnly::move_assign(&mut b, &mut _o)
    });
    (unsafe {
        MoveOnly::move_assign(
            &mut c,
            &mut (*(unsafe { MoveOnly::move_assign(&mut a, &mut b) })),
        )
    });
    assert!((((b.v) == (0)) && ((a.v) == (0))) && ((c.v) == (3)));
    (unsafe {
        let mut _o: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        MoveOnly::new(&raw mut _o, { 5 });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (5)));
    (unsafe {
        let mut _o: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
        (unsafe { make_0(&raw mut _o, 6) });
        MoveOnly::move_assign(&mut a, &mut _o)
    });
    assert!(((a.v) == (6)));
    (unsafe {
        let _o: *mut MoveOnly = &mut a;
        MoveOnly::move_assign(&mut a, _o)
    });
    assert!(((a.v) == (6)));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    {
        let __a1 = {
            let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
            MoveOnly::new(&raw mut __v, { 7 });
            __v
        };
        vec_.push(__a1)
    };
    let mut d: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut d, { 8 });
    (unsafe { MoveOnly::move_assign(&mut vec_[(0_usize)], &mut d) });
    assert!(((vec_[(0_usize)].v) == (8)));
    assert!(((d.v) == (0)));
    let mut m: ConstMoveAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMoveAssign::new(&raw mut m);
    let mut m1: ConstMoveAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMoveAssign::new(&raw mut m1);
    let mut m2: ConstMoveAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMoveAssign::new(&raw mut m2);
    let mut cm: ConstMoveAssign = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMoveAssign::new(&raw mut cm);
    (unsafe { ConstMoveAssign::operator_assign_2(&mut m1, &mut m) });
    (unsafe { ConstMoveAssign::operator_assign_3(&mut m2, &cm) });
    assert!(((m1.mark) == (1)));
    assert!(((m2.mark) == (10)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
