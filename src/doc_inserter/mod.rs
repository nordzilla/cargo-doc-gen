//! Handles inserting documentation into files of a Rust project.

use crate::doc_gen::DocGenerator;

/// Consumes a list of documentation produced by a `DocGenerator`
/// and handles inserting documentation into the relevant files
/// at the relevant locations of a Rust project.
///
/// Must be able to gracefully merge newly generated documentation
/// with existing documentation, and highlight conflicts accordingly.
/// Exactly how to handle this process is still to be determined.
pub struct DocInserter;

impl From<DocGenerator> for DocInserter {
    fn from(_gen: DocGenerator) -> Self {
        todo!("https://github.com/nordzilla/cargo-doc-gen/issues/17")
    }
}
