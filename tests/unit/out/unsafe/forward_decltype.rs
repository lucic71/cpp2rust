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
pub unsafe fn forward_by_decltype_2(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_by_decltype_3(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_4(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_0(x) });
}
pub unsafe fn forward_abbreviated_5(x: *mut Tracked) -> Overload {
    return (unsafe { chosen_overload_1(x) });
}
pub unsafe fn forward_abbreviated_pack_6(args_0: *mut Tracked, args_1: *mut Tracked) -> i32 {
    return (((unsafe { chosen_overload_0(args_0) }) as i32)
        + ((unsafe { chosen_overload_1(args_1) }) as i32));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut a, { 3 });
    assert!(
        (((unsafe { forward_by_decltype_2(&mut a,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((a.v) == (3)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::new(&raw mut _x, { 4 });
            forward_by_decltype_3(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut b: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut b, { 5 });
    assert!(
        (((unsafe { forward_abbreviated_4(&mut b,) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(((b.v) == (5)));
    assert!(
        (((unsafe {
            let mut _x: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::new(&raw mut _x, { 6 });
            forward_abbreviated_5(&mut _x)
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let mut c: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut c, { 7 });
    assert!(
        ((unsafe {
            let mut _args_1: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
            Tracked::new(&raw mut _args_1, { 8 });
            forward_abbreviated_pack_6(&mut c, &mut _args_1)
        }) == ((Overload_kLvalueOverload as i32) + (Overload_kRvalueOverload as i32)))
    );
    assert!(((c.v) == (7)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
