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
pub fn copy_or_move_into_param_4(__ret: Ptr<Tracked>, t: Value<Tracked>) {
    Tracked::move_from(__ret.clone(), { t.as_pointer() });
    return;
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(a.as_pointer(), { 5 });
    assert!(
        ((({ chosen_overload_0(a.as_pointer(),) }) as i32) == (Overload_kLvalueOverload as i32))
    );
    assert!(({ ({ (*a.borrow()).v } == 5) }));
    assert!(
        ((({ chosen_overload_1(a.as_pointer(),) }) as i32) == (Overload_kRvalueOverload as i32))
    );
    assert!(({ ({ (*a.borrow()).v } == 5) }));
    let b: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(b.as_pointer(), { 6 });
    let moved: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    ({
        copy_or_move_into_param_4(moved.as_pointer(), {
            let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
            Tracked::move_from(__v.as_pointer(), { b.as_pointer() });
            __v
        })
    });
    assert!(({ ({ (*moved.borrow()).v } == 6) }));
    assert!(({ ({ (*moved.borrow()).copies } == 0) }));
    assert!(({ ({ (*moved.borrow()).moves } == 2) }));
    assert!(({ ({ (*b.borrow()).v } == 0) }));
    let c: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    Tracked::new(c.as_pointer(), { 7 });
    let copied: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
    ({
        copy_or_move_into_param_4(copied.as_pointer(), {
            let __v: Value<Tracked> = Rc::new(RefCell::new(<Tracked>::uninit()));
            Tracked::copy_from(__v.as_pointer(), { c.as_pointer() });
            __v
        })
    });
    assert!(({ ({ (*copied.borrow()).v } == 7) }));
    assert!(({ ({ (*copied.borrow()).copies } == 1) }));
    assert!(({ ({ (*copied.borrow()).moves } == 1) }));
    assert!(({ ({ (*c.borrow()).v } == 7) }));
    let i: Value<i32> = Rc::new(RefCell::new(8));
    assert!(
        ((({ chosen_overload_2(i.as_pointer(),) }) as i32) == (Overload_kIntLvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(i.as_pointer(),) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(
        ((({ chosen_overload_3(i.as_pointer(),) }) as i32) == (Overload_kIntRvalueOverload as i32))
    );
    assert!(({ ((*i.borrow()) == 8) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
