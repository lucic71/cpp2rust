# Literals

## Integers and floats

An integer literal prints bare when its type maps to `i32`, and with a Rust
suffix otherwise: `0`, `5`, `4_u32`, `5_u64` (`getTypedLiteral`; a type spelled
with a path prints as a cast, `(0 as libc::c_char)`). An implicit integer
conversion applied to a literal is folded into the suffix, so `size_t sz = 20;`
is `20_usize` and `double sum = 0;` is `0_f64`. A negated unsigned literal is
written through a signed one, `(-1_i64 as u64)`, and a negated literal assigned
to a narrower type is cast, `(-8_i32 as i8)`. Floating literals print in
exponent form as clang's `APFloat` renders them: `3.14E+0`.

## Characters and booleans

A character literal is `'a' as libc::c_char` in the unsafe model and `'a' as u8`
in the refcount model; C code, where `'a'` is an `int`, gets
`'a' as i32 as libc::c_char`. A byte above `0x7F` is a byte literal,
`b'\xff' as libc::c_char`. `true`/`false` print as such; a C `bool b = false;`
goes through the `0` macro and becomes `0 != 0`.

## Strings

A string literal is a C string in the unsafe model and a byte string in the
refcount model, and the context it appears in decides the rest:

| Context                    | Unsafe model                                      | Refcount model                        |
| -------------------------- | ------------------------------------------------- | ------------------------------------- |
| decay to `const char *`    | `c"hello".as_ptr()`                               | `Ptr::from_string_literal(b"hello")`  |
| `char s[9] = "papanasi"`   | `std::mem::transmute(*b"papanasi\0")`             | `Box::from(*b"papanasi\0")`           |
| `char s[N] = ""`           | `[0 as libc::c_char; N]`                          | `vec![0u8; N].into_boxed_slice()`     |
| interior NUL, `"\x01\x00"` | `(&[(1 as libc::c_char), ...]).as_ptr()`          | `Ptr::from_string_literal(b"\x01\0")` |
| `"abcd"[idx]`              | `(*c"abcd".as_ptr().offset(idx as isize)) as i32` | `b"abcd"[idx as usize] as i32`        |

The decay adds `.cast_mut()` when the destination is not `const`, the array form
is padded to the declared size, and the interior-NUL byte array lists every
byte, since a `c"..."` literal cannot contain a NUL.

`__func__`, `__FILE__`, and `__LINE__` reach the converter already expanded by
clang and print as the literals they expand to. `std::string s = "..."` is
handled by the string rules, not here.

## Null

`nullptr` and `NULL` arrive wrapped in a null-to-pointer cast, which prints the
[default value](../declarations/defaults.md) of the target type:
`std::ptr::null_mut()` / `std::ptr::null()` and `None` for a function pointer in
the unsafe model; `Ptr::<i32>::null()`, `AnyPtr::default()` for `void *`, and
`FnPtr::<fn(AnyPtr) -> i32>::null()` in the refcount model. A bare `nullptr`
with no cast around it, which only happens for a default argument, prints
`Default::default()`.
