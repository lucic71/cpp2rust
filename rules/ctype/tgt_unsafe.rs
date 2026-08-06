// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f1(a0: i32) -> i32 {
    libc::isalnum(a0)
}

unsafe fn f2(a0: i32) -> i32 {
    libc::isalpha(a0)
}

unsafe fn f3(a0: i32) -> i32 {
    libc::isblank(a0)
}

unsafe fn f4(a0: i32) -> i32 {
    libc::iscntrl(a0)
}

unsafe fn f5(a0: i32) -> i32 {
    libc::isdigit(a0)
}

unsafe fn f6(a0: i32) -> i32 {
    libc::isgraph(a0)
}

unsafe fn f7(a0: i32) -> i32 {
    libc::islower(a0)
}

unsafe fn f8(a0: i32) -> i32 {
    libc::isprint(a0)
}

unsafe fn f9(a0: i32) -> i32 {
    libc::ispunct(a0)
}

unsafe fn f10(a0: i32) -> i32 {
    libc::isspace(a0)
}

unsafe fn f11(a0: i32) -> i32 {
    libc::isupper(a0)
}

unsafe fn f12(a0: i32) -> i32 {
    libc::isxdigit(a0)
}

unsafe fn f13(a0: i32) -> i32 {
    libc::tolower(a0)
}

unsafe fn f14(a0: i32) -> i32 {
    libc::toupper(a0)
}
