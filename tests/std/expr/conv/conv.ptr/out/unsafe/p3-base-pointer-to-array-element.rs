extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait DeclaresVirtual {
    unsafe fn id(&mut self) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct HoldsId {
    pub held: i32,
}
impl Default for HoldsId {
    fn default() -> Self {
        unsafe { HoldsId { held: 0 } }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut elements: [HoldsId; 3] = [<HoldsId>::default(); 3];
    elements[(0) as usize].held = 1;
    elements[(1) as usize].held = 2;
    elements[(2) as usize].held = 3;
    let mut base: *mut dyn DeclaresVirtual = (&mut elements[(1) as usize] as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (2)));
    base = (&mut elements[(2) as usize] as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (3)));
    base = (&mut elements[(0) as usize] as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (1)));
    let mut heap: *mut HoldsId = Box::leak(
        (0..2_usize)
            .map(|_| <HoldsId>::default())
            .collect::<Box<[HoldsId]>>(),
    )
    .as_mut_ptr();
    (*heap.offset((0) as isize)).held = 8;
    (*heap.offset((1) as isize)).held = 9;
    base = (&mut (*heap.offset((1) as isize)) as *mut HoldsId);
    assert!(((unsafe { (*base).id() }) == (9)));
    base = heap;
    assert!(((unsafe { (*base).id() }) == (8)));
    {
        let __p = heap;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<HoldsId>(),
            )))
        }
    };
    return 0;
}
unsafe impl DeclaresVirtual for HoldsId {
    unsafe fn id(&mut self) -> i32 {
        return self.held;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
