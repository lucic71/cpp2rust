extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut moves_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut made_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Key {
    pub k: (),
}
impl Key {
    pub unsafe fn new(mut k: ()) -> Self {
        let mut this = Self { k: k };
        this
    }
    pub unsafe fn copy_from(o: *const Key) -> Self {
        let mut this = Self { k: (*o).k };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)).prefix_inc();
        this
    }
    pub unsafe fn move_from(o: *mut Key) -> Self {
        let mut this = Self { k: (*o).k };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)).prefix_inc();
        this
    }
    pub unsafe fn operator_lt(&self, o: *const Key) -> () {
        return ((self.k) < ((*o).k));
    }
}
impl std::cmp::Ord for Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if Key::operator_lt(self, other as *const Key) {
                std::cmp::Ordering::Less
            } else if Key::operator_lt(other, self as *const Key) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(Key::operator_lt(self, other as *const Key))
                && !(Key::operator_lt(other, self as *const Key))
        }
    }
}
impl std::cmp::Eq for Key {}
impl Clone for Key {
    fn clone(&self) -> Self {
        unsafe { Key::copy_from(self as *const Key) }
    }
}
pub unsafe fn make_key_3(mut k: ()) -> Key {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)).prefix_inc();
    return Key::new({ k });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    let mut ok: () = true;
    let mut m: () = unimplemented!();
    let mut a: Key = Key::new({ 1_() });
    let mut b: Key = Key::new({ 2_() });
    let mut c: Key = Key::new({ 3_() });
    (*unimplemented!()) = 10_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (1_()));
    (*unimplemented!()) = 20_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_()));
    (*unimplemented!()) = 11_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_()));
    ok &= (((*unimplemented!()).second) == (11_()));
    ok &= ((*unimplemented!()) == (20_()));
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_()));
    (*unimplemented!()) = 40_();
    (*unimplemented!()) = 41_();
    ok &= ((((*std::cell::LazyCell::force_mut(&mut *&raw mut made_2)) == (2_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (1_())))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_())));
    (*unimplemented!()) = 30_();
    (*unimplemented!()) = 31_();
    ok &= ((((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (2_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_())))
        && ((c.k) == (3_())));
    ok &= ((unimplemented!()) == (4_()));
    (*unimplemented!()) += 1_();
    ok &= (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (2_()))
        && ((*unimplemented!()) == (12_())));
    let mut first: *const Key = (&(*unimplemented!()).first as *const Key);
    (*unimplemented!()) = 0_();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3_()));
    ok &= ((&(*unimplemented!()).first as *const Key) == (first));
    let mut n: () = unimplemented!();
    ok &= ((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == ((2_()) + (5_())));
    ok &= (((unimplemented!()) == (5_())) && ((*unimplemented!()) == (12_())));
    unimplemented!();
    ok &= ((unimplemented!()) == (1_()));
    ok &= ((unimplemented!()) == (0_()));
    ok &= ((unimplemented!()) == (1_()));
    ok &= (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3_())));
    ok &= (((unimplemented!()) == (2_())) && ((unimplemented!()) == (5_())));
    let mut sum: () = 0_();
    'loop_: for entry in IterRange::new(unimplemented!(), unimplemented!()) {
        let entry: *const () = &(*unimplemented!());
        sum += (((*entry).first.k) * ((*entry).second));
    }
    ok &= ((sum) == (((3_()) * (31_())) + ((4_()) * (41_()))));
    ok &= (((*std::cell::LazyCell::force_mut(&mut *&raw mut copies_0)) == (7_()))
        && ((*std::cell::LazyCell::force_mut(&mut *&raw mut moves_1)) == (3_())));
    unimplemented!();
    return if ok { 0_() } else { 1_() };
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const copies_0);
    std::cell::LazyCell::force(&*&raw const moves_1);
    std::cell::LazyCell::force(&*&raw const made_2);
}
