// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

fn f1(a0: Ptr<u8>, a1: i32) -> AnyPtr {
    panic!("dlopen: dynamic loading is not supported in the refcount model")
}

fn f2(a0: AnyPtr, a1: Ptr<u8>) -> AnyPtr {
    panic!("dlsym: dynamic loading is not supported in the refcount model")
}

fn f3(a0: AnyPtr) -> i32 {
    panic!("dlclose: dynamic loading is not supported in the refcount model")
}

fn f4() -> Ptr<u8> {
    Ptr::<u8>::null()
}
