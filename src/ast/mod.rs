//! Handles generating an abstract syntax tree of a Rust project.

/// An AST that represents a rust project in a given directory.
/// The intent is to reuse as much as possible from the existing Rust
/// parser from rustc. We may aggregate additional information or remove
/// information from indiviual tree nodes depending on our needs.
#[allow(unused)]
pub struct RustAst;
