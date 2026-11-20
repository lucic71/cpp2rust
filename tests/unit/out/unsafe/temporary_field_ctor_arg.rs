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
pub struct Outer {
    pub values: Vec<i32>,
}
impl Outer {
    pub unsafe fn copy_from(this: *mut Outer, _a0: *const Outer) {
        (&raw mut (*this).values).write((*_a0).values.clone());
    }
    pub unsafe fn move_from(this: *mut Outer, _a0: *mut Outer) {
        (&raw mut (*this).values).write(std::mem::take(&mut (*_a0).values));
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Outer) -> *mut Outer {
        {
            self.values = std::mem::take(&mut (*_a0).values);
        };
        return &mut (*(self as *mut Outer));
    }
}
pub unsafe fn make_0(__ret: *mut Outer) {
    {
        let __d = __ret.clone();
        (&raw mut (*__d).values).write(Vec::from([1, 2, 3, 4]));
    };
    return;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(Vec<i32 > , new, ptr)]
pub struct H {
    pub last: i32,
}
impl H {
    pub unsafe fn new(this: *mut H, values: *const Vec<i32>) {
        (&raw mut (*this).last).write((&(*values))[(3_usize)]);
    }
    pub unsafe fn copy_from(this: *mut H, _a0: *const H) {
        (&raw mut (*this).last).write((*_a0).last);
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut h: H = ::std::mem::MaybeUninit::uninit().assume_init();
    H::new(&raw mut h, {
        &{
            let mut __v: Outer = ::std::mem::MaybeUninit::uninit().assume_init();
            (unsafe { make_0(&raw mut __v) });
            __v
        }
        .values
    });
    (unsafe { libc::printf(c"%d\n".as_ptr() as *const libc::c_char, (h.last)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
