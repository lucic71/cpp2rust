extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let zero: Value<FnPtr<fn() -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> ()>::new(|| -> () {
            {
                return 42_();
            }
        })));
    unimplemented!();
    let one: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return (x + 1_());
            }
        },
    )));
    unimplemented!();
    let three: Value<FnPtr<fn((), (), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), (), ()) -> ()>::new(
            |mut x: (), mut y: (), mut z: ()| -> () {
                {
                    return (((x * 100_()) + (y * 10_())) + z);
                }
            },
        )));
    unimplemented!();
    let mut k: () = 3_();
    let mut m: () = 4_();
    let constants: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return ((x + 3_()) + 4_());
            }
        },
    )));
    unimplemented!();
    let mut n: () = (k + m);
    let derived: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return (x + (3_() + 4_()));
            }
        },
    )));
    unimplemented!();
    let implicit: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
        |mut x: ()| -> () {
            {
                return (x + 3_());
            }
        },
    )));
    unimplemented!();
    let bump: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(|x: ()| -> () {
            {
                let x: Value<()> = Rc::new(RefCell::new(x));
                (*x.borrow_mut()) += 1_();
                return (*x.borrow());
            }
        })));
    unimplemented!();
    let through_ptr: Value<FnPtr<fn(()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(|x: ()| -> () {
            {
                let x: Value<()> = Rc::new(RefCell::new(x));
                let mut p: Ptr<()> = (x.as_pointer());
                p.write({ ((p.read()) * 2_()) });
                return (*x.borrow());
            }
        })));
    unimplemented!();
    let mut seed: () = 5_();
    let boxed: Value<()> = Rc::new(RefCell::new(({ (*through_ptr.borrow()).call(seed) })));
    let mut boxed_ptr: Ptr<()> = (boxed.as_pointer());
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
