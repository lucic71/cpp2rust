extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[repr(u32)]
pub enum Flags_enum {
    #[default]
    F_NONE = 0,
    F_A = 1,
    F_B = 2,
    F_AB = 3,
    F_ALL = 7,
}
impl From<i32> for Flags_enum {
    fn from(n: i32) -> Flags_enum {
        match n {
            0 => Flags_enum::F_NONE,
            1 => Flags_enum::F_A,
            2 => Flags_enum::F_B,
            3 => Flags_enum::F_AB,
            7 => Flags_enum::F_ALL,
            _ => panic!("invalid Flags_enum value: {}", n),
        }
    }
}
libcc2rs::impl_enum_inc_dec!(Flags_enum);
impl ByteRepr for Flags_enum {
    fn byte_size() -> usize {
        4
    }
    fn to_bytes(&self, buf: &mut [u8]) {
        (*self as i32).to_bytes(buf);
    }
    fn from_bytes(buf: &[u8]) -> Self {
        <Flags_enum>::from(i32::from_bytes(buf))
    }
}
pub fn add_b_0(f: Flags_enum) -> Flags_enum {
    let f: Value<Flags_enum> = Rc::new(RefCell::new(f));
    {
        let rhs_0 =
            Flags_enum::from(((((*f.borrow()) as u32) | ((Flags_enum::F_B as i32) as u32)) as i32));
        (*f.borrow_mut()) = rhs_0
    };
    return (*f.borrow());
}
pub fn main() {
    libcc2rs::exit_refcount(main_0());
}
fn main_0() -> i32 {
    let f: Value<Flags_enum> = Rc::new(RefCell::new(Flags_enum::F_A));
    {
        let __rhs = ({ add_b_0((*f.borrow())) });
        (*f.borrow_mut()) = __rhs
    };
    assert!((((((*f.borrow()) as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    let g: Value<Flags_enum> = Rc::new(RefCell::new(Flags_enum::F_NONE));
    {
        let rhs_0 =
            Flags_enum::from(((((*g.borrow()) as u32) | ((Flags_enum::F_A as i32) as u32)) as i32));
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum::F_A as i32) as u32)) as i32) != 0));
    {
        let rhs_0 =
            Flags_enum::from(((((*g.borrow()) as u32) | ((Flags_enum::F_B as i32) as u32)) as i32));
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    {
        let rhs_0 = Flags_enum::from(
            ((((*g.borrow()) as u32) & ((Flags_enum::F_ALL as i32) as u32)) as i32),
        );
        (*g.borrow_mut()) = rhs_0
    };
    assert!((((((*g.borrow()) as u32) == ((Flags_enum::F_AB as i32) as u32)) as i32) != 0));
    return 0;
}
