extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn registerSelf(&mut self);
}
thread_local!(
    pub static registered_globally_0: Value<PtrDyn<dyn DeclaresVirtual>> =
        Rc::new(RefCell::new(Ptr::<dyn DeclaresVirtual>::null()));
);
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct Registry {
    #[offset(0)]
    #[byte_size(8)]
    pub entry: PtrDyn<dyn DeclaresVirtual>,
}
impl Default for Registry {
    fn default() -> Self {
        {
            Registry {
                entry: Ptr::<dyn DeclaresVirtual>::null(),
            }
        }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, CppFrom, Default)]
#[cpp_from(Ptr< Registry  >  , new, value)]
#[byte_size(24)]
pub struct StoresThis {
    #[offset(16)]
    #[byte_size(8)]
    pub registry: Ptr<Registry>,
}
impl StoresThis {
    pub fn new(mut r: Ptr<Registry>) -> Self {
        Self {
            registry: (r).clone(),
        }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let registry: Value<Registry> = Rc::new(RefCell::new(<Registry>::default()));
    let d: Value<StoresThis> = Rc::new(RefCell::new(StoresThis::new({ (registry.as_pointer()) })));
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (d.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!(({ (*registered_globally_0.with(Value::clone).borrow()).is_null() }));
    assert!(({ ({ (*registry.borrow()).entry.clone() }).is_null() }));
    ({ (*base.upgrade().deref_mut()).registerSelf() });
    assert!(
        ({
            ({ (*registered_globally_0.with(Value::clone).borrow()).clone() } == { (base).clone() })
        })
    );
    assert!(({ ({ { (*registry.borrow()).entry.clone() } } == { (base).clone() }) }));
    (*(*registered_globally_0.with(Value::clone).borrow())
        .upgrade()
        .deref_mut())
    .value = 3;
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtual).value } == 3) }));
    (*{ (*registry.borrow()).entry.clone() }.upgrade().deref_mut()).value = 4;
    assert!(({ ({ ((*d.borrow()) as DeclaresVirtual).value } == 4) }));
    return 0;
}
impl DeclaresVirtual for StoresThis {
    fn registerSelf(&mut self) {
        registered_globally_0
            .with(|rc| *rc.borrow_mut() = (*self).to_dyn::<dyn DeclaresVirtual>(|w| w));
        field!({ self.registry.clone() }, entry)
            .write((*self).to_dyn::<dyn DeclaresVirtual>(|w| w));
    }
}
pub fn __cpp2rust_init_globals() {
    let _ = registered_globally_0.with(|_| ());
}
