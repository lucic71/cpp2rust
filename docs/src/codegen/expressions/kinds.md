# Expression Kinds and Freshness

## Expression kinds

Before converting a subexpression, the enclosing construct pushes an `ExprKind`
on the [`curr_expr_kind_`](../internals/state.md) stack (`PushExprKind`), and
the subexpression's visitor reads it to choose its output form. For a variable
`x`, the unsafe and refcount models print:

- `RValue`: the value is wanted; `x` and `*x.borrow()`.
- `LValue`: a place to assign to; `x` and `*x.borrow_mut()`, or a
  [pending dereference](./pending-deref.md) when reached through a pointer.
- `AddrOf`: the address is wanted; `&mut x as *mut T` and `x.as_pointer()`.
- `Callee`: the expression is being called; a function `f` prints bare, `f_0`,
  in both models, where under `AddrOf` it would be `Some(f_0)` and
  `FnPtr::<fn()>::new(f_0)`.
- `Object`: the receiver of a method call; only the refcount model pushes it,
  and a pointer `p` to a boxed container then prints as
  `p.to_strong().as_pointer()`.
- `Void`: the value is discarded; pushed by `Convert(Stmt)` around every
  statement, and by the left side of a comma operator; `x;` prints as `x;` and
  `*x.borrow();`.
- `XValue`: pushed only for `std::move` in the refcount model; nothing consumes
  it, but it makes both `isLValue()` and `isRValue()` false, which is what makes
  `std::move(x)` print as `*x.borrow_mut()`.

`ConvertRValue`, `ConvertLValue`, `ConvertPointer`, and `ConvertObject` are the
wrappers that push a kind and return the converted text. With an empty stack
`isLValue()`, `isRValue()`, and `isVoid()` are all true.

`Convert(Expr, implicit_convert_to)` takes an optional target type. When the
expression's C type and the target are the same integer type but map to
different Rust types (`unsigned long` is `u64`, `size_t` is `usize`), the result
is wrapped in `(...) as usize`; see
[Implicit conversions to usize and isize](../types/casts.md#implicit-conversions-to-usize-and-isize).
Assignments, initializers, binary operands, conditional branches, and mapped
call arguments pass a target.

## Freshness

Every visitor also sets [`computed_expr_type_`](../internals/state.md), one of
`Value`, `FreshValue`, `Pointer`, `FreshPointer`. Fresh means the printed Rust
already yields an owned value, a literal, an arithmetic result,
`x.as_pointer()`; not fresh means it names existing storage, `*x.borrow()`, a
reference parameter `r`. When an owned value is needed and the expression is not
fresh, the caller appends `.clone()`: `ConvertFreshRValue`,
`ConvertFreshPointer`, `ConvertFresh*` do this. `SetValueFreshness(type)`
decides after a load: a `Copy` type (`TypeIsCopyable`: builtins, enums, records
that derive `Copy`, and function pointers in the unsafe model only) is fresh
even when read from a variable, since copying it out is free.

The two models differ in one point: raw pointers are `Copy`, so the unsafe model
never clones a pointer, while `Ptr` is not, so the refcount model does. Given
`int x2 = x1; int *p2 = &r;` with `int &r`:

```rust
// unsafe
let mut x2: i32 = x1;
let mut p2: *mut i32 = r;
// refcount
let x2: Value<i32> = Rc::new(RefCell::new(*x1.borrow()));
let p2: Value<Ptr<i32>> = Rc::new(RefCell::new(r.clone()));
```
