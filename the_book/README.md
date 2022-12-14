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
