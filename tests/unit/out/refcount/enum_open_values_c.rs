extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type Flags = u32;
pub const Flags_FLAG_A: Flags = 256;
pub const Flags_FLAG_B: Flags = 512;
pub const Flags_FLAG_A_ALIAS: Flags = 256;
pub fn with_a_0(f: Flags) -> Flags {
    let f: Value<Flags> = Rc::new(RefCell::new(f));
    return (((((*f.borrow()) as u32) | ((Flags_FLAG_A as i32) as u32)) as i32) as Flags);
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let f: Value<Flags> = Rc::new(RefCell::new((('x' as i32) as Flags)));
    {
        let __rhs = ({ with_a_0((*f.borrow())) });
        (*f.borrow_mut()) = __rhs
    };
    {
        let rhs_0 = ((((*f.borrow()) as u32) | ((Flags_FLAG_B as i32) as u32)) as Flags);
        (*f.borrow_mut()) = rhs_0
    };
    assert!(((((((*f.borrow()) as u32) & 255_u32) == (('x' as i32) as u32)) as i32) != 0));
    assert!(
        ((((((*f.borrow()) as u32) & (!255 as u32))
            == (((Flags_FLAG_A as i32) | (Flags_FLAG_B as i32)) as u32)) as i32)
            != 0)
    );
    let zero: Value<Flags> = Rc::new(RefCell::new(((0) as Flags)));
    assert!(((!((*zero.borrow()) != 0) as i32) != 0));
    assert!((((((*zero.borrow()) as u32) != ((*f.borrow()) as u32)) as i32) != 0));
    let as_int: Value<i32> = Rc::new(RefCell::new(((*f.borrow()) as i32)));
    assert!(((((*as_int.borrow()) == ((256 | 512) | ('x' as i32))) as i32) != 0));
    {
        let __rhs = (((((*f.borrow()) as u32) & (!(Flags_FLAG_B as i32) as u32)) as i32) as Flags);
        (*f.borrow_mut()) = __rhs
    };
    assert!((((((*f.borrow()) as u32) == (((256 | ('x' as i32)) as Flags) as u32)) as i32) != 0));
    let seq: Value<Flags> = Rc::new(RefCell::new(Flags_FLAG_A));
    (*seq.borrow_mut()).postfix_inc();
    assert!((((((*seq.borrow()) as u32) == (((257) as Flags) as u32)) as i32) != 0));
    (*seq.borrow_mut()).prefix_dec();
    assert!((((((*seq.borrow()) as u32) == ((Flags_FLAG_A as i32) as u32)) as i32) != 0));
    assert!(((((Flags_FLAG_A_ALIAS as i32) == (Flags_FLAG_A as i32)) as i32) != 0));
    let alias: Value<Flags> = Rc::new(RefCell::new(Flags_FLAG_A_ALIAS));
    assert!((((((*alias.borrow()) as u32) == ((Flags_FLAG_A as i32) as u32)) as i32) != 0));
    assert!(((((*alias.borrow()) as u32) | ((Flags_FLAG_B as i32) as u32)) != 0));
    return 0;
}
