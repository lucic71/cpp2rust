extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor)]
#[placement]
#[byte_size(8)]
pub struct Counted {
    #[offset(0)]
    pub copies: i32,
    #[offset(4)]
    pub moves: i32,
}
impl Counted {
    pub fn new(this: Ptr<Counted>) {
        field!(this, copies).write(0);
        field!(this, moves).write(0);
    }
    pub fn copy_from(this: Ptr<Counted>, o: Ptr<Counted>) {
        field!(this, copies).write((o.with(|__s| __s.copies) + 1));
        field!(this, moves).write(o.with(|__s| __s.moves));
    }
    pub fn move_from(this: Ptr<Counted>, o: Ptr<Counted>) {
        field!(this, copies).write(o.with(|__s| __s.copies));
        field!(this, moves).write((o.with(|__s| __s.moves) + 1));
    }
}
impl Clone for Counted {
    fn clone(&self) -> Self {
        let __src: Value<Counted> = Rc::new(RefCell::new(Counted {
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        let __dst: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
        Counted::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Counted {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
            Counted::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
thread_local!(
    pub static drops_0: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Record, ByteRepr, Uninit, VaArg, FnPtrArg, MoveCtor, Destructor)]
#[placement]
#[byte_size(1)]
pub struct Dropped {}
impl Dropped {
    pub fn new(this: Ptr<Dropped>) {}
    pub fn copy_from(this: Ptr<Dropped>, _a0: Ptr<Dropped>) {}
    pub fn move_from(this: Ptr<Dropped>, _a0: Ptr<Dropped>) {}
}
impl Clone for Dropped {
    fn clone(&self) -> Self {
        let __src: Value<Dropped> = Rc::new(RefCell::new(Dropped {}));
        let __dst: Value<Dropped> = Rc::new(RefCell::new(<Dropped>::uninit()));
        Dropped::copy_from(__dst.as_pointer(), __src.as_pointer());
        Rc::try_unwrap(__dst).ok().unwrap().into_inner()
    }
}
impl Default for Dropped {
    fn default() -> Self {
        Rc::try_unwrap({
            let __v: Value<Dropped> = Rc::new(RefCell::new(<Dropped>::uninit()));
            Dropped::new(__v.as_pointer());
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let c: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
    Counted::new(c.as_pointer());
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let c: Value<Counted> = {
                let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
                Counted::copy_from(__v.as_pointer(), { c.as_pointer() });
                __v
            };
        },
        || -> i32 {
            return (({ (*c.borrow()).copies } * 10) + { (*c.borrow()).moves });
        }
    )));
    assert!(({ (({ (*f.borrow()).call() }) == 10) }));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    assert!(({ (({ (*g.borrow()).call() }) == 20) }));
    assert!(({ (({ (*f.borrow()).call() }) == 10) }));
    let h: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    assert!(({ (({ (*h.borrow()).call() }) == 11) }));
    let mut returned: i32 = {
        Rc::try_unwrap({
            let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
            ({
                lambda!(
                    {
                        let c: Value<Counted> = {
                            let __v: Value<Counted> = Rc::new(RefCell::new(<Counted>::uninit()));
                            Counted::copy_from(__v.as_pointer(), { c.as_pointer() });
                            __v
                        };
                    },
                    |__ret: Ptr<Counted>| {
                        Counted::copy_from(__ret.clone(), { c.as_pointer() });
                        return;
                    }
                )
                .call(__v.as_pointer())
            });
            __v
        })
        .ok()
        .unwrap()
        .into_inner()
        .copies
    };
    assert!((returned == 2));
    let arr: Value<Box<[Counted]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <Counted>::uninit()).collect::<Box<[_]>>(),
    ));
    {
        let __d = arr.decay();
        for __i in 0..2_isize {
            Counted::new(__d.offset(__i));
        }
    };
    let a: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let arr: Value<Box<[Counted]>> = {
                let __v: Value<Box<[Counted]>> = Rc::new(RefCell::new(
                    (0..2).map(|_| <Counted>::uninit()).collect::<Box<[_]>>(),
                ));
                {
                    let __d = __v.decay();
                    for __i in 0..2_usize {
                        Counted::copy_from(__d.offset(__i as isize), { arr.decay().offset(__i) });
                    }
                };
                __v
            };
        },
        || -> i32 {
            return ({ (*arr.borrow())[(0) as usize].copies } + {
                (*arr.borrow())[(1) as usize].copies
            });
        }
    )));
    assert!(({ (({ (*a.borrow()).call() }) == 2) }));
    let a2: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*a.borrow()).copy_from()));
    assert!(({ (({ (*a2.borrow()).call() }) == 4) }));
    let a3: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*a.borrow()).move_from()));
    assert!(({ (({ (*a3.borrow()).call() }) == 2) }));
    {
        let m: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = {
                    let __v: Value<Dropped> = Rc::new(RefCell::new(<Dropped>::uninit()));
                    Dropped::new(__v.as_pointer());
                    __v
                };
            },
            || {}
        )));
        let _dtor_m =
            std::mem::DropGuard::new(m.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
        let m2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*m.borrow()).move_from()));
        let _dtor_m2 =
            std::mem::DropGuard::new(m2.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    }
    assert!((drops_0.with(|rc| *rc.borrow()) == 2));
    {
        let k: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = {
                    let __v: Value<Dropped> = Rc::new(RefCell::new(<Dropped>::uninit()));
                    Dropped::new(__v.as_pointer());
                    __v
                };
            },
            || {}
        )));
        let _dtor_k =
            std::mem::DropGuard::new(k.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
        let k2: Value<FnPtr<fn()>> = Rc::new(RefCell::new((*k.borrow()).copy_from()));
        let _dtor_k2 =
            std::mem::DropGuard::new(k2.clone(), |__p| __p.as_pointer().with(|__f| __f.destroy()));
    }
    assert!((drops_0.with(|rc| *rc.borrow()) == 4));
    {
        let inner: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let d: Value<Dropped> = {
                    let __v: Value<Dropped> = Rc::new(RefCell::new(<Dropped>::uninit()));
                    Dropped::new(__v.as_pointer());
                    __v
                };
            },
            || {}
        )));
        let _dtor_inner = std::mem::DropGuard::new(inner.clone(), |__p| {
            __p.as_pointer().with(|__f| __f.destroy())
        });
        let outer: Value<FnPtr<fn()>> = Rc::new(RefCell::new(lambda!(
            {
                let inner: Value<FnPtr<fn()>> =
                    Rc::new(RefCell::new((*inner.borrow()).copy_from()));
            },
            || {}
        )));
        let _dtor_outer = std::mem::DropGuard::new(outer.clone(), |__p| {
            __p.as_pointer().with(|__f| __f.destroy())
        });
    }
    assert!((drops_0.with(|rc| *rc.borrow()) == 6));
    return 0;
}
pub trait DroppedImpl {
    fn destructor(&self);
}
impl DroppedImpl for Ptr<Dropped> {
    fn destructor(&self) {
        (*drops_0.with(Value::clone).borrow_mut()).postfix_inc();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = drops_0.with(|_| ());
}
