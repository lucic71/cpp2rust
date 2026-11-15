extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static sequence_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(4)]
pub struct StampsConstruction {
    #[offset(0)]
    pub stamp: i32,
}
impl StampsConstruction {
    pub fn new() -> Self {
        Self {
            stamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Default for StampsConstruction {
    fn default() -> Self {
        { StampsConstruction::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct VirtualOfLeft {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct VirtualOfRight {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct DirectVirtual {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct NonVirtualFirst {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct Left {
    #[offset(8)]
    pub leftStamp: i32,
}
impl Left {
    pub fn new() -> Self {
        Self {
            leftStamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Default for Left {
    fn default() -> Self {
        { Left::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(24)]
pub struct Right {
    #[offset(8)]
    pub rightStamp: i32,
}
impl Right {
    pub fn new() -> Self {
        Self {
            rightStamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Default for Right {
    fn default() -> Self {
        { Right::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(48)]
pub struct MostDerived {
    #[offset(28)]
    pub ownStamp: i32,
}
impl MostDerived {
    pub fn new() -> Self {
        Self {
            ownStamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Default for MostDerived {
    fn default() -> Self {
        { MostDerived::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let d: Value<MostDerived> = Rc::new(RefCell::new(MostDerived::new()));
    assert!(({ ({ (((*d.borrow()) as VirtualOfLeft) as StampsConstruction).stamp } == 1) }));
    assert!(({ ({ (((*d.borrow()) as VirtualOfRight) as StampsConstruction).stamp } == 2) }));
    assert!(({ ({ (((*d.borrow()) as DirectVirtual) as StampsConstruction).stamp } == 3) }));
    assert!(({ ({ (((*d.borrow()) as NonVirtualFirst) as StampsConstruction).stamp } == 4) }));
    assert!(({ ({ ((*d.borrow()) as Left).leftStamp } == 5) }));
    assert!(({ ({ ((*d.borrow()) as Right).rightStamp } == 6) }));
    assert!(({ ({ (*d.borrow()).ownStamp } == 7) }));
    assert!((sequence_0.with(|rc| *rc.borrow()) == 7));
    sequence_0.with(|rc| *rc.borrow_mut() = 0);
    let r: Value<Right> = Rc::new(RefCell::new(Right::new()));
    assert!(({ ({ (((*r.borrow()) as VirtualOfRight) as StampsConstruction).stamp } == 1) }));
    assert!(({ ({ (((*r.borrow()) as VirtualOfLeft) as StampsConstruction).stamp } == 2) }));
    assert!(({ ({ (*r.borrow()).rightStamp } == 3) }));
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
}
