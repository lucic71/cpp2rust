# Applying Rules

How rules are loaded, matched, and rewritten is the subject of the
[Translation Rules](../../rules/overview.md) part. This page is the converter
side: where the lookup happens and how the matched rule's placeholders are
filled with converted C++ arguments.

## Where rules are consulted

`Mapper::Contains(expr)` and `GetMappedAsString(expr, args, n)` are called from
`VisitCallExpr`, `VisitCXXConstructExpr`, `VisitMemberExpr`, `VisitDeclRefExpr`,
`VisitUnaryOperator` (for `&std::cout`), and `VisitIntegerLiteral`, where a
literal that came from a macro is looked up under the macro's name, which is how
`O_RDONLY` becomes `::libc::O_RDONLY`. An expression that refers to a
user-defined declaration is never matched (`RefersToUserDefinedDecl`), so rules
only ever apply to library code. `GetMappedAsString` returns an empty string
when there is no rule, and every caller treats that as "not mapped" and falls
through to its normal path. Before a member access is looked up,
`replaceNonUniformLibcField` collapses libc field paths that differ per platform
into the single field the runtime exposes, `st.st_mtim.tv_sec` and
`st.st_mtimespec.tv_sec` both becoming `st.st_mtime` (see
[libc shims](../../runtime/libc-shims.md)).

A function that has a rule but is used as a value rather than called (its
address is taken, or it is assigned to a function pointer) is not replaced by
the rule body; `MapFunctionName` turns it into the runtime's named function
instead, `Some(libcc2rs::fread_unsafe)` and
`FnPtr::<...>::new(libcc2rs::fread_refcount)`.

## Filling placeholders

`BuildUnifiedArgs` lines up the rule's `a0`, `a1`, ... with the call: for a
method call the receiver is `a0` and the explicit arguments follow, so there is
no separate `this` placeholder. `ConvertIRFragment` walks the rule body: text
fragments are copied, a generic fragment is instantiated with
`Mapper::InstantiateTemplate`, a `va` fragment becomes the variadic tail
(`ConvertVariadicTail`, see [Variadic Functions](./variadic.md)), a
`method_call` fragment converts its receiver and body fragments in turn, and
each `aN` placeholder goes through `ConvertPlaceholder` with a `PlaceholderCtx`
that knows the rule's parameter type, whether the argument is a C++ pointer,
whether its Rust type is a pointer (`Mapper::MapsToPointer`), whether the rule
declares the parameter as a pointer, whether this is the receiver, and how the
rule accesses it (read, write, move). The decision, in order:

- A function-pointer argument is converted as a `Callee`, so the bare function
  name is substituted.
- A C array where the rule wants a pointer becomes
  `(arr.as_mut_ptr() as *mut T)` / `(arr.as_pointer() as Ptr<T>)`.
- An argument with no address where the rule wants a pointer becomes a
  [materialized temporary](./temporaries.md), `__tmp_N`.
- A receiver that is not a Rust pointer but is declared as one in the rule is
  passed as `(recv.as_pointer() as Ptr<...>)`.
- A receiver that is a C++ pointer where the rule wants a value is dereferenced:
  `(*comps).shrink_to_fit()`, `(*(*sink.borrow()).upgrade().deref()).len()`.
- Any other non-pointer argument where the rule wants a pointer takes its
  address, `&mut x1` and `x1.as_pointer()`; see the `std::max` example on
  [Temporary Materialization](./temporaries.md).
- Otherwise the access decides: write access converts an lvalue, move access
  wraps it in `std::mem::take(&mut ...)`, and read access converts an rvalue
  with the C++ parameter type as implicit conversion target.

In the refcount model a `Value` receiver under write access comes out as
`(*v.borrow_mut()).push(10)`, and a `Ptr` receiver goes through the `with_mut`
rewrite described in [Rule Rewriting](../../rules/rewriting.md), which is where
the [pending dereference](./pending-deref.md) is consumed.

A rule body with several statements is wrapped in a block, as
`__builtin_mul_overflow(3L, 7L, &r)` shows:

```rust
{
    let (val, ovf) = 3_i64.overflowing_mul(7_i64);
    *(&mut r as *mut i64) = val;
    ovf
}
```

## Reference results

A rule call whose C++ function returns a reference yields a Rust pointer, so
when the value is wanted the mapped call is dereferenced. Given
`std::vector<int> v;`, `return v.front();` becomes
`return (*((v).first_mut().unwrap()));` in the unsafe model and
`return ((v.as_pointer() as Ptr<i32>).read());` in the refcount model. When the
result is assigned to instead, `v.front() += 5;`, the unsafe model writes
through the same dereference, `(*((v).first_mut().unwrap())) += 5;`, and the
refcount model records the pointer as a
[pending dereference](./pending-deref.md) that the assignment consumes:

```rust
{
    let _ptr = (v.as_pointer() as Ptr<i32>).clone();
    _ptr.write(_ptr.read() + 5)
};
```

A mapped member access whose rule returns a pointer (`Mapper::ReturnsPointer`)
is dereferenced the same way: `it->second` is `*it.second()`.
