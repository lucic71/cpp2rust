# Variadic Functions

The runtime side, `VaArg`, `VaList`, and the promotions `.into()` performs, is
on [Variadic Functions](../../runtime/va-args.md) in the runtime part. This page
is what the converter emits.

## Definitions

`ConvertFunctionParameters` appends `__args: &[VaArg]` after the named
parameters of a function declared with `...`. A `va_list` variable is recognized
by `IsVaListType`, which looks for `va_list` in the typedef chain, and converted
by `ConvertVaListVarDecl` to `let mut ap: VaList` in the unsafe model and
`let ap: Value<VaList>` in the refcount model, initialized with
`VaList::default()`. Passing a `va_list` to another function passes the cursor
by value: `inner(count, ap)` is `inner_0(count, ap)` and
`inner_0(*count.borrow(), (*ap.borrow()).clone())`.

`ConvertVAArgCall` handles the three builtins: `va_start(ap, last)` becomes
`ap = VaList::new(__args)` (the `last` argument is ignored), `va_copy(dst, src)`
becomes `dst = src.clone()`, and `va_end` emits nothing. `VisitVAArgExpr` prints
`va_arg(ap, T)` as `ap.arg::<T>()`; a function pointer type is read as
`*mut c_void` and transmuted in the unsafe model, and read directly as
`FnPtr<fn(i32) -> i32>` in the refcount model.

## Calls

At a call to a variadic user function, `EmitArgList` prints the extra arguments
as a slice: `sum_0(3, &[10.into(), 20.into(), 30.into(),])`, or `&[]` when there
are none. A function pointer among them is erased to a raw pointer first in the
unsafe model,
`Some(square_0).map_or(::std::ptr::null_mut(), |f| f as *mut ::libc::c_void).into()`,
and passed as `FnPtr::<fn(i32) -> i32>::new(square_0).into()` in the refcount
model. A rule whose target takes a variadic tail gets the same slice at its
`va_args` fragment (see [Rules IR](../../rules/ir.md)).

Variadic libc functions such as `snprintf` and `fcntl` are libc passthroughs in
the unsafe model: the fixed arguments are cast to the rule's parameter types and
the extra ones are printed as they are,
`libc::snprintf(buf.as_mut_ptr() as *mut libc::c_char, ..., -3_i32, 7_u32,)`
(see [Calls](./calls.md)). Their refcount rules have bodies that take a
`&[VaArg]` and format with `format_c`, so the call is an ordinary rule
application. `printf` and `fprintf` are handled separately, see
[printf and Streams](./io.md).
