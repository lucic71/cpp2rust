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
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct FirstMixin {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub copyStamp: i32,
}
impl FirstMixin {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            copyStamp: 0,
        }
    }
    pub fn copy_from(o: Ptr<FirstMixin>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
            copyStamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Clone for FirstMixin {
    fn clone(&self) -> Self {
        let __src: Value<FirstMixin> = Rc::new(RefCell::new(FirstMixin {
            value: self.value.clone(),
            copyStamp: self.copyStamp.clone(),
        }));
        FirstMixin::copy_from(__src.as_pointer())
    }
}
impl Default for FirstMixin {
    fn default() -> Self {
        {
            FirstMixin {
                value: 0_i32,
                copyStamp: 0,
            }
        }
    }
}
#[derive(Record, ByteRepr, VaArg, FnPtrArg, CppFrom, CopyAssign)]
#[cpp_from(i32, new, value)]
#[byte_size(8)]
pub struct SecondMixin {
    #[offset(0)]
    pub value: i32,
    #[offset(4)]
    pub copyStamp: i32,
}
impl SecondMixin {
    pub fn new(mut v: i32) -> Self {
        Self {
            value: v,
            copyStamp: 0,
        }
    }
    pub fn copy_from(o: Ptr<SecondMixin>) -> Self {
        Self {
            value: o.with(|__s| __s.value),
            copyStamp: (*sequence_0.with(Value::clone).borrow_mut()).prefix_inc(),
        }
    }
}
impl Clone for SecondMixin {
    fn clone(&self) -> Self {
        let __src: Value<SecondMixin> = Rc::new(RefCell::new(SecondMixin {
            value: self.value.clone(),
            copyStamp: self.copyStamp.clone(),
        }));
        SecondMixin::copy_from(__src.as_pointer())
    }
}
impl Default for SecondMixin {
    fn default() -> Self {
        {
            SecondMixin {
                value: 0_i32,
                copyStamp: 0,
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct CopiesMixinsIntoBases_FirstMixin__SecondMixin_ {}
impl CopiesMixinsIntoBases_FirstMixin__SecondMixin_ {
    pub fn new(mixins_0: Ptr<FirstMixin>, mixins_1: Ptr<SecondMixin>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(SecondMixin, new, ptr)]
#[byte_size(8)]
pub struct CopiesMixinsIntoBases_SecondMixin_ {}
impl CopiesMixinsIntoBases_SecondMixin_ {
    pub fn new(mixins: Ptr<SecondMixin>) -> Self {
        Self {}
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct CopiesMixinsIntoBases__ {}
impl CopiesMixinsIntoBases__ {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for CopiesMixinsIntoBases__ {
    fn default() -> Self {
        { CopiesMixinsIntoBases__::new() }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let first: Value<FirstMixin> = Rc::new(RefCell::new(FirstMixin::new({ 1 })));
    let second: Value<SecondMixin> = Rc::new(RefCell::new(SecondMixin::new({ 2 })));
    let both: Value<CopiesMixinsIntoBases_FirstMixin__SecondMixin_> = Rc::new(RefCell::new(
        CopiesMixinsIntoBases_FirstMixin__SecondMixin_::new({ first.as_pointer() }, {
            second.as_pointer()
        }),
    ));
    assert!(({ ({ ((*both.borrow()) as FirstMixin).value } == 1) }));
    assert!(({ ({ ((*both.borrow()) as SecondMixin).value } == 2) }));
    assert!(({ ({ ((*both.borrow()) as FirstMixin).copyStamp } == 1) }));
    assert!(({ ({ ((*both.borrow()) as SecondMixin).copyStamp } == 2) }));
    assert!(
        ({ ({ (*first.borrow()).copyStamp } == 0) && ({ (*second.borrow()).copyStamp } == 0) })
    );
    let one: Value<CopiesMixinsIntoBases_SecondMixin_> =
        Rc::new(RefCell::new(CopiesMixinsIntoBases_SecondMixin_::new({
            second.as_pointer()
        })));
    assert!(({ ({ ((*one.borrow()) as SecondMixin).value } == 2) }));
    assert!(({ ({ ((*one.borrow()) as SecondMixin).copyStamp } == 3) }));
    let mut none: CopiesMixinsIntoBases__ = CopiesMixinsIntoBases__::new();
    assert!((sequence_0.with(|rc| *rc.borrow()) == 3));
    return 0;
}
pub trait FirstMixinImpl {
    fn copy_assign(&self, _a0: Ptr<FirstMixin>) -> Ptr<FirstMixin>;
}
impl FirstMixinImpl for Ptr<FirstMixin> {
    fn copy_assign(&self, _a0: Ptr<FirstMixin>) -> Ptr<FirstMixin> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        field!((*self), copyStamp).write({ _a0.with(|__s| __s.copyStamp) });
        return (*self).clone();
    }
}
pub trait SecondMixinImpl {
    fn copy_assign(&self, _a0: Ptr<SecondMixin>) -> Ptr<SecondMixin>;
}
impl SecondMixinImpl for Ptr<SecondMixin> {
    fn copy_assign(&self, _a0: Ptr<SecondMixin>) -> Ptr<SecondMixin> {
        field!((*self), value).write({ _a0.with(|__s| __s.value) });
        field!((*self), copyStamp).write({ _a0.with(|__s| __s.copyStamp) });
        return (*self).clone();
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = sequence_0.with(|_| ());
}
