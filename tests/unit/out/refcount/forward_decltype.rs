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
pub fn forward_by_decltype_2(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_by_decltype_3(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_4(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_0((x).clone()) });
}
pub fn forward_abbreviated_5(x: Ptr<Tracked>) -> Overload {
    return ({ chosen_overload_1((x).clone()) });
}
pub fn forward_abbreviated_pack_6(args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) -> i32 {
    return ({ (({ chosen_overload_0((args_0).clone()) }) as i32) } + {
        (({ chosen_overload_1((args_1).clone()) }) as i32)
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(a.as_pointer(), { 3 });
    assert!(
        ((({ forward_by_decltype_2(a.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(({ ({ (*a.borrow()).v } == 3) }));
    assert!(
        ((({
            let _x: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 4 });
                __v
            };
            forward_by_decltype_3(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let b: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(b.as_pointer(), { 5 });
    assert!(
        ((({ forward_abbreviated_4(b.as_pointer(),) }) as i32)
            == (Overload_kLvalueOverload as i32))
    );
    assert!(({ ({ (*b.borrow()).v } == 5) }));
    assert!(
        ((({
            let _x: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 6 });
                __v
            };
            forward_abbreviated_5(_x.as_pointer())
        }) as i32)
            == (Overload_kRvalueOverload as i32))
    );
    let c: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(c.as_pointer(), { 7 });
    assert!(
        (({
            let _args_1: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 8 });
                __v
            };
            forward_abbreviated_pack_6(c.as_pointer(), _args_1.as_pointer())
        }) == ((Overload_kLvalueOverload as i32) + (Overload_kRvalueOverload as i32)))
    );
    assert!(({ ({ (*c.borrow()).v } == 7) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
