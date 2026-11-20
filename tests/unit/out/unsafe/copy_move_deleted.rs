extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe, Default)]
#[placement]
#[cpp_from(i32, new, value)]
pub struct NoCopy {
    pub v: i32,
}
impl NoCopy {
    pub unsafe fn new(this: *mut NoCopy, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn move_from(this: *mut NoCopy, o: *mut NoCopy) {
        (&raw mut (*this).v).write((*o).v);
        (*o).v = 0;
    }
    pub unsafe fn move_assign(&mut self, o: *mut NoCopy) -> *mut NoCopy {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut NoCopy));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
#[placement]
pub struct PrivateCopy {
    pub v: i32,
}
impl PrivateCopy {
    pub unsafe fn new(this: *mut PrivateCopy) {
        (&raw mut (*this).v).write(0);
    }
    pub unsafe fn move_from(this: *mut PrivateCopy, o: *mut PrivateCopy) {
        (&raw mut (*this).v).write((*o).v);
        (*o).v = 0;
    }
    pub unsafe fn move_assign(&mut self, o: *mut PrivateCopy) -> *mut PrivateCopy {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut PrivateCopy));
    }
}
impl Default for PrivateCopy {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: PrivateCopy = ::std::mem::MaybeUninit::uninit().assume_init();
                PrivateCopy::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
#[placement]
pub struct Immovable {
    pub v: i32,
}
impl Immovable {
    pub unsafe fn new(this: *mut Immovable) {
        (&raw mut (*this).v).write(0);
    }
}
impl Default for Immovable {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: Immovable = ::std::mem::MaybeUninit::uninit().assume_init();
                Immovable::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
#[placement]
pub struct Container {
    pub inner: NoCopy,
    pub tag: i32,
}
impl Container {
    pub unsafe fn move_from(this: *mut Container, _a0: *mut Container) {
        NoCopy::move_from((&raw mut (*this).inner), { &mut (*_a0).inner });
        (&raw mut (*this).tag).write((*_a0).tag);
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Container) -> *mut Container {
        (unsafe {
            let _o: *mut NoCopy = &mut (*_a0).inner;
            NoCopy::move_assign(&mut self.inner, _o)
        });
        self.tag = (*_a0).tag;
        return &mut (*(self as *mut Container));
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, Default)]
pub struct VecHolder {
    pub items: Vec<NoCopy>,
}
impl VecHolder {
    pub unsafe fn move_from(_a0: *mut VecHolder) -> Self {
        let mut this = Self {
            items: std::mem::take(&mut (*_a0).items),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut VecHolder) -> *mut VecHolder {
        {
            self.items = std::mem::take(&mut (*_a0).items);
        };
        return &mut (*(self as *mut VecHolder));
    }
}
pub unsafe fn bump_0(mut p: *mut NoCopy) {
    (*p).v.postfix_inc();
}
pub unsafe fn bump_ref_1(r: *mut Immovable) {
    (*r).v.postfix_inc();
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: NoCopy = ::std::mem::MaybeUninit::uninit().assume_init();
    NoCopy::new(&raw mut a, { 1 });
    let mut b: NoCopy = ::std::mem::MaybeUninit::uninit().assume_init();
    NoCopy::move_from(&raw mut b, { &mut a });
    assert!(((b.v) == (1)) && ((a.v) == (0)));
    (unsafe { NoCopy::move_assign(&mut a, &mut b) });
    assert!(((a.v) == (1)) && ((b.v) == (0)));
    (unsafe { bump_0((&mut a as *mut NoCopy)) });
    assert!(((a.v) == (2)));
    let mut p: PrivateCopy = ::std::mem::MaybeUninit::uninit().assume_init();
    PrivateCopy::new(&raw mut p);
    p.v = 3;
    let mut q: PrivateCopy = ::std::mem::MaybeUninit::uninit().assume_init();
    PrivateCopy::move_from(&raw mut q, { &mut p });
    assert!(((q.v) == (3)) && ((p.v) == (0)));
    (unsafe { PrivateCopy::move_assign(&mut p, &mut q) });
    assert!(((p.v) == (3)) && ((q.v) == (0)));
    let mut im: Immovable = ::std::mem::MaybeUninit::uninit().assume_init();
    Immovable::new(&raw mut im);
    im.v = 4;
    (unsafe { bump_ref_1(&mut im) });
    let mut pim: *mut Immovable = (&mut im as *mut Immovable);
    assert!((((*pim).v) == (5)));
    let mut c: Container = ::std::mem::MaybeUninit::uninit().assume_init();
    {
        let __d = &raw mut c;
        NoCopy::new((&raw mut (*__d).inner), { 6 });
        (&raw mut (*__d).tag).write(7);
    };
    let mut d: Container = ::std::mem::MaybeUninit::uninit().assume_init();
    Container::move_from(&raw mut d, { &mut c });
    assert!((((d.inner.v) == (6)) && ((d.tag) == (7))) && ((c.inner.v) == (0)));
    let mut h: VecHolder = <VecHolder>::default();
    let mut h2: VecHolder = VecHolder::move_from({ &mut h });
    assert!(((h2.items.len()) == (0_usize)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
