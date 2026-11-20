extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Overload = u32;
pub const Overload_kLvalueOverload: Overload = 1;
pub const Overload_kRvalueOverload: Overload = 2;
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(i32, new, value)]
#[byte_size(12)]
pub struct Tracked {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl Tracked {
    pub fn new(this: Ptr<Tracked>, mut v: i32) {
        field!(this, v).write(v);
        field!(this, copies).write(0);
        field!(this, moves).write(0);
    }
    pub fn copy_from(this: Ptr<Tracked>, o: Ptr<Tracked>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write((o.with(|__s| __s.copies) + 1));
        field!(this, moves).write(o.with(|__s| __s.moves));
    }
    pub fn move_from(this: Ptr<Tracked>, o: Ptr<Tracked>) {
        field!(this, v).write(o.with(|__s| __s.v));
        field!(this, copies).write(o.with(|__s| __s.copies));
        field!(this, moves).write((o.with(|__s| __s.moves) + 1));
        field!(o, v).write(0);
    }
}
impl Clone for Tracked {
    fn clone(&self) -> Self {
        let __src: Value<Tracked> = Rc::new(RefCell::new(Tracked {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        let __dst: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
        Tracked::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
pub fn chosen_overload_0(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kLvalueOverload;
}
pub fn chosen_overload_1(_a0: Ptr<Tracked>) -> Overload {
    return Overload_kRvalueOverload;
}
impl Holder {
    pub fn new_1(this: Ptr<Holder>, x: Ptr<Tracked>) {
        Tracked::copy_from(field_ptr!(this, t), { (x).clone() });
    }
}
impl Holder {
    pub fn new_2(this: Ptr<Holder>, x: Ptr<Tracked>) {
        Tracked::move_from(field_ptr!(this, t), { (x).clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, CppFrom, Default)]
#[placement]
#[cpp_from(Tracked, new_1, mut_ptr)]
#[cpp_from_move(Tracked, new_2, ptr)]
#[byte_size(12)]
pub struct Holder {
    #[offset(0)]
    #[byte_size(12)]
    pub t: Tracked,
}
impl Holder {
    pub fn copy_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Tracked::copy_from(field_ptr!(this, t), { field_ptr!(_a0, t) });
    }
    pub fn move_from(this: Ptr<Holder>, _a0: Ptr<Holder>) {
        Tracked::move_from(field_ptr!(this, t), { field_ptr!(_a0, t) });
    }
}
pub fn forward_once_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_once_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_twice_4(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_2((x).clone()) });
}
pub fn forward_twice_5(x: Ptr<Tracked>) -> Overload {
    return ({ forward_once_3((x).clone()) });
}
pub fn forward_into_ctor_6(__ret: Ptr<Holder>, x: Ptr<Tracked>) {
    Holder::new_1(__ret.clone(), { (x).clone() });
    return;
}
pub fn forward_into_ctor_7(__ret: Ptr<Holder>, x: Ptr<Tracked>) {
    Holder::new_2(__ret.clone(), { (x).clone() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let lvalue: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(lvalue.as_pointer(), { 7 });
    assert!(
        ((({ forward_once_2(lvalue.as_pointer(),) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(({ ({ (*lvalue.borrow()).v } == 7) }));
    assert!(
        ((({
            let _x: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 8 });
                __v
            };
            forward_once_3(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let relayed: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(relayed.as_pointer(), { 9 });
    assert!(
        ((({ forward_twice_4(relayed.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(({ ({ (*relayed.borrow()).v } == 9) }));
    assert!(
        ((({
            let _x: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 10 });
                __v
            };
            forward_twice_5(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let kept: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(kept.as_pointer(), { 11 });
    let from_lvalue: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    ({ forward_into_ctor_6(from_lvalue.as_pointer(), kept.as_pointer()) });
    assert!(({ ({ (*from_lvalue.borrow()).t.v } == 11) }));
    assert!(({ ({ (*from_lvalue.borrow()).t.copies } == 1) }));
    assert!(({ ({ (*from_lvalue.borrow()).t.moves } == 0) }));
    assert!(({ ({ (*kept.borrow()).v } == 11) }));
    let from_rvalue: Value<Holder> = Rc::new(RefCell::new(<Holder>::uninit()));
    ({
        let _x: Value<Tracked> = {
            let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
            Tracked::new(__v.as_pointer(), { 12 });
            __v
        };
        forward_into_ctor_7(from_rvalue.as_pointer(), _x.as_pointer())
    });
    assert!(({ ({ (*from_rvalue.borrow()).t.v } == 12) }));
    assert!(({ ({ (*from_rvalue.borrow()).t.copies } == 0) }));
    assert!(({ ({ (*from_rvalue.borrow()).t.moves } == 1) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
