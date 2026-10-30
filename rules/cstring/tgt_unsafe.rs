// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f2001(a0: *const libc::c_char, a1: i32) -> *const libc::c_char {
    (libc::strchr(a0, a1) as *const libc::c_char)
}

unsafe fn f2003(a0: *const libc::c_char, a1: i32) -> *const libc::c_char {
    (libc::strrchr(a0, a1) as *const libc::c_char)
}

unsafe fn f2004(a0: *mut libc::c_char, a1: i32) -> *mut libc::c_char {
    libc::strrchr(a0, a1)
}

unsafe fn f2005(a0: *const libc::c_char, a1: *const libc::c_char) -> *const libc::c_char {
    (libc::strpbrk(a0, a1) as *const libc::c_char)
}

unsafe fn f2006(a0: *mut libc::c_char, a1: *const libc::c_char) -> *mut libc::c_char {
    libc::strpbrk(a0, a1)
}

unsafe fn f2007(a0: *const libc::c_char, a1: *const libc::c_char) -> *const libc::c_char {
    (libc::strstr(a0, a1) as *const libc::c_char)
}

unsafe fn f2008(a0: *mut libc::c_char, a1: *const libc::c_char) -> *mut libc::c_char {
    libc::strstr(a0, a1)
}

unsafe fn f2009(a0: *const u8, a1: i32, a2: usize) -> *const ::libc::c_void {
    libc::memchr(a0 as *const ::libc::c_void, a1, a2 as usize) as *const ::libc::c_void
}
