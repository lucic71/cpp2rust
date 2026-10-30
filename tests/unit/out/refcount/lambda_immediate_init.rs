extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn pick_0(x: Option<()>) -> () {
    let x: Value<()> = Rc::new(RefCell::new(x.unwrap_or_else(|| {
        ({
            FnPtr::<fn() -> ()>::new(|| -> () {
                {
                    return 237_();
                }
            })
            .call()
        })
    })));
    return (*x.borrow());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct S {
    #[offset(0)]
    pub j: (),
    #[offset(4)]
    pub k: (),
}
impl Default for S {
    fn default() -> Self {
        {
            S {
                j: 10_(),
                k: (({
                    FnPtr::<fn() -> ()>::new(|| -> () {
                        {
                            return 3_();
                        }
                    })
                    .call()
                }) + 1_()),
            }
        }
    }
}
thread_local!(
    pub static g_1: Value<()> = Rc::new(RefCell::new(
        ({
            FnPtr::<fn() -> ()>::new(|| -> () {
                {
                    let s: Value<()> = Rc::new(RefCell::new(0_()));
                    let mut i: () = 1_();
                    'loop_: while (i <= 4_()) {
                        (*s.borrow_mut()) += i;
                        i.postfix_inc();
                    }
                    return (*s.borrow());
                }
            })
            .call()
        }),
    ));
);
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    unimplemented!();
    unimplemented!();
    let mut s: S = <S>::default();
    unimplemented!();
    unimplemented!();
    let a: Value<()> = Rc::new(RefCell::new(2_()));
    let mut c: () = ({
        lambda!(
            {
                let a: Ptr<()> = a.as_pointer();
            },
            || -> () {
                a.with_mut(|__v| __v.postfix_inc());
                return ((a.read()) * 10_());
            }
        )
        .call()
    });
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub fn __cpp2rust_init_globals() {
    let _ = g_1.with(|_| ());
}
