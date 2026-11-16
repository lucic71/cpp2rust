extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(8)]
pub struct DeclaresVirtual {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct HoldsPolymorphicPointer {
    #[offset(0)]
    #[byte_size(8)]
    pub pointer: Ptr<DeclaresVirtual>,
    #[offset(8)]
    pub after: i32,
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(16)]
pub struct HoldsVoidPointer {
    #[offset(0)]
    #[byte_size(8)]
    pub pointer: AnyPtr,
    #[offset(8)]
    pub after: i32,
}
pub fn bytesBetween_0(mut from: AnyPtr, mut to: AnyPtr) -> i64 {
    return ((to.reinterpret_cast::<i8>() - from.reinterpret_cast::<i8>()) as i64 as i64);
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let polymorphicPointers: Value<Box<[Ptr<DeclaresVirtual>]>> =
        Rc::new(RefCell::new(Box::new([
            Ptr::<DeclaresVirtual>::null(),
            Ptr::<DeclaresVirtual>::null(),
            Ptr::<DeclaresVirtual>::null(),
        ])));
    let voidPointers: Value<Box<[AnyPtr]>> = Rc::new(RefCell::new(Box::new([
        AnyPtr::default(),
        AnyPtr::default(),
        AnyPtr::default(),
    ])));
    let mut polymorphicStride: i64 = ({
        let _from: AnyPtr = (polymorphicPointers.decay().offset(0)).to_any();
        let _to: AnyPtr = (polymorphicPointers.decay().offset(1)).to_any();
        bytesBetween_0(_from, _to)
    });
    let mut voidStride: i64 = ({
        let _from: AnyPtr = (voidPointers.decay().offset(0)).to_any();
        let _to: AnyPtr = (voidPointers.decay().offset(1)).to_any();
        bytesBetween_0(_from, _to)
    });
    assert!((polymorphicStride == voidStride));
    assert!(
        (({
            let _from: AnyPtr = (polymorphicPointers.decay().offset(0)).to_any();
            let _to: AnyPtr = (polymorphicPointers.decay().offset(2)).to_any();
            bytesBetween_0(_from, _to)
        }) == (2_i64 * voidStride))
    );
    let holdsPolymorphic: Value<HoldsPolymorphicPointer> =
        Rc::new(RefCell::new(<HoldsPolymorphicPointer>::default()));
    let holdsVoid: Value<HoldsVoidPointer> = Rc::new(RefCell::new(<HoldsVoidPointer>::default()));
    let mut polymorphicOffset: i64 = ({
        let _from: AnyPtr = (holdsPolymorphic.as_pointer()).to_any();
        let _to: AnyPtr = (field_ptr!(holdsPolymorphic.as_pointer(), after)).to_any();
        bytesBetween_0(_from, _to)
    });
    let mut voidOffset: i64 = ({
        let _from: AnyPtr = (holdsVoid.as_pointer()).to_any();
        let _to: AnyPtr = (field_ptr!(holdsVoid.as_pointer(), after)).to_any();
        bytesBetween_0(_from, _to)
    });
    assert!((polymorphicOffset == voidOffset));
    return 0;
}
pub fn __cpp2rust_init_globals() {}
