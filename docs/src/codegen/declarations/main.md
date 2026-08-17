# `main`

C++ `main` is emitted as `main_0` plus a Rust `main` that calls it. Given

```cpp
int main(int argc, char *argv[]) { return argc; }
```

the unsafe model produces

```rust
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe { ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32) }
}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut libc::c_char) -> i32 {
    return argc;
}
```

and the refcount model produces

```rust
pub fn main() {
    let argv: Vec<Value<Vec<u8>>> = ::std::env::args()
        .map(|x| Rc::new(RefCell::new(x.as_bytes().to_vec())))
        .collect();
    let mut argv: Value<Vec<Ptr<u8>>> = Rc::new(RefCell::new(
        argv.iter()
            .map(|x| {
                x.borrow_mut().push(0);
                x.as_pointer()
            })
            .collect(),
    ));
    (*argv.borrow_mut()).push(Ptr::null());
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(argc: i32, argv: Ptr<Ptr<u8>>) -> i32 {
    let argc: Value<i32> = Rc::new(RefCell::new(argc));
    let argv: Value<Ptr<Ptr<u8>>> = Rc::new(RefCell::new(argv));
    return *argc.borrow();
}
```

`VisitFunctionDecl` recognizes `main` with clang's `isMain()`, names it
`main_0`, and calls `ConvertFunctionMain` to print the wrapper first. The
wrapper is a fixed string per model, chosen by whether `main` takes parameters.
With no parameters it is one line:

```rust
pub fn main() { unsafe { std::process::exit(main_0() as i32); } }
```

With parameters it rebuilds a C `argv`: each argument from `std::env::args()` is
copied into a byte vector, NUL-terminated, and its pointer collected into a
vector that ends with a null pointer, so `argv[argc]` is null as C requires.
`argc` is the number of arguments; the unsafe wrapper computes it as
`argv.len() - 1`, the refcount wrapper reads `std::env::args().len()` again. The
refcount model boxes each argument buffer in a `Value` so that `as_pointer()`
can produce a `Ptr<u8>` into it, and passes `argv.as_pointer()` as the
`Ptr<Ptr<u8>>`. In the refcount model `ConvertFunctionParameters` also
special-cases `main`, printing the two parameters as `i32` and `Ptr<Ptr<u8>>`
regardless of how they were spelled.

The return value becomes the process exit status through `std::process::exit`.
`main_0` itself is not `pub` (`ConvertFunctionQualifiers` is skipped for it),
since the wrapper is the entry point.
