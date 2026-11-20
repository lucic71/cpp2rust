extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(this: Ptr<MoveOnly>, mut v: i32) {
        field!(this, v).write(v);
    }
    pub fn move_from(this: Ptr<MoveOnly>, o: Ptr<MoveOnly>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(o, v).write(0);
    }
}
#[derive(Record, ByteRepr, Uninit, FnPtrArg)]
#[placement]
#[byte_size(4)]
pub struct ConstMove {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMove {
    pub fn new(this: Ptr<ConstMove>) {
        field!(this, mark).write(0);
    }
    pub fn new_1(this: Ptr<ConstMove>, o: Ptr<ConstMove>) {
        field!(this, mark).write((o.with(|__s| __s.mark) + 1));
    }
    pub fn new_2(this: Ptr<ConstMove>, o: Ptr<ConstMove>) {
        field!(this, mark).write((o.with(|__s| __s.mark) + 10));
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<ConstMove> = Rc::new(RefCell::new(<ConstMove>::uninit()));
            ConstMove::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(12)]
pub struct ThrowingMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl ThrowingMove {
    pub fn new(this: Ptr<ThrowingMove>, mut v: i32) {
        field!(this, v).write(v);
        field!(this, copies).write(0);
        field!(this, moves).write(0);
    }
    pub fn copy_from(this: Ptr<ThrowingMove>, o: Ptr<ThrowingMove>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write((o.with(|__s| __s.copies) + 1));
        field!(this, moves).write(o.with(|__s| __s.moves));
    }
    pub fn move_from(this: Ptr<ThrowingMove>, o: Ptr<ThrowingMove>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write(o.with(|__s| __s.copies));
        field!(this, moves).write((o.with(|__s| __s.moves) + 1));
        field!(o, v).write(0);
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        let __dst: Value<ThrowingMove> = Rc::new(RefCell::new(<ThrowingMove>::uninit()));
        ThrowingMove::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(12)]
pub struct NoexceptMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl NoexceptMove {
    pub fn new(this: Ptr<NoexceptMove>, mut v: i32) {
        field!(this, v).write(v);
        field!(this, copies).write(0);
        field!(this, moves).write(0);
    }
    pub fn copy_from(this: Ptr<NoexceptMove>, o: Ptr<NoexceptMove>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write((o.with(|__s| __s.copies) + 1));
        field!(this, moves).write(o.with(|__s| __s.moves));
    }
    pub fn move_from(this: Ptr<NoexceptMove>, o: Ptr<NoexceptMove>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write(o.with(|__s| __s.copies));
        field!(this, moves).write((o.with(|__s| __s.moves) + 1));
        field!(o, v).write(0);
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        let __dst: Value<NoexceptMove> = Rc::new(RefCell::new(<NoexceptMove>::uninit()));
        NoexceptMove::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn by_value_0(m: Value<MoveOnly>) -> i32 {
    return { (*m.borrow()).v };
}
pub fn make_1(__ret: Ptr<MoveOnly>, mut v: i32) {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(m.as_pointer(), { v });
    MoveOnly::move_from(__ret.clone(), { m.as_pointer() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(a.as_pointer(), { 1 });
    let b: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::move_from(b.as_pointer(), { a.as_pointer() });
    assert!(({ ({ (*b.borrow()).v } == 1) }));
    assert!(({ ({ (*a.borrow()).v } == 0) }));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::move_from(c.as_pointer(), { b.as_pointer() });
    assert!(({ ({ (*c.borrow()).v } == 1) }));
    assert!(({ ({ (*b.borrow()).v } == 0) }));
    let d: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::move_from(d.as_pointer(), { c.as_pointer() });
    assert!(({ ({ (*d.borrow()).v } == 1) }));
    assert!(({ ({ (*c.borrow()).v } == 0) }));
    let e: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    ({ make_1(e.as_pointer(), 5) });
    assert!(({ ({ (*e.borrow()).v } == 5) }));
    assert!(
        (({
            by_value_0({
                let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
                MoveOnly::new(__v.as_pointer(), { 6 });
                __v
            })
        }) == 6)
    );
    assert!(
        (({
            by_value_0({
                let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
                MoveOnly::move_from(__v.as_pointer(), { e.as_pointer() });
                __v
            })
        }) == 5)
    );
    assert!(({ ({ (*e.borrow()).v } == 0) }));
    let mut vec_: Vec<MoveOnly> = Vec::new();
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::new(__v.as_pointer(), { 7 });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        vec_.push(__a1)
    };
    let f: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(f.as_pointer(), { 8 });
    {
        let __a1 = Rc::try_unwrap({
            let __v: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
            MoveOnly::move_from(__v.as_pointer(), { f.as_pointer() });
            __v
        })
        .ok()
        .unwrap()
        .into_inner();
        vec_.push(__a1)
    };
    assert!((vec_[0_usize].v == 7) && (vec_[1_usize].v == 8));
    assert!(({ ({ (*f.borrow()).v } == 0) }));
    let m: Value<ConstMove> = Rc::new(RefCell::new(<ConstMove>::uninit()));
    ConstMove::new(m.as_pointer());
    let m1: Value<ConstMove> = Rc::new(RefCell::new(<ConstMove>::uninit()));
    ConstMove::new_1(m1.as_pointer(), { m.as_pointer() });
    let cm: Value<ConstMove> = Rc::new(RefCell::new(<ConstMove>::uninit()));
    ConstMove::new(cm.as_pointer());
    let m2: Value<ConstMove> = Rc::new(RefCell::new(<ConstMove>::uninit()));
    ConstMove::new_2(m2.as_pointer(), { cm.as_pointer() });
    assert!(({ ({ (*m1.borrow()).mark } == 1) }));
    assert!(({ ({ (*m2.borrow()).mark } == 10) }));
    let t: Value<ThrowingMove> = Rc::new(RefCell::new(<ThrowingMove>::uninit()));
    ThrowingMove::new(t.as_pointer(), { 1 });
    let t1: Value<ThrowingMove> = Rc::new(RefCell::new(<ThrowingMove>::uninit()));
    ThrowingMove::copy_from(t1.as_pointer(), { t.as_pointer() });
    assert!(({ ({ (*t1.borrow()).v } == 1) }));
    assert!(({ ({ (*t1.borrow()).copies } == 1) }));
    assert!(({ ({ (*t1.borrow()).moves } == 0) }));
    assert!(({ ({ (*t.borrow()).v } == 1) }));
    let n: Value<NoexceptMove> = Rc::new(RefCell::new(<NoexceptMove>::uninit()));
    NoexceptMove::new(n.as_pointer(), { 2 });
    let n1: Value<NoexceptMove> = Rc::new(RefCell::new(<NoexceptMove>::uninit()));
    NoexceptMove::move_from(n1.as_pointer(), { n.as_pointer() });
    assert!(({ ({ (*n1.borrow()).v } == 2) }));
    assert!(({ ({ (*n1.borrow()).copies } == 0) }));
    assert!(({ ({ (*n1.borrow()).moves } == 1) }));
    assert!(({ ({ (*n.borrow()).v } == 0) }));
    let g: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::new(g.as_pointer(), { 3 });
    let g1: Value<MoveOnly> = Rc::new(RefCell::new(<MoveOnly>::uninit()));
    MoveOnly::move_from(g1.as_pointer(), { g.as_pointer() });
    assert!(({ ({ (*g1.borrow()).v } == 3) }));
    assert!(({ ({ (*g.borrow()).v } == 0) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
