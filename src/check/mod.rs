//! The main entry point for the `doc-gen --check` command option.
//! Handles validating that existing documentation in a rust project
//! conforms to the specifications set by `DocConfig`.
//!
//! Should also be capable of determining if documentation is out of date
//! or incomplete.
//!
//! An example of documentation being out of date would be if `DocConfig`
//! has specified that the arguments to functions should be enumerated
//! with descriptions, but the function arguments and/or arg names have
//! changed since it was last documented.
//!
//! Examples of documentation being incomplete would be an empty examples
//! section, or unfilled descriptions for enumerated function arguments.
