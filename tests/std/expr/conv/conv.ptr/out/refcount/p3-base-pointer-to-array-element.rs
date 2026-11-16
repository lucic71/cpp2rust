extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub trait DeclaresVirtual {
    fn id(&mut self) -> i32;
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct HoldsId {
    #[offset(8)]
    pub held: i32,
}
impl Default for HoldsId {
    fn default() -> Self {
        { HoldsId { held: 0 } }
    }
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let elements: Value<Box<[HoldsId]>> = Rc::new(RefCell::new(
        (0..3)
            .map(|_| <HoldsId>::default())
            .collect::<Box<[HoldsId]>>(),
    ));
    (*elements.borrow_mut())[(0) as usize].held = 1;
    (*elements.borrow_mut())[(1) as usize].held = 2;
    (*elements.borrow_mut())[(2) as usize].held = 3;
    let mut base: PtrDyn<dyn DeclaresVirtual> =
        (elements.decay().offset(1)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 2));
    base = (elements.decay().offset(2)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 3));
    base = (elements.decay().offset(0)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 1));
    let mut heap: Ptr<HoldsId> = Ptr::alloc_array(
        (0..2_usize)
            .map(|_| <HoldsId>::default())
            .collect::<Box<[HoldsId]>>(),
    );
    field!(elem!(heap, 0), held).write(8);
    field!(elem!(heap, 1), held).write(9);
    base = (heap.offset((1) as isize)).to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 9));
    base = heap.to_dyn::<dyn DeclaresVirtual>(|w| w);
    assert!((({ (*base.upgrade().deref_mut()).id() }) == 8));
    heap.delete();
    return 0;
}
impl DeclaresVirtual for HoldsId {
    fn id(&mut self) -> i32 {
        return { self.held };
    }
}
pub fn __cpp2rust_init_globals() {}
