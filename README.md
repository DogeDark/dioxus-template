# Overview

**This repo is not intended to be `git cloned`**.

This repo is used by `dx new` when starting new projects. The CLI relies on [cargo-generate](https://crates.io/crates/cargo-generate) to create and use these templates.

### Organization

This repository is organized into three sub-templates:
- `Barebones` - A single file barebones Dioxus project.
- `Jumpstart` - A more structured jumpstart with a starting file structure.
- `Workspace` - A full workspace with a crate for shared functionality and every platform.

All templates support every platform, router, and fullstack.

### Local Development
To run and test the templates without `dx`, you can run `cargo generate --path myPathToTemplates`