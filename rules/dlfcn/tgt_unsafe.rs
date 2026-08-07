// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: *const libc::c_char, a1: i32) -> *mut ::libc::c_void {
    libc::dlopen(a0, a1)
}

unsafe fn f2(a0: *mut ::libc::c_void, a1: *const libc::c_char) -> *mut ::libc::c_void {
    libc::dlsym(a0, a1)
}

unsafe fn f3(a0: *mut ::libc::c_void) -> i32 {
    libc::dlclose(a0)
}

unsafe fn f4() -> *mut libc::c_char {
    libc::dlerror()
}
