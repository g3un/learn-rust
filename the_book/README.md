# The book
## Table of Contents

- [Installation](#installation)
    - [Installing `rustup`](#installing-rustup)
    - [Troubleshooting](#troubleshooting)
    - [Updating and Uninstalling](#updating-and-uninstalling)
    - [Local documentation](#local-documentation)
- [Hello, world!](#hello-world)
    - [Creating a project directory](#creating-a-project-directory)
    - [Writing and running a Rust program](#writing-and-running-a-rust-program)
    - [Anatomy of a Rust program](#anatomy-of-a-rust-program)
- [Hello, Cargo!](#hello-cargo)
    - [Creating a project with Cargo](#creating-a-project-with-cargo)
    - [Building and Running a Cargo project](#building-and-running-a-cargo-project)
    - [Building for release](#building-for-release)

## Installation

### Installing `rustup`

```shell
# To install rustup on macOS
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# If you get linker errors, you should install a C compiler
# To install C compiler on macOS
$ xcode-select --install
```

### Troubleshooting

```shell
# To check Rust is installed correctly
$ rustc --version
> rustc 1.65.0 (897e37553 2022-11-02)
```

### Updating and Uninstalling

```shell
# To update the latest version
$ rustup update

# To uninstall Rust and rustup
$ rustup self uninstall
```

### Local documentation

```shell
# To read the local documentation in your browser
$ rustup doc
```

## Hello, world!

### Creating a project directory

```shell
$ mkdir hello_world
$ cd hello_world
```

### Writing and running a Rust program

Create `main.rs` and enter the code

```rust
// main.rs
fn main() {
    println!("Hello, world!");
}
```

```shell
$ rustc main.rs
$ ./main
> Hello, world!
```

### Anatomy of a Rust program

`!` means that you're calling a **macro** instead of a normal function.
And that macros don't always follow the same rules as functions.

## Hello, Cargo!

### Creating a project with Cargo

```shell
$ cargo new hello_cargo
$ cd hello_cargo
```

If you want to use a version control system (e.g. Git), use `--vcs` flag.

`Cargo.toml` is a configuration file for this project.
And, your source files live inside the `src/` directory.

### Building and Running a Cargo project

```shell
$ cargo build
$ ./target/debug/hello_cargo

# or

$ cargo run
```

If you don't want an executable, use `cargo check` command.
It's much faster than `cargo buiild`.

### Building for release

`cargo build --release` is compile with optimizations.
The optimizations make your Rust code run faster,
  but turning them on lengthens the time it takes for your program to compile.
