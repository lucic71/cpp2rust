# Functions

Given

```cpp
static int scale(int x, const int &k) {
  x *= 2;
  return x;
}
```

the unsafe model produces

```rust
pub unsafe fn scale_0(mut x: i32, k: *const i32) -> i32 {
    x *= 2;
    return x;
}
```

and the refcount model produces

```rust
pub fn scale_0(x: i32, k: Ptr<i32>) -> i32 {
    let x: Value<i32> = Rc::new(RefCell::new(x));
    *x.borrow_mut() *= 2;
    return *x.borrow();
}
```

`VisitFunctionDecl` converts a function only when it is a definition
(`IsConvertibleFunctionDecl`: it has a body); prototypes and `extern`
declarations emit nothing, and a definition seen from a header is emitted once
(see [Translation Unit](../statements/translation-unit.md)). A function template
is converted once per specialization the program uses, each with its own numeric
suffix (see [Naming](../types/naming.md)).

## Signature

The name comes from `GetNamedDeclAsString`, which appends the `_N` suffix. C++
`static` does not restrict the visibility: free functions are always `pub`.
`main` is the one exception, see [`main`](./main.md). Overloaded operators
defined outside a class are supported only for `operator<`, which is emitted as
`lt` and followed by the comparison trait impls described in
[Traits](../types/traits.md#comparison). The unsafe model marks every function
`unsafe fn`, because bodies dereference raw pointers; the refcount model emits
plain `fn`.

Parameters are printed by `ConvertVarDeclSkipInit`, the same routine that prints
local variables, with [`in_function_formals_`](../internals/state.md) set so
that the refcount model prints the type unboxed (see
[Boxing](../types/boxing.md)). In the unsafe model a parameter is `mut` unless
its C++ type is `const` or a reference; a reference parameter is a pointer that
the body never reseats. An unnamed parameter is `_`. A variadic function gets a
trailing `__args: &[VaArg]` parameter (see
[Variadic Arguments](../../runtime/va-args.md)). The return type is omitted for
`void`.

A parameter with a default argument is typed `Option<T>`, and the function
preamble unwraps it:

```cpp
int add(int x, int y = 1);
```

```rust
pub unsafe fn add_0(mut x: i32, mut y: Option<i32>) -> i32 {
    let mut y: i32 = y.unwrap_or(1);
    // ...
}
```

At the call site every argument for such a parameter is wrapped in `Some`, and
an omitted argument is passed as `None`: `add(5, 2)` becomes `add_0(5, Some(2))`
and `add(5)` becomes `add_0(5, None)`, so the `unwrap_or` in the preamble
applies the default.

## Preamble and body

`EmitFunctionPreamble` runs before the body. In the unsafe model it only emits
the `unwrap_or` line for parameters with defaults. In the refcount model it also
boxes every named parameter that is not a reference into a `Value`, so that the
body can treat parameters like locals; the parameter names are taken from the
definition, not from a possibly differently-named prototype in a header.

The body is then converted by `ConvertFunctionBody`, which is where `goto`
detection happens (see [Goto and Hoisting](../statements/goto.md)) and where a
non-`void` function whose last statement is not a `return` gets a trailing
`panic!("ub: non-void function does not return a value")` (see
[Return and Statement Expressions](../statements/return.md)).
