# printf and Streams

## printf

`printf` and `fprintf` have no translation rule; `ConvertCallExpr` recognizes
them by name and calls `ConvertPrintf`. The unsafe model prints a call to libc's
`printf` with the format cast to `*const i8`:

```rust
printf(c"%s\n".as_ptr() as *const i8, c"fprintf stdout".as_ptr());
```

The stream argument of `fprintf` is dropped, so it prints `printf` for every
stream.

The refcount model rewrites the call into a formatting macro. The format must be
a string literal and the stream must be `stdout` or `stderr` (or their macOS
spellings `__stdoutp`, `__stderrp`); anything else stops the translation.
`printf2fmt` rewrites the conversions it knows: `%d %i %u %s`,
`%ld %lu %lld %llu %zd %zu` become `{}`, `%p` becomes `{:?}`, `%c` becomes `{}`
with the argument cast `as u8 as char`, a width form `%5d`, `%5x`, `%5zu`
becomes `{:5}`, `{:5x}`, `{:5}`, and a precision form `%.03f` becomes `{:.3}`.
Any other conversion, `%f` or `%x` without a width included, is an assertion
failure. A trailing `\n` selects the `ln` variant:

```rust
println!("{} {} {}", 1, 2_u32, 3_i64);
print!("{:?} {}", *p.borrow(), (*p.borrow()).read());
eprintln!("{}", Ptr::from_string_literal(b"error"));
```

Functions that format into a buffer, `snprintf` and the like, are not rewritten
this way; they are rules, passthroughs to libc in the unsafe model and
`format_c` calls in the refcount model (see
[Formatting](../../runtime/io.md#formatting)).

## C++ streams

An `operator<<` whose result is a `basic_ostream` is detected by
`IsCallToOstream`, and `ConvertCallToOstream` flattens the chain
`os << a << b << std::endl` into a list. The stream itself is converted by
`ConvertStream`, which yields the `std::cout` rule's expression (a
`std::fs::File` over the descriptor in the unsafe model, `libcc2rs::cout()` in
the refcount model) or the converted variable for an `std::ostream &`.

The list is then emitted as runs of two kinds. String and character literals,
`std::endl` (a `\n`), the manipulators `std::hex` and `std::dec`, and any value
that is not a `char` or a `std::string` are accumulated into a format string and
flushed as `write!(stream, "fmt", args,);`, a value printing as `{}` or `{:x}`
after `std::hex`. `char` values and `std::string` values cannot go through `{}`,
since they are `u8` and `Vec<u8>` on the Rust side, so they are written as byte
slices, `stream.write_all(&([...].concat()));`. Given
`std::cout << i << " a"; std::cout << std::hex << 27;` the refcount model
produces

```rust
write!(libcc2rs::cout(), "{:} a", *i.borrow(),);
write!(libcc2rs::cout(), "0x{:x}", 27,);
```

`std::cerr` and stream references work the same way,
`write!(*os1.borrow(), "hello\n",);`. Input streams (`std::cin >>`, `getline`)
and string streams have no support in the converter or the rules.
