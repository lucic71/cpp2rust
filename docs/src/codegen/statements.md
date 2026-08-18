# Statements

A statement is converted by `Convert(clang::Stmt *)`, which traverses it with
the [expression kind](./expressions/kinds.md) set to `Void` and appends `;`
afterwards unless the statement ends in a block in Rust (`if`, loops, compound
statements, `case` and `default`). Compound statements convert their children in
order; a declaration statement converts each declarator and terminates it with
`;`. Statements print the same in both models except where a construct touches a
variable, so most pages in this chapter show one output and note the refcount
differences.

The chapter covers the top level of a translation unit and then each statement
kind: control flow, `switch`, `goto`, and `return` together with statement
expressions.
