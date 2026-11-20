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
pub struct Static {}
impl Static {
    pub unsafe fn operator_call(mut a: i32, mut b: i32) -> i32 {
        return ((a) * (b));
    }
    pub unsafe fn new_1(this: *mut Static) {}
    pub unsafe fn copy_from(this: *mut Static, _a0: *const Static) {}
    pub unsafe fn move_from(this: *mut Static, _a0: *mut Static) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(i32, to_i32)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn operator_call_1(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn operator_call_2(&self, mut a: i32) -> i32 {
        return ((self.v) + (a));
    }
    pub unsafe fn operator_call_3(&self, mut a: i32, mut b: i32) -> i32 {
        return (((self.v) + (a)) + (b));
    }
    pub unsafe fn operator_comma(&self, __ret: *mut S, o: *const S) {
        {
            let __d = __ret.clone();
            (&raw mut (*__d).v).write((((self.v) * (10)) + ((*o).v)));
        };
        return;
    }
    pub unsafe fn to_i32(&self) -> i32 {
        return self.v;
    }
    pub unsafe fn to_bool(&self) -> bool {
        return ((self.v) != (0));
    }
    pub unsafe fn new_1(this: *mut S) {}
    pub unsafe fn copy_from(this: *mut S, _a0: *const S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
    pub unsafe fn move_from(this: *mut S, _a0: *mut S) {
        (&raw mut (*this).v).write((*_a0).v);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut s;
        (&raw mut (*__d).v).write(3);
    };
    let mut t: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut t;
        (&raw mut (*__d).v).write(4);
    };
    assert!(((unsafe { S::operator_call_1(&s,) }) == (3)));
    assert!(((unsafe { S::operator_call_2(&s, 1,) }) == (4)));
    assert!(((unsafe { S::operator_call_3(&s, 1, 2,) }) == (6)));
    assert!(
        ((({
            let mut __v: S = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { S::operator_comma(&s, &raw mut __v, &t) });
            __v
        })
        .v) == (34))
    );
    let mut i: i32 = (unsafe { S::to_i32(&s) });
    assert!(((i) == (3)));
    assert!((((unsafe { S::to_i32(&s,) }) + (1)) == (4)));
    if (unsafe { S::to_bool(&s) }) {
        assert!((unsafe { S::to_bool(&s,) }));
    } else {
        assert!(false);
    }
    let mut z: S = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut z;
        (&raw mut (*__d).v).write(0);
    };
    assert!((unsafe { S::to_bool(&s,) }));
    assert!(!(unsafe { S::to_bool(&z,) }));
    assert!((unsafe { S::to_bool(&s,) }) && (!(unsafe { S::to_bool(&z,) })));
    let mut st: Static = ::std::mem::MaybeUninit::uninit().assume_init();
    Static::new_1(&raw mut st);
    assert!(((unsafe { Static::operator_call(6, 7,) }) == (42)));
    assert!(
        ((unsafe {
            S::operator_call_1(&mut *(|__p: *mut S| {
                {
                    let __d = __p;
                    (&raw mut (*__d).v).write(5);
                };
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr(),
            ))
        }) == (5))
    );
    assert!(
        ((unsafe {
            S::operator_call_3(
                &mut *(|__p: *mut S| {
                    {
                        let __d = __p;
                        (&raw mut (*__d).v).write(5);
                    };
                    __p
                })((&mut ::std::mem::MaybeUninit::<S>::uninit()).as_mut_ptr()),
                1,
                1,
            )
        }) == (7))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
