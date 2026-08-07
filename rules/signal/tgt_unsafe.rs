// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32, a1: *const ::libc::sigaction, a2: *mut ::libc::sigaction) -> i32 {
    libc::sigaction(a0, a1, a2)
}

unsafe fn f2(a0: i32) -> i32 {
    libc::raise(a0)
}

unsafe fn f3(a0: i32, a1: Option<unsafe fn(i32)>) -> Option<unsafe fn(i32)> {
    let __handler = match a1 {
        None => 0_usize,
        Some(__f) => __f as usize,
    };
    match libc::signal(a0, __handler) {
        0 => None,
        __prev => Some(std::mem::transmute::<usize, unsafe fn(i32)>(__prev)),
    }
}

unsafe fn f4(a0: i32, a1: i32) -> i32 {
    libc::kill(a0, a1)
}
