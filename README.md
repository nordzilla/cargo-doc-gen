# cargo-doc-gen
A Rust documentation generator

## A tentative list of functionality we need to support
- [ ] Parse CLI arguments for `doc-gen`.
- [ ] Make `doc-gen` runnable from cargo as `cargo doc-gen`.
- [ ] `cargo doc-gen` 
    - [ ] Check for a doc-gen config file in the project dir.
    - [ ] Parse a doc-gen config file into a `DocConfig`.
    - [ ] Parse Rust code into a `RustAst`.
    - [ ] Filter Rust AST nodes into `DocTree` nodes.
        - [ ] Determine which nodes are documentable from config.
        - [ ] Determine the file where the `DocTree` node exists.
        - [ ] Determine the line number where the `DocTree` node exists.
    - [ ] Generate a formatted doc comment from a `DocTree` node + `DocConfig`.
    - [ ] Insert a doc comment at the location of a `DocTree` node.
        - [ ] Gracefully handle merging existing documentation and newly generated documentation.
- [ ] `cargo doc-gen --check`
    - [ ] Check for a doc-gen config file in the project dir.
    - [ ] Parse a doc-gen config file into a `DocConfig`.
    - [ ] Parse Rust code into a `RustAst`.
        - [ ] Determine which nodes are documentable from config.
        - [ ] Determine the file where the `DocTree` node exists.
        - [ ] Determine the line number where the `DocTree` node exists.
        - [ ] Parse the existing doc comment for the `DocTree` node if it exists.
            - [ ] Determine if the existing comment is formatted correctly.
            - [ ] Determine if the existing comment is up to date.
            - [ ] Determine if the existing comment has no missing content.