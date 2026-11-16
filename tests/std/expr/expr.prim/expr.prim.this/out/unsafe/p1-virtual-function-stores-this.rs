extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn registerSelf(&mut self);
}
pub static mut registered_globally_0: std::cell::LazyCell<*mut dyn DeclaresVirtual> =
    std::cell::LazyCell::new(|| unsafe { std::ptr::null_mut() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Registry {
    pub entry: *mut dyn DeclaresVirtual,
}
impl Default for Registry {
    fn default() -> Self {
        unsafe {
            Registry {
                entry: std::ptr::null_mut(),
            }
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(*mut Registry , new, value)]
pub struct StoresThis {
    pub registry: *mut Registry,
}
impl StoresThis {
    pub unsafe fn new(mut r: *mut Registry) -> Self {
        let mut this = Self { registry: r };
        this
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut registry: Registry = <Registry>::default();
    let mut d: StoresThis = StoresThis::new({ (&mut registry as *mut Registry) });
    let mut base: *mut dyn DeclaresVirtual = (&mut d as *mut StoresThis);
    assert!((*std::cell::LazyCell::force_mut(&mut *&raw mut registered_globally_0)).is_null());
    assert!((registry.entry).is_null());
    (unsafe { (*base).registerSelf() });
    assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut registered_globally_0)) == (base)));
    assert!(((registry.entry) == (base)));
    (*(*std::cell::LazyCell::force_mut(&mut *&raw mut registered_globally_0))).value = 3;
    assert!((((d as DeclaresVirtual).value) == (3)));
    (*registry.entry).value = 4;
    assert!((((d as DeclaresVirtual).value) == (4)));
    return 0;
}
unsafe impl DeclaresVirtual for StoresThis {
    unsafe fn registerSelf(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut registered_globally_0)) =
            (self as *mut StoresThis);
        (*self.registry).entry = (self as *mut StoresThis);
    }
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const registered_globally_0);
}
