# Temporary Materialization

C++ lets a temporary bind to a `const T &` parameter, and rules sometimes ask
for a pointer where the C++ argument is a plain value. Rust has no implicit
temporaries with an address, so the converter creates a named one. Given

```cpp
struct Stack {
  void push(const int &item);
};
Stack s;
s.push(1);
```

the unsafe model produces

```rust
(unsafe {
    let mut _item = 1;
    s.push(&mut _item)
});
```

and the refcount model produces

```rust
({
    let _item: Value<i32> = Rc::new(RefCell::new(1));
    (*s.borrow()).push(_item.as_pointer())
});
```

`MaterializeTemp(name, type, expr)` emits the binding and returns the expression
that names it: `let mut name = value;` with `&mut name` in the unsafe model,
`let name: Value<T> = Rc::new(RefCell::new(value));` with `name.as_pointer()` in
the refcount model. The unsafe model does not look at the constness of the
reference: the binding is always `let mut` and the argument always `&mut name`,
which coerces to the `*const i32` of the `const int &` parameter (see
[Pointers](../types/pointers.md)) as well as to the `*mut` of a non-const one.
There is no visitor for clang's `MaterializeTemporaryExpr`,
`ExprWithCleanups`, or `CXXBindTemporaryExpr`; they are traversed through, and
only inspected at the two call sites below.

For a generic call, `CollectCallInfo` marks an argument `Materialized` when it
is a `MaterializeTemporaryExpr` passed to an lvalue reference parameter; the
binding is named `_<parameter>` and emitted with the other hoisted arguments
(see [Calls](./calls.md)).

For a rule-mapped call, `CollectRefBindingTempArgs` marks an argument when the
parameter is an lvalue reference and either the argument is a temporary or the
parameter is a `const` reference whose Rust type differs from the argument's, as
with `const size_t &` and an `unsigned long`. The temporary is then created
lazily, named `__tmp_<i>`, the first time a placeholder needs an address
(`TempMaterializationCtx::GetOrMaterialize`), and the bindings are prepended to
the call in a block. `std::max(30, 40)` becomes in the unsafe model

```rust
{
    let mut __tmp_0 = 30;
    let mut __tmp_1 = 40;
    (*if *&mut __tmp_0 >= *&mut __tmp_1 {
        (&mut __tmp_0) as *const _
    } else {
        (&mut __tmp_1) as *const _
    })
}
```

and in the refcount model

```rust
{
    let __tmp_0: Value<i32> = Rc::new(RefCell::new(30));
    let __tmp_1: Value<i32> = Rc::new(RefCell::new(40));
    (if __tmp_0.as_pointer().read() >= __tmp_1.as_pointer().read() {
        __tmp_0.as_pointer()
    } else {
        __tmp_1.as_pointer()
    }
    .read())
}
```
