# Miscellaneous

Constructs that do not belong to a family of their own, and what the converter
does not handle.

## Compile-time queries

`sizeof(T)` becomes `::std::mem::size_of::<T>()`; in the refcount model, where a
record's Rust layout differs from its C layout, `RustSizeDivergesFromC` makes it
print the C size as a literal instead, `8usize`. `offsetof(T, f)` becomes
`::std::mem::offset_of!(T, f)` in the unsafe model, and the evaluated constant,
`4_usize`, in the refcount model. `__builtin_types_compatible_p` and other type
traits are evaluated by clang and printed as integers. `__builtin_constant_p(x)`
prints `1` when `x` is a constant expression and `0` otherwise. `alignof` is not
translated: the visitor logs it as unsupported and emits nothing.

## Builtins

`__builtin_expect`, `__builtin_ctz`, `__builtin_clz`, `__builtin_popcountl`,
`__builtin_bswap16/32/64`, `__builtin_mul_overflow`, and `__builtin_ia32_pause`
are ordinary [rules](../../rules/overview.md) in `rules/builtin`;
`__builtin_expect(e, 1)` is just `e`, `__builtin_ctz(x)` is
`x.trailing_zeros() as i32`. `__builtin_va_*` are handled by
[Variadic Functions](./variadic.md). Any other `__builtin_*` goes through the
generic call path as an ordinary function name.

## Small visitors

`this` prints as `self`, or `this` inside a constructor, where it is the local
being built. An in-class member initializer (`int x = 3;` in the class body)
reaches the constructor as a `CXXDefaultInitExpr` and converts to the stored
expression. A parenthesized expression prints as `(e)`, or `{ a; b }` when it
holds a comma operator. `?:` is `if c { a } else { b }`, with `&mut` on both
arms in the unsafe model when the result is used as an address and
`ConvertFresh` on both arms in the refcount model (see
[Conditional operator](../statements/return.md#conditional-operator)). A
`RecoveryExpr`, clang's placeholder for code that failed to type-check, dumps
the node and stops the translation.

## Not handled

The converter has no visitor for exceptions (`throw`, `try`/`catch`), `typeid`,
`noexcept`, `alignof`, C11 `_Generic`, atomics, or placement `new`.
`RecursiveASTVisitor` traverses through an unhandled node into its children, so
`throw e` prints `e` and a `try` block prints its body, with no error. The
constructs that are rejected explicitly, with an assertion, are: an out-of-line
overloaded operator other than `operator<`, a user-defined copy or move
constructor, an overloaded operator call other than `= * -> [] << ()` and `<`
without a rule, an unsigned binary operator outside `+ - * / %`, a global with
a user-defined default constructor and no initializer in the unsafe model, a
`printf` format the refcount rewrite does not know, and a function pointer cast
it cannot express.
