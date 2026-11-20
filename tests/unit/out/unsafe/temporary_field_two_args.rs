extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Inner {
    pub values: Vec<i32>,
}
impl Inner {
    pub unsafe fn copy_from(this: *mut Inner, _a0: *const Inner) {
        (&raw mut (*this).values).write((*_a0).values.clone());
    }
    pub unsafe fn move_from(this: *mut Inner, _a0: *mut Inner) {
        (&raw mut (*this).values).write(std::mem::take(&mut (*_a0).values));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Inner) -> *mut Inner {
        {
            self.values = std::mem::take(&mut (*_a0).values);
        };
        return &mut (*(self as *mut Inner));
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct Outer {
    pub values: Vec<i32>,
    pub inner: Inner,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).values).write((*_a0).values.clone());
        Inner::copy_from((&raw mut (*this).inner), { &(*_a0).inner });
    }
    pub unsafe fn move_from(this: *mut Outer, _a0: *mut Outer) {
        (&raw mut (*this).values).write(std::mem::take(&mut (*_a0).values));
        Inner::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        {
            self.values = std::mem::take(&mut (*_a0).values);
        };
        (unsafe {
            let _arg0: *mut Inner = &mut (*_a0).inner;
            Inner::move_assign(&mut self.inner, _arg0)
        });
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0(__ret: *mut Outer) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).values).write(Vec::from([1, 2, 3]));
        {
            let __d = (&raw mut (*__d).inner);
            (&raw mut (*__d).values).write(Vec::from([4, 5, 6]));
        };
    };
    return;
}
pub unsafe fn two_1(a: *const Vec<i32>, b: *const Vec<i32>) -> i32 {
    return (((&(*a))[(0_usize)]) + ((&(*b))[(2_usize)]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe {
        libc::printf(
            c"%d\n".as_ptr() as *const libc::c_char,
            (unsafe {
                let _a: *const Vec<i32> = &{
                    let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
                    (unsafe { make_0(&raw mut __v) });
                    __v
                }
                .values;
                let _b: *const Vec<i32> = &{
                    let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
                    (unsafe { make_0(&raw mut __v) });
                    __v
                }
                .inner
                .values;
                two_1(_a, _b)
            }),
        )
    });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
