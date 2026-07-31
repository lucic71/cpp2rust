extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive()]
pub struct WOFF2Params {
    pub extended_metadata: Vec<u8>,
    pub brotli_quality: i32,
    pub allow_transforms: bool,
}
impl WOFF2Params {
    pub fn WOFF2Params() -> Self {
        let mut this = Self {
            extended_metadata: Ptr::from_string_literal(b"")
                .to_c_string_iterator()
                .chain(std::iter::once(0))
                .collect::<Vec<u8>>(),
            brotli_quality: 11,
            allow_transforms: true,
        };
        this
    }
}
impl Clone for WOFF2Params {
    fn clone(&self) -> Self {
        let mut this = Self {
            extended_metadata: (self.extended_metadata).clone(),
            brotli_quality: self.brotli_quality,
            allow_transforms: self.allow_transforms,
        };
        this
    }
}
impl Default for WOFF2Params {
    fn default() -> Self {
        { WOFF2Params::WOFF2Params() }
    }
}
impl ByteRepr for WOFF2Params {}
pub fn main() {
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let params: Value<WOFF2Params> = Rc::new(RefCell::new(WOFF2Params::WOFF2Params()));
    assert!((((*params.borrow()).extended_metadata.len() - 1) == 0_usize));
    assert!(((*params.borrow()).brotli_quality == 11));
    assert!((((*params.borrow()).allow_transforms as i32) == (true as i32)));
    return 0;
}
