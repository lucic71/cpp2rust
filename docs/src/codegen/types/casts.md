# Pointer Casts

Given

```cpp
uint32_t value = 0x04030201;
uint8_t *bytes = (uint8_t *)&value;
void *any = bytes;
uint8_t *back = (uint8_t *)any;
```

the unsafe model produces

```rust
let mut value: u32 = 67305985_u32;
let mut bytes: *mut u8 = (&mut value as *mut u32) as *mut u8;
let mut any: *mut ::libc::c_void = bytes as *mut ::libc::c_void;
let mut back: *mut u8 = any as *mut u8;
```

and the refcount model produces

```rust
let value: Value<u32> = Rc::new(RefCell::new(67305985_u32));
let bytes: Value<Ptr<u8>> =
    Rc::new(RefCell::new(value.as_pointer().reinterpret_cast::<u8>()));
let any: Value<AnyPtr> = Rc::new(RefCell::new((*bytes.borrow()).to_any()));
let back: Value<Ptr<u8>> =
    Rc::new(RefCell::new((*any.borrow()).reinterpret_cast::<u8>()));
```

## Unsafe model

Every pointer cast, whether written as a C cast, `static_cast`, or
`reinterpret_cast`, is a Rust `as` between raw pointer types. Casts that change
nothing in Rust (a `typedef` to its underlying type, an implicit `T *` to
`const T *`) are not emitted. Casts to and from integers are also `as`.

## Refcount model

A `Ptr<T>` is a weak reference to a `RefCell<T>`, so it cannot simply be
relabeled as a `Ptr<U>`: the cell it points to holds a `T`. A cast to another
pointee type therefore produces a different kind of pointer, one that views the
allocation as bytes. Three helpers from the runtime cover the cases:

- `p.reinterpret_cast::<U>()` produces a `Ptr<U>` of the
  [`Reinterpreted` kind](../../runtime/reinterpret.md#views-over-the-original-allocation):
  a byte-level view over the original allocation, with the offset counted in
  bytes. Reads and writes through it go through
  [`ByteRepr`](../../runtime/reinterpret.md#byterepr), which is why every record
  type gets a `ByteRepr` impl.
- `p.to_any()` erases the type into an [`AnyPtr`](../../runtime/void.md), the
  translation of `void *`, remembering the original type.
- `any.reinterpret_cast::<T>()` recovers a `Ptr<T>` from an `AnyPtr`: the
  original pointer if `T` is the type it was erased from, a byte view otherwise
  (see [AnyPtr casts](../../runtime/reinterpret.md#anyptr-casts)).

Two casts do not use these helpers. An array decaying to a pointer is spelled
`arr.as_pointer() as Ptr<T>`, where the `as` only names the pointer type. An
upcast from a derived class to an abstract base becomes
`(p.to_strong() as Value<dyn Base>).as_pointer_dyn()`, which is the ordinary
Rust unsizing coercion applied to the owning cell (see
[Virtual Classes](../../runtime/ptr-dyn.md)). Casts between pointers and
integers use the [integer cast](../../runtime/rc.md#integer-casts) API of `Ptr`.

Constness is dropped in a cast as everywhere else, so `const_cast` is a no-op.
`dynamic_cast` is not supported.

## Function pointers

Casting a function pointer to a different signature, which C code does to call
through a generic type, wraps the function in an adapter closure that converts
the arguments; see [Casts](../../runtime/fn-ptr.md#casts) on the runtime page.
Storing a function pointer in a `void *` uses `to_any()` like any other pointer.
