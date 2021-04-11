//! Handles parsing Rust documentation.
//!
//! This will likely have to be a custom parser due to the functionality
//! functionality that is unique to this crate. Doc comments will be parsed
//! with more context and semantics than what rustc cares about.

/// Parses Rust documentation into a structure that can be
/// compared against `DocTree` nodes and the `DocConfig`.
/// This will be used by `doc-gen --check` in order to validate
/// that the documentation is both complete and up-to-date.
#[allow(unused)]
pub struct DocParser;
