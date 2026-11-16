extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn isSameObject(&mut self, other: PtrDyn<dyn DeclaresVirtual>) -> bool;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct ComparesThis {}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<ComparesThis> = Rc::new(RefCell::new(<ComparesThis>::default()));
    let b: Value<ComparesThis> = Rc::new(RefCell::new(<ComparesThis>::default()));
    let mut baseOfA: PtrDyn<dyn DeclaresVirtual> =
        (a.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    let mut baseOfB: PtrDyn<dyn DeclaresVirtual> =
        (b.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!(
        ({
            let _other: PtrDyn<dyn DeclaresVirtual> = (baseOfA).clone();
            (*baseOfA.upgrade().deref_mut()).isSameObject(_other)
        })
    );
    assert!(!({ (*baseOfA.upgrade().deref_mut()).isSameObject((baseOfB).clone(),) }));
    assert!(
        ({
            (*baseOfB.upgrade().deref_mut())
                .isSameObject((b.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w))
        })
    );
    assert!(
        !({
            (*baseOfB.upgrade().deref_mut())
                .isSameObject((a.as_pointer()).to_dyn::<dyn DeclaresVirtual>(|w| w))
        })
    );
    return 0;
}
impl DeclaresVirtual for ComparesThis {
    fn isSameObject(&mut self, mut other: PtrDyn<dyn DeclaresVirtual>) -> bool {
        return ((*self).to_dyn::<dyn DeclaresVirtual>(|w| w) == other);
    }
}
pub fn __cpp2rust_init_globals() {}
