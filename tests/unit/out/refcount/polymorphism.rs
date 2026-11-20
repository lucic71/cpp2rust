extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait Animal {
    fn bark(&self) -> bool;
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Dog {}
impl Dog {
    pub fn new_1(this: Ptr<Dog>) {}
    pub fn copy_from(this: Ptr<Dog>, _a0: Ptr<Dog>) {}
}
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct Cat {}
impl Cat {
    pub fn new_1(this: Ptr<Cat>) {}
    pub fn copy_from(this: Ptr<Cat>, _a0: Ptr<Cat>) {}
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let dog: Value<Dog> = Rc::new(RefCell::new(<Dog>::uninit()));
    Dog::new_1(dog.as_pointer());
    let mut animal: PtrDyn<dyn Animal> = (dog.as_pointer()).to_dyn::<dyn Animal>(|w| w);
    let mut eat1: bool = ({ (*animal.upgrade().deref()).bark() });
    let cat: Value<Cat> = Rc::new(RefCell::new(<Cat>::uninit()));
    Cat::new_1(cat.as_pointer());
    animal = (cat.as_pointer()).to_dyn::<dyn Animal>(|w| w);
    let mut eat2: bool = ({ (*animal.upgrade().deref()).bark() });
    assert!((eat1) && (!(eat2)));
    return 0;
}
impl Animal for Cat {
    fn bark(&self) -> bool {
        return false;
    }
}
impl Animal for Dog {
    fn bark(&self) -> bool {
        return true;
    }
}
pub trait CatImpl {
    fn meow(&self) -> bool;
}
impl CatImpl for Ptr<Cat> {
    fn meow(&self) -> bool {
        return true;
    }
}
pub fn __cpp2rust_init_globals() {}
