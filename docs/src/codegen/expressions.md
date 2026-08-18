# Expressions

An expression is converted by `Convert(clang::Expr *, implicit_convert_to)`.
Unlike statements and declarations, most of the difference between the two
models lives here: the unsafe model prints an expression almost as it was
written, while the refcount model has to decide, for every variable and every
dereference, whether to `borrow()`, `borrow_mut()`, `read()`, `write()`, or take
a pointer. Two pieces of converter state drive that decision and are described
first: the [expression kind](./expressions/kinds.md), which is what the
enclosing construct wants from the expression, and the freshness of the result,
which says whether the printed Rust yields an owned value or names existing
storage.

The remaining pages go through the expression forms: variable references,
pending dereferences, literals, operators, calls and their arguments, the
temporaries the converter has to create, how a matched translation rule and the
plugins are applied, variadic functions, member access and subscripts,
construction with `new` and `delete`, and a page of miscellaneous constructs.
