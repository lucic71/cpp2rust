// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::RefCell;
use std::ffi::c_void;
use std::rc::Rc;

use crate::rc::Ptr;
use crate::{AsPointer, Value};

unsafe extern "C" {
    #[cfg(target_os = "linux")]
    #[link_name = "malloc_usable_size"]
    fn platform_malloc_size(ptr: *mut c_void) -> usize;

    #[cfg(target_os = "macos")]
    #[link_name = "malloc_size"]
    fn platform_malloc_size(ptr: *const c_void) -> usize;

    #[cfg(target_os = "linux")]
    #[link_name = "__errno_location"]
    fn platform_errno_location() -> *mut i32;

    #[cfg(target_os = "macos")]
    #[link_name = "__error"]
    fn platform_errno_location() -> *mut i32;
}

/// # Safety
///
/// The pointer `ptr` must be a pointer to a block of memory allocated by
/// the appropriate allocator (e.g., `malloc`).
// The memory must not have been deallocated.
pub unsafe fn malloc_usable_size(ptr: *mut c_void) -> usize {
    #[cfg(target_os = "linux")]
    {
        unsafe { platform_malloc_size(ptr) }
    }
    #[cfg(target_os = "macos")]
    {
        unsafe { platform_malloc_size(ptr as *const c_void) }
    }
}

/// # Safety
///
/// Invokes the platform specific errno.
pub unsafe fn cpp2rust_errno_unsafe() -> *mut i32 {
    unsafe { platform_errno_location() }
}

thread_local! {
    static ERRNO: Value<i32> = Rc::new(RefCell::new(0));
}

pub fn cpp2rust_errno() -> Ptr<i32> {
    ERRNO.with(AsPointer::as_pointer)
}

thread_local! {
    static EXIT_HANDLERS: RefCell<Vec<crate::FnPtr<fn()>>> = const { RefCell::new(Vec::new()) };
}

pub fn atexit_refcount(a0: crate::FnPtr<fn()>) -> i32 {
    EXIT_HANDLERS.with(|handlers| handlers.borrow_mut().push(a0));
    0
}

pub fn exit_refcount(a0: i32) -> ! {
    let handlers = EXIT_HANDLERS.with(|handlers| handlers.take());
    for handler in handlers.iter().rev() {
        (**handler)();
    }
    std::process::exit(a0)
}

thread_local! {
    static SIGNAL_HANDLERS: RefCell<std::collections::HashMap<i32, crate::FnPtr<fn(i32)>>> =
        RefCell::new(std::collections::HashMap::new());
}

extern "C" fn cpp2rust_signal_trampoline(sig: i32) {
    let handler = SIGNAL_HANDLERS.with(|handlers| handlers.borrow().get(&sig).cloned());
    if let Some(handler) = handler {
        (*handler)(sig);
    }
}

pub fn signal_refcount(a0: i32, a1: crate::FnPtr<fn(i32)>) -> crate::FnPtr<fn(i32)> {
    let sig = match nix::sys::signal::Signal::try_from(a0) {
        Ok(sig) => sig,
        Err(e) => {
            cpp2rust_errno().write(e as i32);
            return crate::FnPtr::from_int(usize::MAX);
        }
    };
    let handler = if a1.is_null() {
        nix::sys::signal::SigHandler::SigDfl
    } else if a1.dangling_value() == Some(1) {
        nix::sys::signal::SigHandler::SigIgn
    } else {
        nix::sys::signal::SigHandler::Handler(cpp2rust_signal_trampoline)
    };
    match unsafe { nix::sys::signal::signal(sig, handler) } {
        Ok(_) => SIGNAL_HANDLERS.with(|handlers| match handlers.borrow_mut().insert(a0, a1) {
            Some(previous) => previous,
            None => crate::FnPtr::null(),
        }),
        Err(e) => {
            cpp2rust_errno().write(e as i32);
            crate::FnPtr::from_int(usize::MAX)
        }
    }
}
