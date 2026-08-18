# Default Values

`GetDefaultAsString(QualType)` returns the Rust expression for a
zero-initialized value of a type. It is what a variable declared without an
initializer gets, what constructor fields not in the member initializer list
get, what hoisted variables are declared with, what the elements of `new T[n]`
start as, and what the hand-written `Default` impls of records use for their
fields (see [Default](../types/traits.md#default)).

The unsafe model tries these cases in order:

| Type                                     | Default                                                                 |
| ---------------------------------------- | ----------------------------------------------------------------------- |
| `va_list`                                | `VaList::default()`                                                     |
| `T[N]`                                   | `[d; N]` with `d` the default of `T`                                    |
| `T[N]`, `T` a non-`Copy` record          | `std::array::from_fn::<_, N, _>(\|_\| d)`                               |
| `T[]`                                    | the default of `T`                                                      |
| `std::array<T, N>`                       | `std::array::from_fn::<_, N, _>(\|_\| Default::default()).to_vec()`     |
| type with a rule that has an initializer | the rule's initializer, for example `Vec::new()`                        |
| `T *`, `T` not a function                | `std::ptr::null_mut()`, or `std::ptr::null()` for `const T *`           |
| function pointer                         | `None`                                                                  |
| `bool`                                   | `false`                                                                 |
| integer                                  | `0_i32`, `0_usize`, ... (typed literal)                                 |
| floating point                           | `0.0_f32`, `0.0_f64`                                                    |
| record, inside a `static` initializer    | a struct literal with each field's default (`EmitDefaultStructLiteral`) |
| POD record from a system header          | `unsafe { std::mem::zeroed::<T>() }`                                    |
| enum                                     | its first enumerator, `Color::RED`                                      |
| anything else                            | `<T>::default()`                                                        |

The struct literal case exists because a `static` initializer must be a constant
expression and `Default::default()` is not `const`; a record with a user-defined
default constructor cannot be zero-initialized this way and stops the
translation when it appears as a global without an initializer. The rule
initializer comes from the body of the type rule's function (see
[Type rules](../../rules/format.md#type-rules)); libc records use it for the
`zeroed()` form.

The refcount model overrides the array, pointer, and fallback cases and boxes
the result when the value is going into a `Value`:

| Type                       | Default                                     |
| -------------------------- | ------------------------------------------- |
| `T[N]`                     | `(0..N).map(\|_\| d).collect::<Box<[T]>>()` |
| `T *`, `T` not a function  | `Ptr::<T>::null()`                          |
| `void *`                   | `AnyPtr::default()`                         |
| function pointer           | `FnPtr::<fn(A) -> R>::null()`               |
| anything not handled above | `<T>::default()`                            |

Scalars, enums, and records all fall through to `<T>::default()`, where `T` is
the type as printed in the current position, so a local `int` without an
initializer is `<Value<i32>>::default()` and a pointer is
`Rc::new(RefCell::new(Ptr::<i32>::null()))`. There is no `static` special case:
`thread_local!` initializers run lazily and may call anything, so a global
record is simply `<T>::default()` even when that runs a user-defined
constructor.

In C++ a class-typed variable without an initializer is not uninitialized: clang
attaches an implicit call to the default constructor, and
`VisitCXXConstructExpr` prints a non-user-provided default constructor as the
type's default value. The result is the same expression, boxed as an
initializer: `Rc::new(RefCell::new(<Outer>::default()))` in the refcount model
and `<Outer>::default()` in the unsafe model.
