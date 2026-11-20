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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
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
pub unsafe fn chosen_overload_2(_a0: *const i32) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub unsafe fn chosen_overload_3(_a0: *mut i32) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub unsafe fn forward_pack_4() -> i32 {
    let mut digits: i32 = 0;
    ();
    return digits;
}
pub unsafe fn forward_pack_5(args: *mut Tracked) -> i32 {
    let mut digits: i32 = 0;
    digits = (((digits) * (10)) + ((unsafe { chosen_overload_0(args) }) as i32));
    return digits;
}
pub unsafe fn forward_pack_6(args: *mut Tracked) -> i32 {
    let mut digits: i32 = 0;
    digits = (((digits) * (10)) + ((unsafe { chosen_overload_1(args) }) as i32));
    return digits;
}
pub unsafe fn forward_pack_7(
    args_0: *mut Tracked,
    args_1: *mut Tracked,
    args_2: *mut i32,
    args_3: *mut i32,
) -> i32 {
    let mut digits: i32 = 0;
    {
        digits = (((digits) * (10)) + ((unsafe { chosen_overload_0(args_0) }) as i32));
        {
            digits = (((digits) * (10)) + ((unsafe { chosen_overload_1(args_1) }) as i32));
            {
                digits = (((digits) * (10)) + ((unsafe { chosen_overload_2(args_2) }) as i32));
                digits = (((digits) * (10)) + ((unsafe { chosen_overload_3(args_3) }) as i32))
            }
        }
    };
    return digits;
}
impl Pair {
    pub unsafe fn new(this: *mut Pair, x: *mut Tracked, y: *mut Tracked) {
        Tracked::copy_from((&raw mut (*this).a), { x });
        Tracked::move_from((&raw mut (*this).b), { y });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Pair {
    pub a: Tracked,
    pub b: Tracked,
}
impl Pair {
    pub unsafe fn copy_from(this: *mut Pair, _a0: *const Pair) {
        Tracked::copy_from((&raw mut (*this).a), { &(*_a0).a });
        Tracked::copy_from((&raw mut (*this).b), { &(*_a0).b });
    }
    pub unsafe fn move_from(this: *mut Pair, _a0: *mut Pair) {
        Tracked::move_from((&raw mut (*this).a), { &mut (*_a0).a });
        Tracked::move_from((&raw mut (*this).b), { &mut (*_a0).b });
    }
}
pub unsafe fn forward_pack_into_ctor_8(
    __ret: *mut Pair,
    args_0: *mut Tracked,
    args_1: *mut Tracked,
) {
    Pair::new(__ret.clone(), { args_0 }, { args_1 });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    assert!(((unsafe { forward_pack_4() }) == (0)));
    let mut a: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut a, { 1 });
    assert!(((unsafe { forward_pack_5(&mut a,) }) == (Overload_kLvalueOverload as i32)));
    assert!(((a.v) == (1)));
    assert!(
        ((unsafe {
            let mut _args: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            let __tmp_0 = &raw mut _args;
            Tracked::new(__tmp_0, { 2 });
            forward_pack_6(&mut _args)
        }) == (Overload_kRvalueOverload as i32))
    );
    let mut i: i32 = 3;
    assert!(
        ((unsafe {
            let mut _args_1: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            let __tmp_1 = &raw mut _args_1;
            Tracked::new(__tmp_1, { 4 });
            let mut _args_3: i32 = 5;
            forward_pack_7(&mut a, &mut _args_1, &mut i, &mut _args_3)
        }) == (1234))
    );
    assert!(((a.v) == (1)));
    assert!(((i) == (3)));
    let mut lhs: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut lhs, { 6 });
    let mut p: Pair = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        let mut _args_1: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
        let __tmp_2 = &raw mut _args_1;
        Tracked::new(__tmp_2, { 7 });
        forward_pack_into_ctor_8(&raw mut p, &mut lhs, &mut _args_1)
    });
    assert!(((p.a.v) == (6)));
    assert!(((p.a.copies) == (1)));
    assert!(((p.b.v) == (7)));
    assert!(((p.b.copies) == (0)));
    assert!(((p.b.moves) == (1)));
    assert!(((lhs.v) == (6)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
