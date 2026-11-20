extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct MoveOnly {
    pub v: i32,
}
impl MoveOnly {
    pub unsafe fn new(this: *mut MoveOnly, mut v: i32) {
        (&raw mut (*this).v).write(v);
    }
    pub unsafe fn move_from(this: *mut MoveOnly, o: *mut MoveOnly) {
        (&raw mut (*this).v).write((*o).v);
        (*o).v = 0;
    }
}
#[repr(C)]
#[derive(FnPtrArg)]
pub struct ConstMove {
    pub mark: i32,
}
impl ConstMove {
    pub unsafe fn new(this: *mut ConstMove) {
        (&raw mut (*this).mark).write(0);
    }
    pub unsafe fn new_1(this: *mut ConstMove, o: *mut ConstMove) {
        (&raw mut (*this).mark).write((((*o).mark) + (1)));
    }
    pub unsafe fn new_2(this: *mut ConstMove, o: *const ConstMove) {
        (&raw mut (*this).mark).write((((*o).mark) + (10)));
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        unsafe {
            {
                let mut __v: ConstMove = ::std::mem::MaybeUninit::uninit().assume_init();
                ConstMove::new(&raw mut __v);
                __v
            }
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct ThrowingMove {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl ThrowingMove {
    pub unsafe fn new(this: *mut ThrowingMove, mut v: i32) {
        (&raw mut (*this).v).write(v);
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn copy_from(this: *mut ThrowingMove, o: *const ThrowingMove) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((((*o).copies) + (1)));
        (&raw mut (*this).moves).write((*o).moves);
    }
    pub unsafe fn move_from(this: *mut ThrowingMove, o: *mut ThrowingMove) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((*o).copies);
        (&raw mut (*this).moves).write((((*o).moves) + (1)));
        (*o).v = 0;
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: ThrowingMove = ::std::mem::MaybeUninit::uninit().assume_init();
            ThrowingMove::copy_from(&raw mut __v, self as *const ThrowingMove);
            __v
        }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NoexceptMove {
    pub v: i32,
    pub copies: i32,
    pub moves: i32,
}
impl NoexceptMove {
    pub unsafe fn new(this: *mut NoexceptMove, mut v: i32) {
        (&raw mut (*this).v).write(v);
        (&raw mut (*this).copies).write(0);
        (&raw mut (*this).moves).write(0);
    }
    pub unsafe fn copy_from(this: *mut NoexceptMove, o: *const NoexceptMove) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((((*o).copies) + (1)));
        (&raw mut (*this).moves).write((*o).moves);
    }
    pub unsafe fn move_from(this: *mut NoexceptMove, o: *mut NoexceptMove) {
        (&raw mut (*this).v).write((*o).v);
        (&raw mut (*this).copies).write((*o).copies);
        (&raw mut (*this).moves).write((((*o).moves) + (1)));
        (*o).v = 0;
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        unsafe {
            let mut __v: NoexceptMove = ::std::mem::MaybeUninit::uninit().assume_init();
            NoexceptMove::copy_from(&raw mut __v, self as *const NoexceptMove);
            __v
        }
    }
}
pub unsafe fn by_value_0(m: *mut MoveOnly) -> i32 {
    return (*m).v;
}
pub unsafe fn make_1(__ret: *mut MoveOnly, mut v: i32) {
    let mut m: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut m, { v });
    MoveOnly::move_from(__ret.clone(), { &mut m });
    return;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        libcc2rs::cpp2rust_exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut a, { 1 });
    let mut b: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::move_from(&raw mut b, { &mut a });
    assert!(((b.v) == (1)));
    assert!(((a.v) == (0)));
    let mut c: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::move_from(&raw mut c, { &mut b });
    assert!(((c.v) == (1)));
    assert!(((b.v) == (0)));
    let mut d: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::move_from(&raw mut d, { &mut c });
    assert!(((d.v) == (1)));
    assert!(((c.v) == (0)));
    let mut e: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    (unsafe { make_1(&raw mut e, 5) });
    assert!(((e.v) == (5)));
    assert!(
        ((unsafe {
            by_value_0((|__p: *mut MoveOnly| {
                MoveOnly::new(__p, { 6 });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<MoveOnly>::uninit()).as_mut_ptr(),
            ))
        }) == (6))
    );
    assert!(
        ((unsafe {
            by_value_0((|__p: *mut MoveOnly| {
                MoveOnly::move_from(__p, { &mut e });
                __p
            })(
                (&mut ::std::mem::MaybeUninit::<MoveOnly>::uninit()).as_mut_ptr(),
            ))
        }) == (5))
    );
    assert!(((e.v) == (0)));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    {
        let __a1 = {
            let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
            MoveOnly::new(&raw mut __v, { 7 });
            __v
        };
        vec_.push(__a1)
    };
    let mut f: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut f, { 8 });
    {
        let __a1 = {
            let mut __v: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
            MoveOnly::move_from(&raw mut __v, { &mut f });
            __v
        };
        vec_.push(__a1)
    };
    assert!(((vec_[(0_usize)].v) == (7)) && ((vec_[(1_usize)].v) == (8)));
    assert!(((f.v) == (0)));
    let mut m: ConstMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMove::new(&raw mut m);
    let mut m1: ConstMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMove::new_1(&raw mut m1, { &mut m });
    let mut cm: ConstMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMove::new(&raw mut cm);
    let mut m2: ConstMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ConstMove::new_2(&raw mut m2, { &cm });
    assert!(((m1.mark) == (1)));
    assert!(((m2.mark) == (10)));
    let mut t: ThrowingMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ThrowingMove::new(&raw mut t, { 1 });
    let mut t1: ThrowingMove = ::std::mem::MaybeUninit::uninit().assume_init();
    ThrowingMove::copy_from(&raw mut t1, { &t });
    assert!(((t1.v) == (1)));
    assert!(((t1.copies) == (1)));
    assert!(((t1.moves) == (0)));
    assert!(((t.v) == (1)));
    let mut n: NoexceptMove = ::std::mem::MaybeUninit::uninit().assume_init();
    NoexceptMove::new(&raw mut n, { 2 });
    let mut n1: NoexceptMove = ::std::mem::MaybeUninit::uninit().assume_init();
    NoexceptMove::move_from(&raw mut n1, { &mut n });
    assert!(((n1.v) == (2)));
    assert!(((n1.copies) == (0)));
    assert!(((n1.moves) == (1)));
    assert!(((n.v) == (0)));
    let mut g: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::new(&raw mut g, { 3 });
    let mut g1: MoveOnly = ::std::mem::MaybeUninit::uninit().assume_init();
    MoveOnly::move_from(&raw mut g1, { &mut g });
    assert!(((g1.v) == (3)));
    assert!(((g.v) == (0)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
