# Members and Subscripts

## Field access

Given `ptr->x = 2;`, `int r = c ? obj.x : ptr->x;`, and `int *p = &obj.x;`, the
unsafe model produces

```rust
(*ptr).x = 2;
let mut r: i32 = if c { obj.x } else { (*ptr).x };
let mut p: *mut i32 = &mut obj.x as *mut i32;
```

and the refcount model produces

```rust
*(*(*ptr.borrow()).upgrade().deref()).x.borrow_mut() = 2;
let r: Value<i32> = Rc::new(RefCell::new(if *c.borrow() {
    *(*obj.borrow()).x.borrow()
} else {
    *(*(*ptr.borrow()).upgrade().deref()).x.borrow()
}));
let p: Value<Ptr<i32>> = Rc::new(RefCell::new((*obj.borrow()).x.as_pointer()));
```

`ConvertMemberExpr` first tries a rule for the whole member expression (that is
how `it->second` and libc struct fields are handled, see
[Applying Rules](./rules.md)), then converts the base and appends `.name`. In
the unsafe model `->` dereferences the base, `(*ptr).x`; `this->x` and `x`
inside a method are `self.x`, and inside a constructor `this.x`, the local being
built. Under `AddrOf` the whole access is prefixed with `&`. A reference-typed
field is a pointer and is dereferenced on use, `(*f2.y)`.

In the refcount model every field is a `Value`, so the base is converted as an
rvalue and the field is then borrowed like a variable: `*base.x.borrow()` for a
read, `.borrow_mut()` for a write, `.x.as_pointer()` under `AddrOf` (see
[Variable References](./variables.md)). A pointer base becomes
`(*p.borrow()).upgrade().deref()`, briefly turning the weak pointer into a
strong one to reach the record (see
[Pointers and References](../types/pointers.md#dereference)). The base of a
method call is borrowed immutably, `(*c.borrow()).get()`, even for a non-`const`
method, since the fields of a user class carry their own cells; `NeedsMutAccess`
asks for `borrow_mut()` when the call is a mutating library method such as
`push`, `(*v.borrow_mut()).push(10)`. A union member becomes an accessor call,
`(*u.borrow_mut()).i().write(42)` (see [Unions](../types/unions.md)).

Members of an anonymous struct or union are reached through the implicit field
clang inserts, named `anon_N` like its type: `o.e = 7` becomes `o.anon_2.e = 7`.
Static data members are globals and static methods `Record::method`, both
`DeclRefExpr`s (see [Variable References](./variables.md)).

## Subscripts

Given `arr1[i] = i + arr2[i];` on C arrays and `out += ptr[i];` on a pointer,
the unsafe model produces

```rust
arr1[i as usize] = i + arr2[i as usize];
out += *ptr.offset(i as isize);
```

and the refcount model produces

```rust
let __rhs = *i.borrow() + (*arr2.borrow())[*i.borrow() as usize];
(*arr1.borrow_mut())[*i.borrow() as usize] = __rhs;
let __rhs = (*ptr.borrow()).offset(*i.borrow() as isize).read();
*out.borrow_mut() += __rhs;
```

`VisitArraySubscriptExpr` translates `p[i]` differently depending on whether `p`
is a pointer or an array. When it is a pointer, `ConvertPointerSubscript` emits
pointer arithmetic, `(*p.offset(i as isize))`, or just `p.offset(i as isize)`
under `AddrOf`; in the refcount model the offset is read with `.read()`
(`.upgrade().deref()` for a record) or, as an lvalue, recorded as a
[pending dereference](./pending-deref.md). When it is an array, the result is a
Rust index, `arr[i as usize]`; the refcount model borrows the array first, and
for a two-dimensional array inserts a `.borrow()` between the two subscripts,
since each row is its own `Value`:
`(*grid.borrow())[2 as usize].borrow_mut()[5 as usize]`. Taking the address of
an element gives `(arr.as_pointer() as Ptr<i32>).offset(0)`.

A flexible array member is not indexed, which Rust would bounds-check against
the declared length, but offset from its start: `s.bytes[i]` becomes
`*s.bytes.as_mut_ptr().add(i as usize)` (see
[Flexible array members](../types/classes.md#flexible-array-members)). A
`unique_ptr<T[]>` is `x.as_mut().unwrap()[i as usize]`.

`operator[]` of `std::vector` and `std::string` is not a rule; the operator call
is converted like an array subscript in the unsafe model, `v[0_usize]`, with
`(&mut (*p))[...]` when the container is reached through a pointer. In the
refcount model it becomes pointer arithmetic on the container's element pointer,
`(v2.as_pointer() as Ptr<i32>).offset(0_usize).read()`, and an lvalue use
records the offset as a pending dereference,
`(v2.as_pointer() as Ptr<i32>).offset(0_usize).write(1)`. This is what makes
`v[i] = x` a single `write` instead of a mutable borrow of the vector held
across the assignment.
