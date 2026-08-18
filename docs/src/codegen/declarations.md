# Declarations

A declaration is converted by `Convert(clang::Decl *)`, which hands it to the
`Visit*Decl` method for its kind. Which declarations of a unit are visited at
all is decided by the [Translation Unit](./statements/translation-unit.md) walk;
records, enums, and unions are covered in [Types](./types.md). This chapter
covers the rest: functions, `main`, methods and constructors, and variables,
local and global, together with the default value a variable gets when it has no
initializer.

Typedefs and `using` aliases emit nothing. `VisitTypedefDecl` returns without
printing, and every use of the alias is desugared when the type is printed
(`VisitTypedefType`), unless a [type rule](../rules/format.md#type-rules) maps
the alias name itself, as it does for `size_t`.

The two models share the emission code for declarations. `Converter` holds three
[keyword strings](./internals/state.md), `keyword_unsafe_`, `keyword_mut_`, and
`keyword_const_fn_`, which are `unsafe`, `mut`, and `const` (printed before
`unsafe fn` for a `constexpr` function) in the unsafe model; `ConverterRefCount`
sets all three to the empty string. That is why the same code prints
`pub unsafe fn f(mut x: i32)` in one model and `pub fn f(x: i32)` in the other,
and why refcount locals are never `mut`: the `Value` is what gets mutated,
through `borrow_mut`, not the binding.
