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
- [Programming a guessing game](#programming-a-guessing-game)
    - [Processing a guess](#processing-a-guess)
    - [Storing values with variables](#storing-values-with-variables)
    - [Receiving user input](#receiving-user-input)
    - [Handling potential failure with `Result`](#handling-potential-failure-with-result)
    - [Printing values with `println!` placeholders](#printing-values-with-println-placeholders)
    - [Generating a secret number](#generating-a-secret-number)
    - [Using a crate to get more functionality](#using-a-crate-to-get-more-functionality)
    - [Ensuring reproducible builds with the *Cargo.lock* file](#ensuring-reproducible-builds-with-the-cargolock-file)
    - [Updating a crate to get a new version](#updating-a-crate-to-get-a-new-version)
    - [Generatiing a random number](#generatiing-a-random-number)

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

## Programming a guessing game

### Processing a guess

```rust
use std::io;
```

We need to use the `io`(input/output) library.

By default, Rust has a set of items defined in the standard library.
This set is called the *prelude*, and you can see everything in it [in the standard library documentation](https://doc.rust-lang.org/stable/std/prelude/index.html).

### Storing values with variables

```rust
let mut guess = String::new();
```

In Rust, variables are **immutable** by default.
To make a variable mutable, we add `mut` before the variable name.

`String::new()` returns a new instance of a `String`.
[String](https://doc.rust-lang.org/stable/std/string/struct.String.html) is a string type provided by the standard library that a growable, UTF-8 encoded bit of text.

### Receiving user input

```rust
io::stdin()
    .read_line(&mut guess)
```

If we hadn't imported the `io` library, we could use `std::io::stdin()`

The `&` indicates that is argument is a *reference*.
References are **immutable** by default.
Hence, you need to write `&mut guess` rather than `&guess` to make it **mutable**.

### Handling potential failure with `Result`

```rust
    .expect("Failed to read line");
```

`read_line()` returns a `Result` value.
[Result](https://doc.rust-lang.org/stable/std/result/enum.Result.html) is an [*enumeration*](https://doc.rust-lang.org/stable/book/ch06-00-enums.html),
    which is a type that can be in one of multiple possible states.
We call each possible state a *variant*.

`Result`'s variants are `Ok` and `Err`.
The `Ok` variant indicates the operation was sucessful, and inside `Ok` is the successfully generated value.
The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

An instance of `Result` has an [`expect` method](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.expect).
If this instance of `Result` is an `Err` value,
    `expect` will cause the program to crash and display the message the you passed as an argument to `expect`.
If this instance of `Result` is an `Ok` value,
    `expect` will take the return value that `Ok` is holding and return just that value.

    If you don't call `expect`, you'll get a warning.

### Printing values with `println!` placeholders

The `{}` set of curly brackets is a placeholder.
When printing the value of a variable,
    the variable name can go inside the curly brackets.
When printing the result of evaluating an expression,
    place empty curly brackets, then follow the format string with a comma-separated list of expressoins.

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

### Generating a secret number

Rust doesn't yet include random number functionality in its standard library.
However, the Rust team does provide a [`rand` crate](https://crates.io/crates/rand) with said functionality.

### Using a crate to get more functionality

Before we can write code that uses `rand`,
    we need to modify the *Cargo.toml* file to include the `rand` crate as a dependency.
Open that file and add the following line to the bottom.

```toml
[dependencies]
rand = "0.8.5"
```

The specifier `0.8.5` is actually shorthand for `^0.8.5`,
    which means any version that is at least 0.8.5 but below 0.9.0.

When we include an external dependency,
    Cargo fetches the latest versions of everthing that dependency needs from the *registry*,
    which is a copy of data from [Crates.io](https://crates.io/).
Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the `[dependencies]` section and downloads any crates listed.

### Ensuring reproducible builds with the *Cargo.lock* file

To ensure you can rebuild the same artifact every time you or anyone else builds your code:
    Cargo will use only the versions of the dependencies you specified until you indecate otherwise.

### Updating a crate to get a new version

When you *do* want to update a crate, Cargo provides the command `update`,
    which will ignore the *Cargo.lock* file and figure out all the latest versions that fit your specifications in *Cargo.toml*.
Cargo will then write those versions to the *Cargo.lock* file.
Otherwise, by default, Cargo will only look for versions greater than 0.8.5 and less than 0.9.0.

### Generatiing a random number

```rust
// ...
use rand::Rng;

fn main() {
    // ...

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // ...
}
```

First we add the line `use rand::Rng`.
The `Rng` trait defines methods that random number generator implements,
    and this trait must be in scope for us to use those methods.

Next, we're adding two lines.
In the first line, we call the `rand::thread_rng()` function that gives us the particular random number generator we're going to use:
    one that is local to the current thread of execution and is seeded by the operating system.
Then we call the `gen_range()` method on the random number generator.
This method is defined by the `Rng` trait.
The `gen_range()` method takes a range expression as an argument and generates a random number in the range.
The kine of range expression we're using here takes the form `start..=end` and is inclusive on the lower and upper bounds,
    so we need to specify `1..=100` to request a number between 1 and 100.
