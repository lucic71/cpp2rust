// Copyright (c) 2022-present INESC-ID.
// Distributed under the MIT license that can be found in the LICENSE file.

use std::cell::{Cell, UnsafeCell};
use std::mem::MaybeUninit;
use std::sync::Mutex;

pub struct LazyStorage<T> {
    state: Cell<u8>,
    value: UnsafeCell<MaybeUninit<T>>,
    init: fn(*mut T),
}

const UNINIT: u8 = 0;
const RUNNING: u8 = 1;
const INIT: u8 = 2;

impl<T> LazyStorage<T> {
    pub const fn new(init: fn(*mut T)) -> Self {
        LazyStorage {
            state: Cell::new(UNINIT),
            value: UnsafeCell::new(MaybeUninit::zeroed()),
            init,
        }
    }

    #[inline]
    pub fn get(&self) -> *mut T {
        let ptr = self.value.get().cast::<T>();
        match self.state.get() {
            INIT => {}
            RUNNING => panic!("a static variable is used by its own initializer"),
            _ => {
                self.state.set(RUNNING);
                (self.init)(ptr);
                self.state.set(INIT);
            }
        }
        ptr
    }
}

static STATIC_DESTRUCTORS: Mutex<Vec<fn()>> = Mutex::new(Vec::new());

pub fn register_static_destructor(destructor: fn()) {
    STATIC_DESTRUCTORS.lock().unwrap().push(destructor);
}

pub fn run_static_destructors() {
    loop {
        let destructor = STATIC_DESTRUCTORS.lock().unwrap().pop();
        match destructor {
            Some(destructor) => destructor(),
            None => return,
        }
    }
}

pub fn cpp2rust_exit(code: i32) -> ! {
    run_static_destructors();
    std::process::exit(code)
}
