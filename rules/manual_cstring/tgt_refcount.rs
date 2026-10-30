// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

#[cfg(target_os = "linux")]
fn f25(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>().offset(a2);
    let mut __i: usize = a2;
    loop {
        if __i == 0 {
            break Ptr::<u8>::null().to_any();
        }
        __p -= 1;
        __i -= 1;
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
    }
}

#[cfg(target_os = "linux")]
fn f26(a0: AnyPtr, a1: i32, a2: usize) -> AnyPtr {
    let mut __p = a0.reinterpret_cast::<u8>().offset(a2);
    let mut __i: usize = a2;
    loop {
        if __i == 0 {
            break Ptr::<u8>::null().to_any();
        }
        __p -= 1;
        __i -= 1;
        if __p.read() == a1 as u8 {
            break __p.to_any();
        }
    }
}
