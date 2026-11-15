extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut sequence_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe)]
#[cpp_from(i32, new, value)]
pub struct FirstMixin {
    pub value: i32,
    pub copyStamp: i32,
}
impl FirstMixin {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            copyStamp: 0,
        };
        this
    }
    pub unsafe fn copy_from(o: *const FirstMixin) -> Self {
        let mut this = Self {
            value: (*o).value,
            copyStamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const FirstMixin) -> *mut FirstMixin {
        self.value = (*_a0).value;
        self.copyStamp = (*_a0).copyStamp;
        return &mut (*(self as *mut FirstMixin));
    }
}
impl Clone for FirstMixin {
    fn clone(&self) -> Self {
        unsafe { FirstMixin::copy_from(self as *const FirstMixin) }
    }
}
impl Default for FirstMixin {
    fn default() -> Self {
        unsafe {
            FirstMixin {
                value: 0_i32,
                copyStamp: 0,
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe)]
#[cpp_from(i32, new, value)]
pub struct SecondMixin {
    pub value: i32,
    pub copyStamp: i32,
}
impl SecondMixin {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            value: v,
            copyStamp: 0,
        };
        this
    }
    pub unsafe fn copy_from(o: *const SecondMixin) -> Self {
        let mut this = Self {
            value: (*o).value,
            copyStamp: (*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)).prefix_inc(),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const SecondMixin) -> *mut SecondMixin {
        self.value = (*_a0).value;
        self.copyStamp = (*_a0).copyStamp;
        return &mut (*(self as *mut SecondMixin));
    }
}
impl Clone for SecondMixin {
    fn clone(&self) -> Self {
        unsafe { SecondMixin::copy_from(self as *const SecondMixin) }
    }
}
impl Default for SecondMixin {
    fn default() -> Self {
        unsafe {
            SecondMixin {
                value: 0_i32,
                copyStamp: 0,
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopiesMixinsIntoBases_FirstMixin__SecondMixin_ {}
impl CopiesMixinsIntoBases_FirstMixin__SecondMixin_ {
    pub unsafe fn new(mixins_0: *const FirstMixin, mixins_1: *const SecondMixin) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(SecondMixin, new, ptr)]
pub struct CopiesMixinsIntoBases_SecondMixin_ {}
impl CopiesMixinsIntoBases_SecondMixin_ {
    pub unsafe fn new(mixins: *const SecondMixin) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CopiesMixinsIntoBases__ {}
impl CopiesMixinsIntoBases__ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
}
impl Default for CopiesMixinsIntoBases__ {
    fn default() -> Self {
        unsafe { CopiesMixinsIntoBases__::new() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut first: FirstMixin = FirstMixin::new({ 1 });
    let mut second: SecondMixin = SecondMixin::new({ 2 });
    let mut both: CopiesMixinsIntoBases_FirstMixin__SecondMixin_ =
        CopiesMixinsIntoBases_FirstMixin__SecondMixin_::new({ &first }, { &second });
    assert!((((both as FirstMixin).value) == (1)));
    assert!((((both as SecondMixin).value) == (2)));
    assert!((((both as FirstMixin).copyStamp) == (1)));
    assert!((((both as SecondMixin).copyStamp) == (2)));
    assert!(((first.copyStamp) == (0)) && ((second.copyStamp) == (0)));
    let mut one: CopiesMixinsIntoBases_SecondMixin_ =
        CopiesMixinsIntoBases_SecondMixin_::new({ &second });
    assert!((((one as SecondMixin).value) == (2)));
    assert!((((one as SecondMixin).copyStamp) == (3)));
    let mut none: CopiesMixinsIntoBases__ = CopiesMixinsIntoBases__::new();
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut sequence_0)) == (3)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const sequence_0);
}
