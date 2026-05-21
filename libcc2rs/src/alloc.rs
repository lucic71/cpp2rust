// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::ffi::{c_char, c_void};

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
}

/// # Safety
///
/// Same contract as C's `malloc`.
pub unsafe fn malloc_unsafe(a0: u64) -> *mut c_void {
    unsafe { malloc(a0 as usize) }
}

/// # Safety
///
/// Same contract as C's `free`.
pub unsafe fn free_unsafe(a0: *mut c_void) {
    unsafe { free(a0) }
}

/// # Safety
///
/// Same contract as C's `realloc`.
pub unsafe fn realloc_unsafe(a0: *mut c_void, a1: u64) -> *mut c_void {
    unsafe { realloc(a0, a1 as usize) }
}

/// # Safety
///
/// Same contract as C's `calloc`.
pub unsafe fn calloc_unsafe(a0: u64, a1: u64) -> *mut c_void {
    unsafe { calloc(a0 as usize, a1 as usize) }
}

/// # Safety
///
/// Same contract as C's `strdup`.
pub unsafe fn strdup_unsafe(a0: *const u8) -> *mut u8 {
    unsafe { strdup(a0 as *const c_char) as *mut u8 }
}
