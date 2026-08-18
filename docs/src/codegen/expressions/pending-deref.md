# Pending Dereferences

The refcount model cannot print `*p` on the left of an assignment as a Rust
place: `Ptr` hands out no references, only `read()` and `write(v)`. So when a
dereference is converted as an `LValue`, `ConverterRefCount` prints nothing and
instead records the pointer expression in
[`pending_deref_`](../internals/state.md); whoever consumes the lvalue then
wraps the pointer in the right call. Given

```cpp
*p2 = 1;
*p += 10;
++*other_pointer;
v.push_back(20);  // v is std::vector<int> &
```

the refcount model produces

```rust
(*p2.borrow()).write(1);
{
    let _ptr = (*p.borrow()).clone();
    _ptr.write(_ptr.read() + 10)
};
(*other_pointer.borrow()).with_mut(|__v| __v.prefix_inc());
v.with_mut(|__v: &mut Vec<i32>| __v.push(20));
```

`PendingDeref` holds the pointer text and a `pointee_is_boxed` flag (true when
the pointee maps to a `Vec<..>` or `Box<..>`, so that the consumer knows to go
through the inner `Value`; see the nested containers paragraph of
[Boxing](../types/boxing.md)). `set` asserts nothing is already pending, `take`
returns and clears it, and the refcount override of `Convert(Stmt)` calls
`assert_consumed` after every statement, so a pending dereference can never leak
into the next one.

## Who sets it

All under `isLValue()`: `ConvertDeref` for `*p`; `VisitDeclRefExpr` for a
reference variable and `VisitMemberExpr` for a reference-typed field; a union
member accessor; a call whose callee returns a reference; the overloaded
`operator*`, `operator->` and `operator[]` of mapped types (the subscript is
recorded as `(v.as_pointer() as Ptr<T>).offset(i)`); and pointer subscripts
`p[i]`.

## Who consumes it

- `EmitSetOrAssign(lhs, rhs)` converts the lhs as an lvalue; if a dereference is
  pending it prints `ptr.write(rhs)`, otherwise `lhs = rhs`.
- A compound assignment prints
  `{ let _ptr = ptr.clone(); _ptr.write(_ptr.read() op rhs) }`.
- `ConvertIncAndDec` prints `ptr.with_mut(|__v| __v.prefix_inc())`.
- `ConvertMappedMethodCall`, for a rule whose receiver placeholder is written
  to, prints `ptr.with_mut(|__v: &mut T| __v.method(...))`, or
  `ptr.with_mut(|__v: &mut Value<T>| (*__v.borrow_mut()).method(...))` when the
  pointee is boxed (see [Applying Rules](./rules.md)).

The right-hand side of an assignment is converted before the left, so converting
it cannot clobber the pending pointer; when the two sides could conflict the rhs
is hoisted into `let __rhs = ...;` first (see
[Operators](./operators.md#assignment)).

Field access through a pointer does not use this mechanism: `p->x = 10` converts
the base as an rvalue and prints
`(*(*(*p.borrow()).upgrade().deref()).x.borrow_mut()) = 10` directly (see
[Members and Subscripts](./members.md)).
