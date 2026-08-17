# Global Variables

Given

```cpp
int counter;
static const char *name = "x";
int *p = &counter;
```

the unsafe model produces

```rust
pub static mut counter_0: i32 = unsafe { 0_i32 };
pub static mut name_1: *const libc::c_char = unsafe { c"x".as_ptr() };
pub static mut p_2: *mut i32 = unsafe { &raw mut counter_0 as *mut i32 };
```

and the refcount model produces

```rust
thread_local!(
    pub static counter_0: Value<i32> = <Value<i32>>::default();
);
thread_local!(
    pub static name_1: Value<Ptr<u8>> =
        Rc::new(RefCell::new(Ptr::from_string_literal(b"x")));
);
thread_local!(
    pub static p_2: Value<Ptr<i32>> =
        Rc::new(RefCell::new(counter_0.with(Value::clone).as_pointer()));
);
```

`IsGlobalVar` is true for file-scope variables and for `static` locals, and both
go through `ConvertGlobalVarDecl`. Only definitions are emitted: an
`extern int x;` with no initializer prints nothing, and a C tentative definition
repeated in the same unit is emitted once (`globals_` tracks the names). Like
functions, globals are always `pub`, C `static` included, and get the `_N`
suffix that keeps same-named `static`s from different files apart (see
[Naming](../types/naming.md)).

## Unsafe model

A global is a `static mut` whose initializer is wrapped in an `unsafe` block, so
that it can take the address of another `static mut` (`&raw mut counter_0`).
`ConvertVarDecl` sets `in_const_initializer_` while converting it, because a
Rust `static` initializer must be a constant expression: a record without an
initializer is printed as a struct literal of its fields' defaults rather than
`<T>::default()`, which is not `const` (see [Default Values](./defaults.md)). A
global whose class has a user-defined default constructor and no initializer
cannot be emitted this way and stops the translation. Uses of the global are
plain identifiers, `counter_0`, which is why every function is `unsafe fn`.

## Refcount model

Global variables are mapped to thread-local storage, because a `Value<T>` cannot
be a true Rust global. A global must be `Sync`, since every thread can reach it,
and both `Rc` and `RefCell` are single-threaded types: the reference counter and
the borrow checks are not atomic. Thread-local storage sidesteps the requirement
by giving each thread its own copy, which matches the original semantics because
`cpp2rust` does not currently support multi-threaded code.

`ConvertGlobalVarDecl` wraps the ordinary variable declaration in
`thread_local!( ... );`, and since the initializer of a `thread_local!` runs
lazily on first access, it can be any expression: `<T>::default()` for records,
constructor calls, addresses of other globals. A use of the global is
`counter_0.with(Value::clone)`: `with` lends the `Value` to a closure, and
cloning the `Rc` out of it gives an owned `Value` that the rest of the
expression borrows like any local (`*counter_0.with(Value::clone).borrow()`).

> [!NOTE]
>
> `with(Value::clone)` may change in the future to something closer to how
> `with` and `with_mut` work on `Ptr` (see
> [Dereferences](../../runtime/rc.md#dereferences)): the access would run inside
> the closure instead of cloning the `Rc` out first.

## Static locals and static data members

A `static` local is emitted with the same forms, inside the function body:
`static mut kX1_4: i32 = unsafe { 1 };` and `thread_local!(static kX1_4: ...)`.
Because they are globals, hoisting skips them and their names carry the `_N`
suffix, so two functions with a `static int i` do not collide.

A `static` data member of a class is emitted before the struct as a top-level
global (`EmitRustStructOrUnion` visits the class's variables first), and a use
such as `S::inner_const` is the plain global name `inner_const_0`.
