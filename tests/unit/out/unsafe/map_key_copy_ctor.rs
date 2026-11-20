extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut copies_0: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut moves_1: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
pub static mut made_2: libcc2rs::LazyStorage<i32> =
    libcc2rs::LazyStorage::new(|__p| unsafe { __p.write(0) });
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct Key {
    pub k: i32,
}
impl Key {
    pub unsafe fn new(this: *mut Key, mut k: i32) {
        (&raw mut (*this).k).write(k);
    }
    pub unsafe fn copy_from(this: *mut Key, o: *const Key) {
        (&raw mut (*this).k).write((*o).k);
        (*(*&raw const copies_0).get()).prefix_inc();
    }
    pub unsafe fn move_from(this: *mut Key, o: *mut Key) {
        (&raw mut (*this).k).write((*o).k);
        (*(*&raw const moves_1).get()).prefix_inc();
    }
    pub unsafe fn operator_lt(&self, o: *const Key) -> bool {
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
        unsafe {
            let mut __v: Key = ::std::mem::MaybeUninit::uninit().assume_init();
            Key::copy_from(&raw mut __v, self as *const Key);
            __v
        }
    }
}
pub unsafe fn make_key_3(__ret: *mut Key, mut k: i32) {
    (*(*&raw const made_2).get()).prefix_inc();
    Key::new(__ret.clone(), { k });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut ok: bool = true;
    let mut m: UnsafeMap<Key, (Key, i32)> = UnsafeMap::new();
    let mut a: Key = ::std::mem::MaybeUninit::uninit().assume_init();
    Key::new(&raw mut a, { 1 });
    let mut b: Key = ::std::mem::MaybeUninit::uninit().assume_init();
    Key::new(&raw mut b, { 2 });
    let mut c: Key = ::std::mem::MaybeUninit::uninit().assume_init();
    Key::new(&raw mut c, { 3 });
    (*m.subscript(&a, || 0_i32)) = 10;
    ok = ((ok as i32) & (((*(*&raw const copies_0).get()) == (1)) as i32)) != 0;
    (*m.subscript(&b, || 0_i32)) = 20;
    ok = ((ok as i32) & (((*(*&raw const copies_0).get()) == (2)) as i32)) != 0;
    (*m.subscript(&a, || 0_i32)) = 11;
    ok = ((ok as i32) & (((*(*&raw const copies_0).get()) == (2)) as i32)) != 0;
    ok = ((ok as i32) & ((((*m.find(&a).iter_deref()).1) == (11)) as i32)) != 0;
    ok = ((ok as i32) & ((((*m.get(&b).expect("out of range!")).1) == (20)) as i32)) != 0;
    ok = ((ok as i32) & (((*(*&raw const copies_0).get()) == (2)) as i32)) != 0;
    (*m.subscript_move(
        &mut *(|__p: *mut Key| {
            (unsafe { make_key_3(__p, 4) });
            __p
        })((&mut ::std::mem::MaybeUninit::<Key>::uninit()).as_mut_ptr()),
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 40;
    (*m.subscript_move(
        &mut *(|__p: *mut Key| {
            (unsafe { make_key_3(__p, 4) });
            __p
        })((&mut ::std::mem::MaybeUninit::<Key>::uninit()).as_mut_ptr()),
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 41;
    ok = ((ok as i32)
        & (((((*(*&raw const made_2).get()) == (2)) && ((*(*&raw const moves_1).get()) == (1)))
            && ((*(*&raw const copies_0).get()) == (2))) as i32))
        != 0;
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k), || 0_i32)) = 30;
    (*m.subscript_move(&mut c, |__k| cpp_move_unsafe!(__k), || 0_i32)) = 31;
    ok = ((ok as i32)
        & (((((*(*&raw const moves_1).get()) == (2)) && ((*(*&raw const copies_0).get()) == (2)))
            && ((c.k) == (3))) as i32))
        != 0;
    ok = ((ok as i32) & (((m.len()) == (4_usize)) as i32)) != 0;
    (*m.subscript(&(*m.begin().iter_deref()).0, || 0_i32)) += 1;
    ok = ((ok as i32)
        & ((((*(*&raw const copies_0).get()) == (2)) && ((*m.subscript(&a, || 0_i32)) == (12)))
            as i32))
        != 0;
    let mut first: *const Key = (&(*m.begin().iter_deref()).0 as *const Key);
    (*m.subscript_move(
        &mut *(|__p: *mut Key| {
            Key::new(__p, { 0 });
            __p
        })((&mut ::std::mem::MaybeUninit::<Key>::uninit()).as_mut_ptr()),
        |__k| cpp_move_unsafe!(__k),
        || 0_i32,
    )) = 0;
    ok = ((ok as i32) & (((*(*&raw const moves_1).get()) == (3)) as i32)) != 0;
    ok = ((ok as i32) & (((&(*m.find(&a).iter_deref()).0 as *const Key) == (first)) as i32)) != 0;
    let mut n: UnsafeMap<Key, (Key, i32)> = m.clone();
    ok = ((ok as i32) & (((*(*&raw const copies_0).get()) == ((2) + (5))) as i32)) != 0;
    ok = ((ok as i32)
        & ((((n.len()) == (5_usize)) && ((*n.subscript(&a, || 0_i32)) == (12))) as i32))
        != 0;
    m.erase(&m.find(&a));
    ok = ((ok as i32) & (((m.erase_key(&b)) == (1_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((m.erase_key(&b)) == (0_usize)) as i32)) != 0;
    ok = ((ok as i32) & (((m.erase_key(&(*m.begin().iter_deref()).0)) == (1_usize)) as i32)) != 0;
    ok = ((ok as i32)
        & ((((*(*&raw const copies_0).get()) == (7)) && ((*(*&raw const moves_1).get()) == (3)))
            as i32))
        != 0;
    ok = ((ok as i32) & ((((m.len()) == (2_usize)) && ((n.len()) == (5_usize))) as i32)) != 0;
    let mut sum: i32 = 0;
    'loop_: for entry in IterRange::new(m.begin(), m.end()) {
        let entry: *const (Key, i32) = &(*entry.iter_deref());
        sum += (((*entry).0.k) * ((*entry).1));
    }
    ok = ((ok as i32) & (((sum) == (((3) * (31)) + ((4) * (41)))) as i32)) != 0;
    ok = ((ok as i32)
        & ((((*(*&raw const copies_0).get()) == (7)) && ((*(*&raw const moves_1).get()) == (3)))
            as i32))
        != 0;
    (unsafe {
        libc::printf(
            c"%d %d %d\n".as_ptr() as *const libc::c_char,
            (*(*&raw const copies_0).get()),
            (*(*&raw const moves_1).get()),
            (*(*&raw const made_2).get()),
        )
    });
    return if ok { 0 } else { 1 };
}
pub unsafe fn __cpp2rust_init_globals() {
    (*&raw const copies_0).get();
    (*&raw const moves_1).get();
    (*&raw const made_2).get();
}
