# Pointers and References

The unsafe model keeps C++ pointers as raw pointers and dereferences them
directly. The refcount model replaces every pointer and reference with
[`Ptr<T>`](../../runtime/rc.md#values-and-pointers), a weak reference plus an
offset, and every dereference with a short-lived borrow of the pointee. Given

```cpp
int f(int *q) {
  int b = 2;
  int *p = &b;
  *p = *q;
  return b;
}
```

the unsafe model produces

```rust
pub unsafe fn f_0(mut q: *mut i32) -> i32 {
    let mut b: i32 = 2;
    let mut p: *mut i32 = &mut b as *mut i32;
    *p = *q;
    return b;
}
```

and the refcount model produces

```rust
pub fn f_0(q: Ptr<i32>) -> i32 {
    let q: Value<Ptr<i32>> = Rc::new(RefCell::new(q));
    let b: Value<i32> = Rc::new(RefCell::new(2));
    let p: Value<Ptr<i32>> = Rc::new(RefCell::new(b.as_pointer()));
    p.borrow().write(q.borrow().read());
    return *b.borrow();
}
```

The rest of the page goes through the pointer operations one at a time.

## Address-of

Unsafe model: `&x` becomes `&mut x as *mut T`, or `&x as *const T` when the
pointer type is to `const`. Globals use `&raw mut x` so no reference to the
`static` is formed. An array decays with `arr.as_mut_ptr()`, and the address of
an element is `&mut arr[i] as *mut T`.

Refcount model: `&x` becomes `x.as_pointer()`, which produces a `Ptr` holding a
weak reference to the variable's `Value`. Since every field is its own `Value`,
`&s.field` is `s.field.as_pointer()`. An array decays with
`arr.as_pointer() as Ptr<T>`, a `Ptr` to element 0 of the whole array, and
`&arr[i]` is that pointer offset by `i`. Taking the address is pushed down to
the innermost place expression: `&(cond ? x : y)` becomes
`if cond { x.as_pointer() } else { y.as_pointer() }`. Rust does not allow
`as_pointer()` on the result of the `if`, because that result is a value copied
out of whichever branch ran, not the branch's storage, so `as_pointer()` has to
be applied inside each branch, where the place is still known.

## Dereference

Unsafe model: `*p` stays `*p`, `p->x` becomes `(*p).x`, and an assignment
through a pointer is `*p = v`.

Refcount model: a dereference cannot hand out a `&T` into the pointee, because
nothing would bound the borrow's lifetime, so a read copies the value out and a
write copies it in:

- Reading a scalar or pointer pointee is `p.read()`.
- Writing is `p.write(v)`.
- A compound assignment `*p += v` becomes
  `{ let _ptr = p.clone(); _ptr.write(_ptr.read() + v) }`.
- Reading or writing a field of a record pointee goes through
  `p.upgrade().deref()`, which briefly turns the weak pointer into a
  [strong one](../../runtime/rc.md#strong-pointers) and borrows the record; the
  field is then a `Value` and is borrowed as usual: `(*p.upgrade().deref()).x`.
  The strong pointer only ever appears as a temporary inside the expression,
  which is what breaks field writes and field addresses through reinterpreted
  pointers ([#309](https://github.com/Cpp2Rust/cpp2rust/issues/309)) and union
  accessors ([#311](https://github.com/Cpp2Rust/cpp2rust/issues/311)).
- Calling a method on a pointee whose type is itself boxed, such as a
  `Ptr<Vec<T>>` receiver, uses `p.to_strong().as_pointer()` to reach the owning
  cell, and mutation then goes through [`with_mut`](../../rules/rewriting.md).

Which form is emitted is decided by the expression kind (`ExprKind`, what the
enclosing construct expects from the expression):

- An rvalue use of `*p` prints `p.read()`.
- An address-of use of `*p` prints `p` itself.
- An lvalue use does not print anything at once. The converter records the
  pointer expression as a [pending dereference](../pending-deref.md), and
  whoever consumes the lvalue, the assignment or a mapped method call, emits
  `p.write(...)` or `p.with_mut(...)` around it. This is what lets `*p = v` come
  out as a single `write` instead of a borrow followed by an assignment.

## Arithmetic and comparison

Unsafe model: `p + n` is `p.offset(n as isize)` and `p - n` is
`p.offset(-(n as isize))`; `p - q` is
`(p as usize - q as usize) / ::std::mem::size_of::<T>()`; `++p` is
`p.prefix_inc()` through the [increment traits](../../runtime/inc-dec.md);
`p == NULL` is `p.is_null()` and the null literal is `std::ptr::null_mut()`, or
`std::ptr::null()` for a pointer to `const`.

Refcount model: the same operations on `Ptr`, `p.offset(n as isize)`,
`p.clone() - q.clone()` (subtraction takes its operands by value, hence the
clones), `p.prefix_inc()`, `p.is_null()`, and `Ptr::null()`. Arithmetic only
moves the offset; whether the result is in bounds is checked when it is
dereferenced.

## References

A C++ reference is a pointer that cannot be reseated, and both models translate
it as one. In the unsafe model a reference parameter is `*mut T` (`*const T` for
`const T &`), an argument `f(x)` is `f(&mut x as *mut T)`, and uses of the
reference are `*r`. In the refcount model it is a `Ptr<T>` that is
[never boxed](./boxing.md): the argument is `f(x.as_pointer())`, uses are
`r.read()` and `r.write(v)`, and returning a reference returns the `Ptr` (with a
`.clone()`, since `Ptr` is not `Copy`).

## Heap

`new T(v)` becomes `Box::leak(Box::new(v)) as *mut T` in the unsafe model and
[`Ptr::alloc(v)`](../../runtime/rc.md#the-heap) in the refcount model;
`delete p` becomes `::std::mem::drop(Box::from_raw(p))` and `p.delete()`. Array
forms use a boxed slice and `Ptr::alloc_array`. In the refcount model the heap
allocation is a leaked `Rc` that `delete` recovers, so a double `delete` or a
`delete` of something that was not allocated with `new` panics instead of
corrupting memory.
