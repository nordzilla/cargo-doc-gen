//! Handles generating documentation comments from a `DocTree`.

use crate::doc_config::DocConfig;
use crate::doc_tree::DocTree;

/// The `DocGenerator` is a type that consumes `DocTree` nodes
/// and produces a flattened list of formatted documentation comments,
/// along with the files and line numbers into which the comments should
/// be inserted. Comments are generated in a style determined by `DocConfig`.
pub struct DocGenerator;

impl DocGenerator {
    #[allow(dead_code)]
    pub fn new(_tree: DocTree, _cfg: DocConfig) -> Self {
        todo!("https://github.com/nordzilla/cargo-doc-gen/issues/5")
    }
}
