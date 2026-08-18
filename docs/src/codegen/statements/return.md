# Return and Statement Expressions

## `return`

In a non-`void` function, `return` keeps its operand, which is converted the
same way as the initializer of a variable of the return type (`ConvertVarInit`):
the casts, clones, and boxing that apply to `T x = e;` apply to `return e;` too.

In a `void` function, `return e;` becomes `e; return;`, and `return;` stays as
it is.

A non-`void` function whose body does not end in a `return` gets
`panic!("ub: non-void function does not return a value")` appended, since Rust
requires a value on every path and C treats running off the end as undefined
behavior.

## Statement expressions

A GNU statement expression becomes a Rust block whose last expression is its
value. Given

```cpp
int x = ({
  int a = 1;
  int b = 2;
  a + b;
});
```

the unsafe model emits the last expression as the block's tail:

```rust
let mut x: i32 = {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    a + b
};
```

The refcount model binds the tail to a temporary first, so that the borrows in
it end before the block yields the value:

```rust
let x: Value<i32> = Rc::new(RefCell::new({
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let __result = *a.borrow() + *b.borrow();
    __result
}));
```

Multi-statement rule bodies are emitted as blocks of the same shape, which is
why such blocks also appear in the output of ordinary library calls.

## Conditional operator

`c ? a : b` becomes `if c { a } else { b }` with the condition normalized to
`bool`. Both branches are converted to the operator's type (an
`implicit_convert_to` target), and in the refcount model each branch is
converted fresh, so a non-`Copy` result is cloned. When the conditional is used
as an lvalue or an address (`&(c ? x : y)`) the address-of is pushed into each
branch, as described under [Pointers and References](../types/pointers.md).
