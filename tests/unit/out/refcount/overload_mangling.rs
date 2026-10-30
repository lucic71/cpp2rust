extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn inc_0(mut p: Ptr<()>) {
    {
        p.with_mut(|__v| *__v = *__v + 1_())
    };
}
pub fn add_1(mut p: Ptr<()>, mut n: ()) {
    {
        let __rhs = { n };
        p.with_mut(|__v| *__v = *__v + __rhs)
    };
}
pub fn twice_2(mut n: ()) -> () {
    return (n * 2_());
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct Access_S_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct S {
    #[offset(0)]
    pub base: (),
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(4)]
pub struct Box_ {
    #[offset(0)]
    pub v: (),
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> () {
    let s: Value<S> = Rc::new(RefCell::new(S { base: 100_() }));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let y: Value<()> = Rc::new(RefCell::new(1_()));
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let c: Value<()> = Rc::new(RefCell::new(3_()));
    unimplemented!();
    let z: Value<()> = Rc::new(RefCell::new(1_()));
    unimplemented!();
    unimplemented!();
    let a: Value<Access_S_> = Rc::new(RefCell::new(<Access_S_>::default()));
    let mut cs: Ptr<S> = (s.as_pointer());
    let cr: Ptr<S> = s.as_pointer();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut b: Box_ = Box_ { v: 4_() };
    unimplemented!();
    return 0_();
}
pub trait Access_S_Impl {
    fn get_1(&self, p: Ptr<S>) -> ();
    fn get_2(&self, p: Ptr<S>) -> ();
    fn ref_3(&self, r: Ptr<S>) -> ();
    fn ref_4(&self, r: Ptr<S>) -> ();
}
impl Access_S_Impl for Ptr<Access_S_> {
    fn get_1(&self, mut p: Ptr<S>) -> () {
        return p.with(|__s| __s.base);
    }
    fn get_2(&self, mut p: Ptr<S>) -> () {
        return (p.with(|__s| __s.base) + 1_());
    }
    fn ref_3(&self, r: Ptr<S>) -> () {
        return (r.with(|__s| __s.base) + 2_());
    }
    fn ref_4(&self, r: Ptr<S>) -> () {
        return (r.with(|__s| __s.base) + 3_());
    }
}
pub trait SImpl {
    fn plain_4(&self, x: ()) -> ();
    fn plain_5(&self, x: ()) -> ();
    fn take_6(&self, x: Ptr<()>) -> ();
    fn take_7(&self, x: Ptr<()>) -> ();
    fn pick_8(&self, p: ()) -> ();
    fn pick_9(&self, p: ()) -> ();
    fn apply_10(&self, f: FnPtr<fn(Ptr<()>)>, x: ()) -> ();
    fn apply_11(&self, f: FnPtr<fn(Ptr<()>, ())>, x: ()) -> ();
    fn apply_12(&self, f: FnPtr<fn(()) -> ()>, x: ()) -> ();
    fn combine_13(&self, p: Ptr<()>, f: FnPtr<fn(()) -> ()>, q: Ptr<()>, n: ()) -> ();
    fn combine_14(&self, p: Ptr<()>, f: FnPtr<fn(Ptr<()>, ())>, q: Ptr<()>, n: ()) -> ();
    fn width_1_char(&self, x: ()) -> ();
    fn width_1_int(&self, x: ()) -> ();
    fn scale_2_2(&self, x: ()) -> ();
    fn scale_2_3(&self, x: ()) -> ();
    fn count_3(&self, x: ()) -> ();
    fn count_3_int_long(&self, x: ()) -> ();
}
impl SImpl for Ptr<S> {
    fn plain_4(&self, mut x: ()) -> () {
        return ((*self).with(|__s| __s.base) + x);
    }
    fn plain_5(&self, mut x: ()) -> () {
        return (((*self).with(|__s| __s.base) + x) + 1_());
    }
    fn take_6(&self, x: Ptr<()>) -> () {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 1_());
    }
    fn take_7(&self, x: Ptr<()>) -> () {
        return (({ (*self).with(|__s| __s.base) } + { (x.read()) }) + 2_());
    }
    fn pick_8(&self, p: ()) -> () {
        let p: Value<()> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + { (*p.borrow()).first });
    }
    fn pick_9(&self, p: ()) -> () {
        let p: Value<()> = Rc::new(RefCell::new(p));
        return ((*self).with(|__s| __s.base) + { (*p.borrow()).second });
    }
    fn apply_10(&self, mut f: FnPtr<fn(Ptr<()>)>, x: ()) -> () {
        let x: Value<()> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer())) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_11(&self, mut f: FnPtr<fn(Ptr<()>, ())>, x: ()) -> () {
        let x: Value<()> = Rc::new(RefCell::new(x));
        ({ f.call((x.as_pointer()), 10_()) });
        return ((*self).with(|__s| __s.base) + (*x.borrow()));
    }
    fn apply_12(&self, mut f: FnPtr<fn(()) -> ()>, mut x: ()) -> () {
        return ({ (*self).with(|__s| __s.base) } + { ({ f.call(x) }) });
    }
    fn combine_13(&self, p: Ptr<()>, mut f: FnPtr<fn(()) -> ()>, mut q: Ptr<()>, mut n: ()) -> () {
        return ({
            ({ ({ (*self).with(|__s| __s.base) } + { p.with(|__s| __s.second) }) } + {
                ({ f.call((q.read())) })
            })
        } + { n });
    }
    fn combine_14(
        &self,
        p: Ptr<()>,
        mut f: FnPtr<fn(Ptr<()>, ())>,
        mut q: Ptr<()>,
        mut n: (),
    ) -> () {
        ({
            let _arg0: Ptr<()> = (q).clone();
            let _arg1: () = n;
            f.call(_arg0, _arg1)
        });
        return ({ ({ (*self).with(|__s| __s.base) } + { p.with(|__s| __s.first) }) } + {
            (q.read())
        });
    }
    fn width_1_char(&self, mut x: ()) -> () {
        return ((*self).with(|__s| __s.base) + (x * ::std::mem::size_of::<()>()));
    }
    fn width_1_int(&self, mut x: ()) -> () {
        return ((*self).with(|__s| __s.base) + (x * ::std::mem::size_of::<()>()));
    }
    fn scale_2_2(&self, mut x: ()) -> () {
        return ((*self).with(|__s| __s.base) + (x * 2_()));
    }
    fn scale_2_3(&self, mut x: ()) -> () {
        return ((*self).with(|__s| __s.base) + (x * 3_()));
    }
    fn count_3(&self, mut x: ()) -> () {
        return (((*self).with(|__s| __s.base) + x) + 0);
    }
    fn count_3_int_long(&self, mut x: ()) -> () {
        return (((*self).with(|__s| __s.base) + x) + 2);
    }
}
pub fn __cpp2rust_init_globals() {}
