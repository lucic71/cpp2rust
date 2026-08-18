# Calls

Given

```cpp
double f1(double a, double b);
double f2(double c, double d, double e);
// ...
double z2 = f2(z1, f1(x, y), y);
```

the unsafe model produces

```rust
let mut z2: f64 = unsafe {
    let _d: f64 = unsafe { f1_0(x, y) };
    let _e: f64 = y;
    f2_1(z1, _d, _e)
};
```

and the refcount model produces

```rust
let z2: Value<f64> = Rc::new(RefCell::new({
    let _d: f64 = { f1_0(*x.borrow(), *y.borrow()) };
    let _e: f64 = *y.borrow();
    f2_1(*z1.borrow(), _d, _e)
}));
```

## Dispatch

`VisitCallExpr` tries, in order:

1. The `va_start`/`va_end`/`va_copy` builtins (see
   [Variadic Functions](./variadic.md)).
2. A [plugin](./plugins.md).
3. A translation rule for the callee (see [Applying Rules](./rules.md)). If the
   rule is a libc passthrough, this step goes straight to the generic path of
   step 8 instead.
4. `std::move`, which prints just its argument.
5. An overloaded operator call.
6. `printf` and `fprintf`, recognized by name in `ConvertCallExpr` and sent to
   `ConvertPrintf` (see [printf and Streams](./io.md)). They have no translation
   rule, which is why step 3 does not catch them.
7. `__builtin_constant_p`, printed as `1` or `0` depending on whether its
   argument is a constant expression.
8. The generic path, `ConvertGenericCallExpr`, which handles user functions,
   methods, function pointers, and the libc passthroughs sent here by step 3.

A generic call is printed as `(unsafe { ... })` in the unsafe model and
`({ ... })` in the refcount model. The block holds the hoisted argument
bindings, and the parentheses let the block be used as an operand.

In the unsafe model the block is `unsafe` because every translated function is
an `unsafe fn` (see [Functions](../declarations/functions.md)), so calling one
is an unsafe operation, and in Rust 2024 the body of an `unsafe fn` is not an
unsafe context: an unsafe operation inside it must still be inside an `unsafe`
block, or `rustc` reports `unsafe_op_in_unsafe_fn`.

## Arguments

`CollectCallInfo` builds a `CallInfo` with one `CallArg` per named parameter,
each of kind `Inline`, `Hoisted`, or `Materialized`, plus the variadic tail:

- A literal argument, or any argument of a libc passthrough call, is `Inline`.
- A `MaterializeTemporaryExpr` bound to a reference parameter is `Materialized`
  (see [Temporary Materialization](./temporaries.md)); bound to a value
  parameter it is `Inline`.
- Everything else starts as `Hoisted` and is demoted to `Inline` when it aliases
  nothing: `ArgsMayAlias` checks each hoisted argument against the receiver and
  against every other argument (both mention `this`, share a variable, or one
  goes through a pointer or reference to the other's type). This is what
  produced `_d` and `_e` above: `y` appears in both `f1(x, y)` and the last
  argument.

`EmitCall` prints the hoisted bindings first, `let _d: f64 = ...;` named
`_<parameter>`, then the callee, then `EmitArgList`. Each argument is converted
through `ConvertParamTy`, which converts as `AddrOf` for a reference parameter
and otherwise like a variable initializer with the parameter type as implicit
conversion target, so a `size_t` argument to an `unsigned long` parameter is
cast, `take_ulong_1(sz as u64)`. An argument for a parameter with a default is
wrapped in `Some(...)` (see [Functions](../declarations/functions.md)). Every
argument is followed by a comma, so trailing commas appear inside macros:
`apply_1(5, None,)`.

## Callees

- A user function prints its suffixed name; a method is `recv.method(...)` with
  the receiver `(*p).m()` in the unsafe model, `(*v.borrow()).m()` for a value
  and `(*(*p.borrow()).upgrade().deref()).m()` through a pointer in the refcount
  model.
- A function pointer is `fn_.unwrap()(args)` and `(*(*fn_.borrow()))(args)` (see
  [Function Pointers](../types/fn-pointers.md)).
- A libc passthrough (see
  [Passthrough rules](../../rules/writing-rules.md#passthrough-rules)) is
  `libc::name(args)`, each inline argument cast to the Rust parameter type of
  the rule: `libc::fcntl(fds[0 as usize] as i32, 3 as i32, 0)`.
  `IsLibcPassthrough` holds when the rule body is empty and `extern`, and only
  the unsafe rules are; the refcount rules for the same functions have bodies
  and go through the rule path.

## Return values

A call returning a reference is dereferenced when the value is wanted. Given
`struct Stack { const int &top(); };` and `int t = s.top();`, the initializer is
`(*(unsafe { s.top() }))` in the unsafe model and
`({ (*s.borrow()).top() }).read()` in the refcount model (`.upgrade().deref()`
for a record); as an lvalue in the refcount model it becomes a
[pending dereference](./pending-deref.md). When the address of a value return is
wanted, the refcount model boxes it on the spot:
`Rc::new(RefCell::new(call)).as_pointer()`.
