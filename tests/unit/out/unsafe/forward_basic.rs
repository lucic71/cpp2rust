extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type Overload = u32;
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct Tracked {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl Tracked {
    pub unsafe fn new(this: *mut Tracked, mut v: i32) {
        (&raw mut (*this).v).write(v);
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn copy_from(this: *mut Tracked, o: *const Tracked) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((((*o).copies) + (1)));
        (&raw mut (*this).moves).write((*o).moves);
    }
    pub unsafe fn move_from(this: *mut Tracked, o: *mut Tracked) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((*o).copies);
        (&raw mut (*this).moves).write((((*o).moves) + (1)));
        (*o).v = 0;
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::copy_from(&raw mut __v, self as *const Tracked);
            __v
        }
    }
}
pub unsafe fn chosen_overload_0(_a0: *const Tracked) -> Overload {
    return Overload_kLvalueOverload;
}
pub unsafe fn chosen_overload_1(_a0: *mut Tracked) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub unsafe fn new_1(this: *mut Holder, x: *mut Tracked) {
        Tracked::copy_from((&raw mut (*this).t), { x });
    }
}
impl Holder {
    pub unsafe fn new_2(this: *mut Holder, x: *mut Tracked) {
        Tracked::move_from((&raw mut (*this).t), { x });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[placement]
#[cpp_from(Tracked, new_1, mut_ptr)]
#[cpp_from_move(Tracked, new_2, ptr)]
pub struct Holder {
    pub t: Tracked,
}
impl Holder {
    pub unsafe fn copy_from(this: *mut Holder, _a0: *const Holder) {
        Tracked::copy_from((&raw mut (*this).t), { &(*_a0).t });
    }
    pub unsafe fn move_from(this: *mut Holder, _a0: *mut Holder) {
        Tracked::move_from((&raw mut (*this).t), { &mut (*_a0).t });
    }
}
pub unsafe fn forward_once_2(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_once_3(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_twice_4(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_2(x) });
}
pub unsafe fn forward_twice_5(x: *mut Tracked) -> Overload {
    return (unsafe { forward_once_3(x) });
}
pub unsafe fn forward_into_ctor_6(__ret: *mut Holder, x: *mut Tracked) {
    Holder::new_1(__ret.clone(), { x });
    return;
}
pub unsafe fn forward_into_ctor_7(__ret: *mut Holder, x: *mut Tracked) {
    Holder::new_2(__ret.clone(), { x });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut lvalue: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut lvalue, { 7 });
    assert!(
        (((unsafe { forward_once_2(&mut lvalue,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((lvalue.v) == (7)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::new(&raw mut _x, { 8 });
            forward_once_3(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut relayed: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut relayed, { 9 });
    assert!(
        (((unsafe { forward_twice_4(&mut relayed,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((relayed.v) == (9)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::new(&raw mut _x, { 10 });
            forward_twice_5(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut kept: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut kept, { 11 });
    let mut from_lvalue: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { forward_into_ctor_6(&raw mut from_lvalue, &mut kept) });
    assert!(((from_lvalue.t.v) == (11)));
    assert!(((from_lvalue.t.copies) == (1)));
    assert!(((from_lvalue.t.moves) == (0)));
    assert!(((kept.v) == (11)));
    let mut from_rvalue: Holder = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        let mut _x: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        Tracked::new(&raw mut _x, { 12 });
        forward_into_ctor_7(&raw mut from_rvalue, &mut _x)
    });
    assert!(((from_rvalue.t.v) == (12)));
    assert!(((from_rvalue.t.copies) == (0)));
    assert!(((from_rvalue.t.moves) == (1)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
