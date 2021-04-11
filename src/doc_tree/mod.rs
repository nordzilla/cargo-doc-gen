//! Handles generating a tree of documentable Rust code objects.

/// The `DocTree` is an AST-like structure produced from a `RustAst`.
///
/// The difference between a `DocTree` and a `RustAst` is that the `DocTree`
/// will contain only items that are documentable by `doc-gen`.
///
/// Some items will be optionally included in the `DocTree` based on the `DocConfig`.
///
/// The DocTree can be consumed by a `DocGenerator` to produce
/// documentation comments.
pub struct DocTree;

impl From<syn::File> for DocTree {
    fn from(_file: syn::File) -> Self {
        todo!("https://github.com/nordzilla/cargo-doc-gen/issues/4")
    }
}
