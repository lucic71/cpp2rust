# Operators

## Assignment

Given `x = y + 1;`, `x = x + 1;`, and `x += z;` with `int &z`, the unsafe model
produces

```rust
x = y + 1;
x = x + 1;
x += *z;
```

and the refcount model produces

```rust
*x.borrow_mut() = *y.borrow() + 1;
let __rhs = *x.borrow() + 1;
*x.borrow_mut() = __rhs;
let __rhs = z.read();
*x.borrow_mut() += __rhs;
```

`ConvertAssignment` converts the left side as an lvalue and the right side as a
fresh rvalue with the left side's type as implicit conversion target. In the
refcount model the right side is converted first, and if
`MayCauseBorrowMutError` holds, that is, both sides mention the same variable,
or one side goes through a pointer or reference to the other side's type, the
right side is hoisted into `let __rhs = ...;` so that its `borrow()` ends before
the `borrow_mut()` of the left side starts. The assignment itself goes through
`EmitSetOrAssign`, which prints `lhs = rhs` or, when the left side is a
[pending dereference](./pending-deref.md), `ptr.write(rhs)`.

An assignment used as a value is wrapped in a block that ends by reading the
variable back: `q = { p = p.wrapping_add(1 as usize); p };` and
`*q.borrow_mut() = { *p.borrow_mut() += 1; (*p.borrow()).clone() };`.

A compound assignment whose C++ computation type differs from the variable's
type (integer promotion of a `uint8_t`) is spelled out:
`flags = (flags as i32 | 1 << 0) as u8`, and in the refcount model
`{ let rhs_0 = ...; *flags.borrow_mut() = rhs_0 }`. Unsigned `+= -= *= /= %=`
become `x = x.wrapping_add(rhs)` and the like, since Rust panics on unsigned
overflow in debug builds where C wraps.

## Arithmetic and comparison

The unsafe model parenthesizes every operand, `((a) + (b))`, and otherwise
prints the operator as is; the refcount model prints `(a + b)`. Unsigned
`+ - * / %` become `a.wrapping_add(b)` etc. (`ConvertUnsignedArithOperand` casts
an operand to the operation type when its Rust type differs). Each operand
receives the sibling's or the operation's type as implicit conversion target
through `GetOperandImplicitConversionTarget`, which is what inserts `as usize`
between `size_t` and `unsigned long` (see
[Casts](../types/casts.md#implicit-conversions-to-usize-and-isize)). Signed
division, modulo, shifts, and bitwise operators have no special handling.

In the refcount model `ConvertGenericBinaryOperator` splits an operation into
`{ let _lhs = a; _lhs op b }` when both sides have variables, they are not the
same variables, and one side goes through a pointer or reference: two live
borrows in one expression could otherwise conflict. `int x6 = *p1 + x3 + 5;`
becomes

```rust
let x6: Value<i32> = Rc::new(RefCell::new(
    ({
        let _lhs = (*p1.borrow()).read();
        _lhs + *x3.borrow()
    } + 5),
));
```

Pointer arithmetic, pointer difference, and null comparisons are on
[Pointers and References](../types/pointers.md#arithmetic-and-comparison).

## Conditions and logical operators

Rust conditions must be `bool`. In C++ they already are: clang inserts the
conversion to `bool` in the AST, so `ConvertCondition` prints the expression as
is. In C there is no `bool`, a condition is an integer or a pointer compared
against zero, and comparisons and logical operators themselves yield `int`.
`ConvertCondition` therefore first passes the condition through
`NormalizeToBool`, which leaves a `bool` alone, turns a pointer into
`!p.is_null()`, an integer or enum into `x != 0`, and rebuilds `!x` on an
integer as a boolean `!(x != 0)`. `&&` and `||` normalize each side the same
way. Because a C comparison or logical operator has type `int`, its result is
cast back, `(a < b) as i32`, and a condition on such a value compares it again:
`if (n && p)` in C is `if (n != 0 && !p.is_null()) as i32 != 0`.

## Comma

`a, b` prints as `a; b` with `a` converted as `Void`; parenthesized, it becomes
a block: `int y = (x = 2, x + 1);` is `let mut y: i32 = { x = 2; x + 1 };`.

## Increment and decrement

`++x`, `x++`, `--x`, `x--` become method calls on the
[increment traits](../../runtime/inc-dec.md): `out.prefix_dec()`,
`x.postfix_inc()`. The refcount model converts the operand as an lvalue,
`(*x.borrow_mut()).postfix_inc()`, and through a pointer uses
`ptr.with_mut(|__v| __v.prefix_inc())`. Enums get the same methods from
`impl_enum_inc_dec!`.

## Other unary operators

`~x` prints as `!x`. `!x` is a condition, see above. Unary minus prints `-`
before the operand, with the literal cases described in
[Literals](./literals.md). `sizeof(T)` becomes `::std::mem::size_of::<T>()`; in
the refcount model, where a record's Rust size differs from its C size, it is
the C size as a literal, `8usize`. `alignof` is not translated: the visitor logs
it as unsupported and emits nothing. `&` and `*` are on
[Pointers and References](../types/pointers.md).
