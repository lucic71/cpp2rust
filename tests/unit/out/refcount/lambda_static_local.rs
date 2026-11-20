extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static a_0: Value<i32> = Rc::new(RefCell::new(0_i32));
);
pub fn main() {
    __cpp2rust_init_globals();
    libcc2rs::cpp2rust_exit(main_0());
}
fn main_0() -> i32 {
    let next: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return (*(*n_1.with(Value::clone).borrow_mut()).prefix_inc());
            }
        })));
    assert!(({ (({ (*next.borrow()).call() }) == 1) }));
    assert!(({ (({ (*next.borrow()).call() }) == 2) }));
    let copy: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*next.borrow()).copy_from()));
    assert!(({ (({ (*copy.borrow()).call() }) == 3) }));
    assert!(({ (({ (*next.borrow()).call() }) == 4) }));
    let fresh: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                return (*(*n_1.with(Value::clone).borrow_mut()).prefix_inc());
            }
        })));
    assert!(({ (({ (*fresh.borrow()).call() }) == 5) }));
    assert!(({ (({ (*next.borrow()).call() }) == 6) }));
    let per_type: Value<FnPtr<Generic>> = Rc::new(RefCell::new(lambda!(
        Generic,
        {},
        |mut x: i32| -> i32 {
            (*calls_2.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_2.with(|rc| *rc.borrow());
        },
        |mut x: f64| -> i32 {
            (*calls_3.with(Value::clone).borrow_mut()).postfix_inc();
            return calls_3.with(|rc| *rc.borrow());
        }
    )));
    assert!(({ (({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(1,) }) == 1) }));
    assert!(({ (({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(2,) }) == 2) }));
    assert!(({ (({ (*per_type.borrow()).spec::<fn(f64) -> i32>(1).call(1_f64,) }) == 1) }));
    assert!(({ (({ (*per_type.borrow()).spec::<fn(i32) -> i32>(0).call(3,) }) == 3) }));
    let read: Value<FnPtr<fn(i32) -> i32>> = Rc::new(RefCell::new(FnPtr::<fn(i32) -> i32>::new(
        |mut _a0: i32| -> i32 {
            {
                return a_0.with(|rc| *rc.borrow());
            }
        },
    )));
    assert!(({ (({ (*read.borrow()).call(0,) }) == 0) }));
    a_0.with(|rc| *rc.borrow_mut() = 1);
    assert!(({ (({ (*read.borrow()).call(0,) }) == 1) }));
    let local: Value<FnPtr<fn() -> i32>> =
        Rc::new(RefCell::new(FnPtr::<fn() -> i32>::new(|| -> i32 {
            {
                let p: Value<P_4> = Rc::new(RefCell::new(<P_4>::uninit()));
                {
                    let __d = p.as_pointer();
                    field!(__d, x).write(2);
                    field!(__d, y).write(3);
                };
                return ({ (*p.borrow()).x } * { (*p.borrow()).y });
            }
        })));
    assert!(({ (({ (*local.borrow()).call() }) == 6) }));
    return 0;
}
thread_local!(
    static n_1: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static calls_2: Value<i32> = Rc::new(RefCell::new(0));
);
thread_local!(
    static calls_3: Value<i32> = Rc::new(RefCell::new(0));
);
#[derive(Copy, Clone, Record, ByteRepr, Uninit, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct P_4 {
    #[offset(0)]
    pub x: i32,
    #[offset(4)]
    pub y: i32,
}
impl P_4 {
    pub fn copy_from(this: Ptr<P_4>, _a0: Ptr<P_4>) {
        field!(this, x).write(_a0.with(|__s| __s.x));
        field!(this, y).write(_a0.with(|__s| __s.y));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = a_0.with(|_| ());
}
