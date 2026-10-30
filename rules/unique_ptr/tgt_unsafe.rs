// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

unsafe fn f2<T1>(a0: T1) -> Option<Box<T1>> {
    Some(Box::new(a0))
}

unsafe fn f23<T1>(a0: &mut Option<Box<[T1]>>) -> Option<Box<[T1]>> {
    a0.take()
}

unsafe fn f24<T1>(a0: &mut Option<Box<[T1]>>, a1: &mut Option<Box<[T1]>>) {
    *a0 = a1.take()
}

unsafe fn f26<T1>(a0: &mut Option<Box<[T1]>>, a1: usize) -> *mut T1 {
    a0.as_deref()
        .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut T1)
        .add(a1)
}

unsafe fn f27<T1>(a0: &mut Option<Box<[T1]>>) -> *mut T1 {
    a0.as_deref_mut()
        .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr())
}

unsafe fn f34<T1>() -> Option<Box<[T1]>> {
    None
}

unsafe fn f35<T1>(a0: *mut T1) -> Option<Box<T1>> {
    Some(Box::from_raw(a0))
}

unsafe fn f41<T1>(a0: &mut Option<Box<T1>>, a1: *mut T1) {
    *a0 = Some(Box::from_raw(a1))
}

unsafe fn f42<T1>(a0: &mut Option<Box<T1>>) -> Option<Box<T1>> {
    a0.take()
}

unsafe fn f43<T1>(a0: &mut Option<Box<T1>>, a1: &mut Option<Box<T1>>) {
    *a0 = a1.take()
}

unsafe fn f45<T1>(a0: &mut Option<Box<T1>>) -> *mut T1 {
    a0.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const T1 as *mut T1)
}

unsafe fn f46<T1>(a0: &mut Option<Box<T1>>) -> *mut T1 {
    a0.as_deref()
        .map_or(::std::ptr::null_mut(), |v| v as *const T1 as *mut T1)
}

unsafe fn f47<T1>(a0: &mut Option<Box<T1>>) -> *mut T1 {
    a0.as_deref_mut()
        .map_or(::std::ptr::null_mut(), |v| v as *mut T1)
}

unsafe fn f52<T1: Default>(a0: &mut Option<Box<T1>>, a1: *mut T1) {
    let _a0: *mut T1 = a1;
    *a0 = if _a0.is_null() {
        None
    } else {
        Some(Box::from_raw(_a0))
    }
}

unsafe fn f54<T1: Default>() -> Option<Box<T1>> {
    None
}

unsafe fn f55<T1>(a0: *mut T1) -> Option<Box<T1>> {
    Some(Box::from_raw(a0))
}
