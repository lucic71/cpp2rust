// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use libcc2rs::*;

unsafe fn f1() -> *mut ::std::fs::File {
    libcc2rs::cerr_unsafe()
}

unsafe fn f4() -> *mut ::std::fs::File {
    libcc2rs::cout_unsafe()
}
