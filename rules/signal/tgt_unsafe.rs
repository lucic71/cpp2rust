// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *const ::libc::sigaction, a2: *mut ::libc::sigaction) -> i32 {
    libc::sigaction(a0, a1, a2)
}

unsafe fn f2(a0: i32) -> i32 {
    libc::raise(a0)
}

unsafe fn f3(a0: i32, a1: unsafe fn(i32)) -> unsafe fn(i32) {
    let __prev = libc::signal(
        a0,
        std::mem::transmute::<*const (), unsafe extern "C" fn(i32)>(a1 as *const ()) as usize,
    );
    std::mem::transmute::<usize, unsafe fn(i32)>(__prev)
}
