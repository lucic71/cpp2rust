// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: &mut ::std::fs::File) -> &mut ::std::fs::File {
    a0
}

unsafe fn f3(a0: ::std::fs::File) -> ::std::fs::File {
    a0.try_clone().unwrap()
}
