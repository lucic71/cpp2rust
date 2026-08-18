# Variable References

`VisitDeclRefExpr` prints a name. Both models first resolve what the name is
(`ConvertDeclRefExpr`): a function becomes its suffixed name `foo_1` (or
`Record::method` for a static method), an enumerator `Color::RED`, or
`(Color::RED as i32)` where C uses it as an integer, and a variable its suffixed
name. A name that has a translation rule is replaced by the rule instead. What
is printed around the name depends on the [expression kind](./kinds.md).

## Unsafe model

A local or global prints as its name: `x = 0;`, `x = y + 1;`,
`side_effect_0.prefix_inc();`. Under `AddrOf` it becomes `&mut x` (`&x` when
`const`; `&raw mut x` for a global, so no reference to the `static` is formed).
A reference-typed variable is a pointer and prints dereferenced, `*r1`, unless
its address is wanted, in which case it prints bare. A function name under
`AddrOf` becomes `Some(my_foo_0)`, a function pointer value. A variable
initialized with a lambda is not printed at all: the closure is re-emitted at
each use (see [Lambdas](../types/lambdas.md)).

## Refcount model

The cases, in the order the code checks them:

- A function under `AddrOf` becomes `FnPtr::<fn(AnyPtr) -> i32>::new(my_foo_0)`.
- A global's name becomes `name.with(Value::clone)` and then follows the same
  rules as a local (see [Global Variables](../declarations/globals.md)).
- A reference-typed variable is an unboxed `Ptr<T>`. Under `AddrOf` it prints
  bare (`r1`, not fresh, hence `r1.clone()` when stored); as an `Object` whose
  pointee is a boxed container it prints `r.to_strong().as_pointer()`; as an
  `LValue` it becomes a [pending dereference](./pending-deref.md); otherwise it
  is read, `r1.read()`, or, for a record pointee, `(*r1.upgrade().deref())`. A
  range-for variable over a map is exempt: it is registered in
  [`map_iter_decls_`](../internals/state.md) and prints bare, `i.second()`.
- Any other variable under `AddrOf` becomes `x.as_pointer()`.
- Under `RValue` it becomes `*x.borrow()`, and in every other kind
  `*x.borrow_mut()`.

Given `x = 0; x = y + 1;` and `++side_effect;` on a global:

```rust
*x.borrow_mut() = 0;
*x.borrow_mut() = *y.borrow() + 1;
(*side_effect_0.with(Value::clone).borrow_mut()).prefix_inc();
```

Given `int *pr1 = &r1;` and `r1 = rx2;` with `int &r1, &rx2`:

```rust
let pr1: Value<Ptr<i32>> = Rc::new(RefCell::new(r1.clone()));
let __rhs = rx2.read();
r1.write(__rhs);
```

The read of `rx2` is hoisted into `__rhs` because both sides of the assignment
go through a pointer to the same type, so `r1` and `rx2` may alias; see
[Assignment](./operators.md#assignment).
