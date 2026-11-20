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
pub const Overload_kIntLvalueOverload: Overload = 3;
pub const Overload_kIntRvalueOverload: Overload = 4;
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
pub fn chosen_overload_2(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntLvalueOverload;
}
pub fn chosen_overload_3(_a0: Ptr<i32>) -> Overload {
    return Overload_kIntRvalueOverload;
}
pub fn forward_pack_4() -> i32 {
    let mut digits: i32 = 0;
    ();
    return digits;
}
pub fn forward_pack_5(args: Ptr<Tracked>) -> i32 {
    let mut digits: i32 = 0;
    let __rhs = ({ (digits * 10) } + { (({ chosen_overload_0((args).clone()) }) as i32) });
    digits = __rhs;
    return digits;
}
pub fn forward_pack_6(args: Ptr<Tracked>) -> i32 {
    let mut digits: i32 = 0;
    let __rhs = ({ (digits * 10) } + { (({ chosen_overload_1((args).clone()) }) as i32) });
    digits = __rhs;
    return digits;
}
pub fn forward_pack_7(
    args_0: Ptr<Tracked>,
    args_1: Ptr<Tracked>,
    args_2: Ptr<i32>,
    args_3: Ptr<i32>,
) -> i32 {
    let mut digits: i32 = 0;
    {
        let __rhs = ({ (digits * 10) } + { (({ chosen_overload_0((args_0).clone()) }) as i32) });
        digits = __rhs;
        {
            let __rhs =
                ({ (digits * 10) } + { (({ chosen_overload_1((args_1).clone()) }) as i32) });
            digits = __rhs;
            {
                let __rhs =
                    ({ (digits * 10) } + { (({ chosen_overload_2((args_2).clone()) }) as i32) });
                digits = __rhs;
                let __rhs =
                    ({ (digits * 10) } + { (({ chosen_overload_3((args_3).clone()) }) as i32) });
                digits = __rhs
            }
        }
    };
    return digits;
}
impl Pair {
    pub fn new(this: Ptr<Pair>, x: Ptr<Tracked>, y: Ptr<Tracked>) {
        Tracked::copy_from(field_ptr!(this, a), { (x).clone() });
        Tracked::move_from(field_ptr!(this, b), { (y).clone() });
    }
}
#[derive(Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Default)]
#[placement]
#[byte_size(24)]
pub struct Pair {
    #[offset(0)]
    #[byte_size(12)]
    pub a: Tracked,
    #[offset(12)]
    #[byte_size(12)]
    pub b: Tracked,
}
impl Pair {
    pub fn copy_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        Tracked::copy_from(field_ptr!(this, a), { field_ptr!(_a0, a) });
        Tracked::copy_from(field_ptr!(this, b), { field_ptr!(_a0, b) });
    }
    pub fn move_from(this: Ptr<Pair>, _a0: Ptr<Pair>) {
        Tracked::move_from(field_ptr!(this, a), { field_ptr!(_a0, a) });
        Tracked::move_from(field_ptr!(this, b), { field_ptr!(_a0, b) });
    }
}
pub fn forward_pack_into_ctor_8(__ret: Ptr<Pair>, args_0: Ptr<Tracked>, args_1: Ptr<Tracked>) {
    Pair::new(__ret.clone(), { (args_0).clone() }, { (args_1).clone() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    assert!((({ forward_pack_4() }) == 0));
    let a: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(a.as_pointer(), { 1 });
    assert!((({ forward_pack_5(a.as_pointer(),) }) == (Overload_kLvalueOverload as i32)));
    assert!(({ ({ (*a.borrow()).v } == 1) }));
    assert!(
        (({
            let _args: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 2 });
                __v
            };
            forward_pack_6(_args.as_pointer())
        }) == (Overload_kRvalueOverload as i32))
    );
    let i: Value<i32> = Rc::new(RefCell::new(3));
    assert!(
        (({
            let _args_1: Value<Tracked> = {
                let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
                Tracked::new(__v.as_pointer(), { 4 });
                __v
            };
            let _args_3: Value<i32> = Rc::new(RefCell::new(5));
            forward_pack_7(
                a.as_pointer(),
                _args_1.as_pointer(),
                i.as_pointer(),
                _args_3.as_pointer(),
            )
        }) == 1234)
    );
    assert!(({ ({ (*a.borrow()).v } == 1) }));
    assert!(({ ((*i.borrow()) == 3) }));
    let lhs: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(lhs.as_pointer(), { 6 });
    let p: Value<Pair> = Rc::new(RefCell::new(<Pair>::uninit()));
    ({
        let _args_1: Value<Tracked> = {
            let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
            Tracked::new(__v.as_pointer(), { 7 });
            __v
        };
        forward_pack_into_ctor_8(p.as_pointer(), lhs.as_pointer(), _args_1.as_pointer())
    });
    assert!(({ ({ (*p.borrow()).a.v } == 6) }));
    assert!(({ ({ (*p.borrow()).a.copies } == 1) }));
    assert!(({ ({ (*p.borrow()).b.v } == 7) }));
    assert!(({ ({ (*p.borrow()).b.copies } == 0) }));
    assert!(({ ({ (*p.borrow()).b.moves } == 1) }));
    assert!(({ ({ (*lhs.borrow()).v } == 6) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
