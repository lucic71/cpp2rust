extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Chain {
    pub v: i32,
}
impl Chain {
    pub unsafe fn new(this: *mut Chain, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Chain, o: *const Chain) {
        (&raw mut (*this).v).write((((*o).v) + (100)));
    }
    pub unsafe fn move_from(this: *mut Chain, o: *mut Chain) {
        (&raw mut (*this).v).write((((*o).v) + (1)));
        (*o).v = 0;
    }
    pub unsafe fn add_4(&mut self, mut n: i32) -> *mut Chain {
        self.v += n;
        return &mut (*(self as *mut Chain));
    }
    pub unsafe fn add_5(&mut self, mut n: i32) -> *mut Chain {
        self.v += n;
        return &mut (*(self as *mut Chain));
    }
    pub unsafe fn take(&mut self, __ret: *mut Chain) {
        Chain::move_from(__ret.clone(), { &mut (*(self as *mut Chain)) });
        return;
    }
    pub unsafe fn copy(&self, __ret: *mut Chain) {
        Chain::copy_from(__ret.clone(), { &(*(self as *const Chain)) });
        return;
    }
    pub unsafe fn self_(&mut self) -> *mut Chain {
        return &mut (*(self as *mut Chain));
    }
}
impl Clone for Chain {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
            Chain::copy_from(&raw mut __v, self as *const Chain);
            __v
        }
    }
}
pub unsafe fn consume_0(c: *mut Chain) -> i32 {
    return (*c).v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    Chain::new(&raw mut a, { 1 });
    (unsafe { Chain::add_4(&mut (*(unsafe { Chain::add_4(&mut a, 1) })), 1) });
    assert!(((a.v) == (3)));
    let mut b0: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    Chain::new(&raw mut b0, { 5 });
    let mut b: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    Chain::move_from(&raw mut b, {
        (unsafe { Chain::add_5(&mut (*(unsafe { Chain::add_5(&mut b0, 1) })), 1) })
    });
    assert!(((b.v) == (8)) && ((b0.v) == (0)));
    let mut c: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe {
        Chain::take(
            &mut *(|__p: *mut Chain| {
                Chain::new(__p, { 10 });
                __p
            })((&mut ::std::mem::MaybeUninit::<Chain>::uninit()).as_mut_ptr()),
            &raw mut c,
        )
    });
    assert!(((c.v) == (11)));
    let mut d: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { Chain::copy(&c, &raw mut d) });
    assert!(((d.v) == (111)) && ((c.v) == (11)));
    let mut g: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    Chain::new(&raw mut g, { 20 });
    assert!(
        ((unsafe {
            consume_0((|__p: *mut Chain| {
                Chain::move_from(__p, { (unsafe { Chain::self_(&mut g) }) });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<Chain>::uninit()).as_mut_ptr(),
            ))
        }) == (21))
    );
    let mut e: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    Chain::new(&raw mut e, { 30 });
    let mut f: Chain = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { Chain::take(&mut e, &raw mut f) });
    assert!(((f.v) == (31)) && ((e.v) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
