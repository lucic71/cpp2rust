extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Flags_enum = u32;
pub const Flags_enum_F_NONE: Flags_enum = 0;
pub const Flags_enum_F_A: Flags_enum = 1;
pub const Flags_enum_F_B: Flags_enum = 2;
pub const Flags_enum_F_AB: Flags_enum = 3;
pub const Flags_enum_F_ALL: Flags_enum = 7;
pub fn add_b_0(f: Flags_enum) -> Flags_enum {
    let f: Value<Flags_enum> = Rc::new(RefCell::new(f));
    {
        let rhs_0 = ((((*f.borrow()) as u32) | ((Flags_enum_F_B as i32) as u32)) as Flags_enum);
        (*f.borrow_mut()) = rhs_0
    };
    return (*f.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let f: Value<Flags_enum> = Rc::new(RefCell::new(Flags_enum_F_A));
    {
        let __rhs = ({ add_b_0((*f.borrow())) });
        (*f.borrow_mut()) = __rhs
    };
    assert!((((((*f.borrow()) as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    let g: Value<Flags_enum> = Rc::new(RefCell::new(Flags_enum_F_NONE));
    {
        let rhs_0 = ((((*g.borrow()) as u32) | ((Flags_enum_F_A as i32) as u32)) as Flags_enum);
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum_F_A as i32) as u32)) as i32) != 0));
    {
        let rhs_0 = ((((*g.borrow()) as u32) | ((Flags_enum_F_B as i32) as u32)) as Flags_enum);
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    {
        let rhs_0 = ((((*g.borrow()) as u32) & ((Flags_enum_F_ALL as i32) as u32)) as Flags_enum);
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum_F_AB as i32) as u32)) as i32) != 0));
    return 0;
}
