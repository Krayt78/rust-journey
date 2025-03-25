# Packages in Rust

A package is the highest level of code organization in Rust's module system. It represents a bundle of functionality that can be shared with others.

## What is a Package?

A package is a collection of crates that together provide a set of functionality. Specifically, a package:

- Contains one or more crates
- Must contain zero or one library crate
- Can contain any number of binary crates
- Must contain at least one crate (library or binary)

## Package Structure

The standard structure of a Rust package looks like this:

```
my_project/            // Package
├── Cargo.toml         // Package manifest
├── src/
│   ├── main.rs        // Binary crate root
│   ├── lib.rs         // Library crate root
│   └── bin/           // Additional binary crates
│       └── another_binary.rs
```

The `Cargo.toml` file is the package manifest, which contains metadata about the package and its dependencies.

## Cargo.toml

The `Cargo.toml` file defines your package and its dependencies:

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A brief description of your package"

[dependencies]
rand = "0.8.5"
```

## Creating a New Package

You can create a new package using the `cargo new` command:

```bash
# Create a binary application
cargo new my_app

# Create a library
cargo new my_lib --lib
```

## Multiple Binary Crates

A package can contain multiple binary crates, each with its own entry point:

```
my_project/
├── Cargo.toml
├── src/
│   ├── main.rs         # Default binary
│   └── bin/
│       ├── tool1.rs    # Additional binary
│       └── tool2.rs    # Additional binary
```

Each file in the `bin` directory becomes a separate binary crate, which can be built and run independently.

## Package Workspaces

For larger projects, Rust supports workspaces that allow you to manage multiple related packages together:

```
workspace/
├── Cargo.toml          # Workspace manifest
├── package1/
│   ├── Cargo.toml
│   └── src/
├── package2/
│   ├── Cargo.toml
│   └── src/
└── package3/
    ├── Cargo.toml
    └── src/
```

A workspace's `Cargo.toml` file looks like this:

```toml
[workspace]
members = [
    "package1",
    "package2",
    "package3",
]
```

## Practice Exercise

Open the [`1_packages.rs`](./1_packages.rs) file and complete the exercises to test your understanding of Rust packages.

## Next Steps

Now that you understand packages, we can move on to [crates](../02_crates/README.md), which are the compilation units within packages. 