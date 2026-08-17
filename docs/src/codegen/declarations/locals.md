# Local Variables

Given

```cpp
void f() {
  int a = 1;
  const int b = a;
  int c;
  int &r = a;
  int arr[2];
}
```

the unsafe model produces

```rust
pub unsafe fn f_0() {
    let mut a: i32 = 1;
    let b: i32 = a;
    let mut c: i32 = 0_i32;
    let r: *mut i32 = &mut a as *mut i32;
    let mut arr: [i32; 2] = [0_i32; 2];
}
```

and the refcount model produces

```rust
pub fn f_0() {
    let a: Value<i32> = Rc::new(RefCell::new(1));
    let b: Value<i32> = Rc::new(RefCell::new(*a.borrow()));
    let c: Value<i32> = <Value<i32>>::default();
    let r: Ptr<i32> = a.as_pointer();
    let arr: Value<Box<[i32]>> = Rc::new(RefCell::new(
        (0..2).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
    ));
}
```

A declaration statement (`VisitDeclStmt`) converts each declarator on its own
and ends it with `;`, so `int a, b;` becomes two `let`s. Each one goes through
`VisitVarDecl`, which dispatches to `ConvertVarDecl` for locals and to
`ConvertGlobalVarDecl` for file-scope and `static` locals (see
[Global Variables](./globals.md)).

## The binding

`ConvertVarDeclSkipInit` prints everything up to the `=`: `let`, then `mut` when
the model has one and the type is neither `const` nor a reference, then the name
and the type. The type is printed with `Convert(QualType)`; the refcount model
pushes `FullRefCount` first, so it comes out as `Value<T>`, except for
references, which are an unboxed `Ptr<T>` (see [Boxing](../types/boxing.md)). A
`const` local is `let b` without `mut` in the unsafe model, and the refcount
model never prints `mut` (see [Declarations](../declarations.md)). A variable
named `_` gets no `mut` either.

Two kinds of declaration take a different path before any of this. A local
initialized with a lambda is not emitted at all by the unsafe model, which
inlines the closure at every use (see [Lambdas](../types/lambdas.md)); the
refcount model declares it like any other variable. A `va_list` local is
declared as `VaList` (`Value<VaList>` in the refcount model) by
`ConvertVaListVarDecl` (see [Variadic Arguments](../../runtime/va-args.md)).

## The initializer

When the declaration has an initializer, `ConvertVarInit` converts it with the
declared type as the target:

- An ordinary variable gets an rvalue conversion; a reference or function
  pointer variable gets an address-of conversion.
- The declared type is the implicit conversion target, so an `int` initializer
  of a `size_t` variable is cast (see
  [Casts](../types/casts.md#implicit-conversions-to-usize-and-isize)).
- A reference initialized from a non-reference expression takes its address:
  `&mut a as *mut i32` and `a.as_pointer()` above.
- In the refcount model the value is then wrapped in
  `Rc::new(RefCell::new(...))`, unless the variable is a reference.
- A lambda initializer is emitted as the closure itself, wrapped in
  `FnPtr::new(...)` when the variable has function pointer type.

When there is no initializer, `ConvertVarDefaultInit` prints the type's
[default value](./defaults.md). C++ leaves `int c;` and `int arr[2];`
uninitialized; the translation zero-initializes them, `let mut c: i32 = 0_i32;`
and `let mut arr: [i32; 2] = [0_i32; 2];`.

Locals in a function that uses `goto` are declared up front and only assigned
where the C++ declaration was (`EmitHoistedInArmAssignment`); see
[Hoisting](../statements/goto.md#hoisting).
