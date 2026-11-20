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
pub unsafe fn chosen_overload_2(_a0: *const i32) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub unsafe fn chosen_overload_3(_a0: *mut i32) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub unsafe fn copy_or_move_into_param_4(__ret: *mut Tracked, t: *mut Tracked) {
    Tracked::move_from(__ret.clone(), { &mut (*t) });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut a, { 5 });
    assert!((((unsafe { chosen_overload_0(&a,) }) as i32) == (Overload_kLvalueOverload as i32)));
    assert!(((a.v) == (5)));
    assert!(
        (((unsafe { chosen_overload_1(&mut a,) }) as i32) == (Overload_kRvalueOverload as i32))
    );
    assert!(((a.v) == (5)));
    let mut b: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut b, { 6 });
    let mut moved: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        copy_or_move_into_param_4(
            &raw mut moved,
            (|__p: *mut Tracked| {
                Tracked::move_from(__p, { &mut b });
                __p
            })((&mut ::std::mem::MaybeUninit::<Tracked>::uninit()).as_mut_ptr()),
        )
    });
    assert!(((moved.v) == (6)));
    assert!(((moved.copies) == (0)));
    assert!(((moved.moves) == (2)));
    assert!(((b.v) == (0)));
    let mut c: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    Tracked::new(&raw mut c, { 7 });
    let mut copied: Tracked = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        copy_or_move_into_param_4(
            &raw mut copied,
            (|__p: *mut Tracked| {
                Tracked::copy_from(__p, { &c });
                __p
            })((&mut ::std::mem::MaybeUninit::<Tracked>::uninit()).as_mut_ptr()),
        )
    });
    assert!(((copied.v) == (7)));
    assert!(((copied.copies) == (1)));
    assert!(((copied.moves) == (1)));
    assert!(((c.v) == (7)));
    let mut i: i32 = 8;
    assert!((((unsafe { chosen_overload_2(&i,) }) as i32) == (Overload_kIntLvalueOverload as i32)));
    assert!(
        (((unsafe { chosen_overload_3(&mut i,) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(
        (((unsafe { chosen_overload_3(&mut i,) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(((i) == (8)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
