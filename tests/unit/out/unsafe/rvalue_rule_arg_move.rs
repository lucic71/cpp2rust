extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
pub static mut moves_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, CopyAssignUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Elem {
    pub v: i32,
}
impl Elem {
    pub unsafe fn new(this: *mut Elem, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn copy_from(this: *mut Elem, o: *const Elem) {
        (&raw mut (*this).v).write((*o).v);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Elem, o: *mut Elem) {
        (&raw mut (*this).v).write((*o).v);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const Elem) -> *mut Elem {
        self.v = (*_a0).v;
        return &mut (*(self as *mut Elem));
    }
}
impl Clone for Elem {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
            Elem::copy_from(&raw mut __v, self as *const Elem);
            __v
        }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v: Vec<Elem> = Vec::new();
    if 4_usize > v.capacity() as usize {
        let len_0 = v.len();
        v.reserve_exact(4_usize - len_0 as usize);
    };
    let mut a: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
    Elem::new(&raw mut a, { 1 });
    let mut b: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
    Elem::new(&raw mut b, { 2 });
    {
        let pos = <UnsafeRandomAccessIterator<Elem>>::from(v.end())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v.insert_mut(pos, {
            let mut __v: Elem = ::std::mem::MaybeUninit::uninit().assume_init();
            Elem::move_from(&raw mut __v, { &mut a });
            __v
        }))
    };
    {
        let pos = <UnsafeRandomAccessIterator<Elem>>::from(v.end())
            .iter_distance(&UnsafeRandomAccessIterator::Vec(v.as_mut_ptr()))
            as usize;
        UnsafeRandomAccessIterator::Vec(v.insert_mut(pos, (b).clone()))
    };
    let mut ok: bool = ((((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1)))
        && ((v[(0_usize)].v) == (1)))
        && ((v[(1_usize)].v) == (2));
    let mut pa: *mut Elem = (&mut a as *mut Elem);
    (unsafe {
        libc::printf(
            c"%d %d %d\n".as_ptr() as *const libc::c_char,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)),
            ((*pa).v),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
}
