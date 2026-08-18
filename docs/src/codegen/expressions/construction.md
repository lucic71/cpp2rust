# Construction, `new` and `delete`

## `new` and `delete`

Given `int *d = new int(0); delete d;` and `int *e = new int[2]; delete[] e;`,
the unsafe model produces

```rust
let mut d: *mut i32 = Box::leak(Box::new(0)) as *mut i32;
::std::mem::drop(Box::from_raw(d));
let mut e: *mut i32 =
    Box::leak((0..2_usize).map(|_| 0_i32).collect::<Box<[i32]>>()).as_mut_ptr();
::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
    e,
    libcc2rs::malloc_usable_size(e as *mut ::libc::c_void)
        / ::std::mem::size_of::<i32>(),
)));
```

and the refcount model produces

```rust
let d: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc(0)));
(*d.borrow()).delete();
let e: Value<Ptr<i32>> = Rc::new(RefCell::new(Ptr::alloc_array(
    (0..2_usize).map(|_| <i32>::default()).collect::<Box<[i32]>>(),
)));
(*e.borrow()).delete_array();
```

`VisitCXXNewExpr` leaks a `Box` in the unsafe model and allocates through
[`Ptr::alloc`](../../runtime/rc.md#the-heap) in the refcount model. The
initializer is converted like a variable initializer: `new Pair{1, 2}` gives
`Box::new(Pair { x: 1, y: 2 })`, `new Pair()` the type's default value (a bare
`new T` with no initializer is `Ptr::alloc(Default::default())` in the refcount
model), and `new T[n]` fills `n` copies of the
[default value](../declarations/defaults.md) (`new int[3]{7, 8}` becomes
`Box::new([7, 8, 0_i32])`, padded to the size). The size expression is converted
as written, `(*N2) as usize` for a reference. In the unsafe model the
`.as_mut_ptr()` is only appended when the result goes into a pointer, so
`unique_ptr<int[]>(new int[100])` gets the `Box<[i32]>` its rule expects.
Placement `new` is not handled.

`VisitCXXDeleteExpr` frees with `Box::from_raw`. For `delete[]` the unsafe model
has no length, so it recovers one from the allocator with `malloc_usable_size`
before rebuilding the slice; the refcount model's `delete()` and
`delete_array()` know their allocation.

## Constructor calls

`VisitCXXConstructExpr` handles, in order:

- A library constructor with a rule: `std::string s(10, 'a')` becomes
  `vec!['a' as u8; 10_usize as usize]`, `std::vector<int>(10)` becomes
  `(0..10_usize as usize).map(|_| <i32>::default()).collect::<Vec<_>>()`.
- A copy or move constructor: the argument, with `.clone()` for a copy of
  something not [fresh](./kinds.md), `let mut ww: X = xx.clone();` and
  `Rc::new(RefCell::new((*xx.borrow()).clone()))`; a move converts the argument
  as an lvalue and takes it.
- An implicit default constructor: the type's default value, `<X>::default()`.
- A user constructor: `Name::Name(args)`, or `Name::Name1(args)` when the class
  has several (see [Methods and Constructors](../declarations/methods.md)), each
  argument converted like an initializer of the parameter type, `Some(...)`
  around arguments for parameters with defaults and `None` for omitted ones. An
  array of such objects is
  `std::array::from_fn::<_, N, _>(|_| Name::Name(...))`.

## Initializer lists

`VisitInitListExpr` works on clang's semantic form, where designated
initializers are reordered, braces elided, and missing members filled with
`ImplicitValueInitExpr`, which prints the default value. So:

- A scalar `int i{3}` is `3`, `int i{}` is `0_i32`.
- A record is a struct literal with every field in declaration order,
  `Point { x: 10, y: 20 }` and
  `Point { x: Rc::new(RefCell::new(10)), y: Rc::new(RefCell::new(20)) }`;
  `struct Layout v = {0};` fills the rest,
  `Layout { a: 0_u8, b: 0_u32, c: 0_u16 }`, and a `0` in a pointer position is a
  null pointer.
- An array is `[1, 0_i32, 0_i32]` for `int a[3] = {1}`, `Box::new([...])` in the
  refcount model, and rows of a two-dimensional array are boxed individually,
  `Box::new([Rc::new(RefCell::new(Box::new([0, 1, 2, 3]))), ...])`.
- A `char` array from a string literal, `char s[6] = "hello"`, is
  `std::mem::transmute(*b"hello\0")` and `Box::from(*b"hello\0")` (see
  [Literals](./literals.md)).
- `std::array<int, 3> a = {1, 2, 3}` is `vec![1, 2, 3]` (see
  [`std::array`](../types/special-types.md#stdarray)).
- A braced list passed where `std::initializer_list<T>` is expected, or given to
  a container constructor, goes through `VisitCXXStdInitializerListExpr` and is
  `vec![1, 2, 3, 4]`.

A C compound literal is handled only for a transparent union such as glibc's
`__SOCKADDR_ARG`, where the single initializer is emitted. In
`getsockname(fd, (struct sockaddr *)&ssloc, &slen)`, glibc's parameter type is
`__SOCKADDR_ARG` and clang wraps the argument in
`(__SOCKADDR_ARG){ (struct sockaddr *)&ssloc }`, which is printed as
`((&mut ssloc as *mut libc::sockaddr_storage) as *mut libc::sockaddr)`. A
functional cast `Foo(x)` with one argument, `const_cast`, and `dynamic_cast` are
transparent and print their operand.
