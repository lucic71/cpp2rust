# Classes

A class becomes a struct with one field per data member, an `impl` block holding
its constructors and methods, and trait implementations after it. Given

```cpp
class Counter {
  int count_;

public:
  Counter(int start) : count_(start) {}
  ~Counter() { count_ = 0; }
  int get() const { return count_; }
  void set(int v) { count_ = v; }
};
```

the unsafe model produces

```rust
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Counter {
    count_: i32,
}
impl Counter {
    pub unsafe fn Counter(mut start: i32) -> Self {
        let mut this = Self { count_: start };
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.count_;
    }
    pub unsafe fn set(&mut self, mut v: i32) {
        self.count_ = v;
    }
}
```

and the refcount model produces

```rust
#[derive(Default)]
pub struct Counter {
    count_: Value<i32>,
}
impl Counter {
    pub fn Counter(start: i32) -> Self {
        let start: Value<i32> = Rc::new(RefCell::new(start));
        let mut this = Self {
            count_: Rc::new(RefCell::new(*start.borrow())),
        };
        this
    }
    pub fn get(&self) -> i32 {
        return *self.count_.borrow();
    }
    pub fn set(&self, v: i32) {
        let v: Value<i32> = Rc::new(RefCell::new(v));
        *self.count_.borrow_mut() = *v.borrow();
    }
}
impl Drop for Counter {
    fn drop(&mut self) {
        *self.count_.borrow_mut() = 0;
    }
}
impl Clone for Counter {
    fn clone(&self) -> Self {
        let mut this = Self {
            count_: Rc::new(RefCell::new(*self.count_.borrow())),
        };
        this
    }
}
impl ByteRepr for Counter { /* byte_size, to_bytes, from_bytes */ }
```

In the unsafe model the struct carries `#[repr(C)]` and derives `Copy` (when all
fields are copyable), `Clone`, and `Default`. The refcount model derives only
`Default` and writes `Clone` by hand: C++ copies a struct member by member, and
a derived `Clone` on `Value` fields would only bump reference counts and leave
the copy sharing the original's storage, so the hand-written one creates a fresh
box per field. `ByteRepr` supports
[type reinterpretation](../../runtime/reinterpret.md).

Fields keep their C++ access: `pub` for public members, nothing for private
ones.

A constructor becomes an associated function named after the class that builds
`this` from the initializer list, runs the body, and returns it. Methods take
`&self` when `const` and `&mut self` otherwise in the unsafe model; in the
refcount model they always take `&self`, since mutation goes through the fields'
`RefCell`s.

A destructor with a body becomes `impl Drop` in the refcount model. The unsafe
model does not emit destructors at all.

Abstract classes become traits, so pointers to them print with `dyn`:
`*mut dyn Base` in the unsafe model, `PtrDyn<dyn Base>` in the refcount model.
