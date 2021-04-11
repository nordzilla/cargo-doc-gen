//! Handles inserting documentation into files of a Rust project.

/// Consumes a list of documentation produced by a `DocGenerator`
/// and handles inserting documentation into the relevant files
/// at the relevant locations of a Rust project.
///
/// Must be able to gracefully merge newly generated documentation
/// with existing documentation, and highlight conflicts accordingly.
/// Exactly how to handle this process is still to be determined.
#[allow(unused)]
pub struct DocInserter;
