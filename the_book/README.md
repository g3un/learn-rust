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
    - [Comparing the guess to the secret number](#comparing-the-guess-to-the-secret-number)
    - [Allowing multiple guesses with loopig](#allowing-multiple-guesses-with-loopig)
    - [Quitting after a correct guess](#quitting-after-a-correct-guess)
    - [Handling invalid input](#handling-invalid-input)
- [Variables and mutability](#variables-and-mutability)
    - [Constants](#constants)
    - [Shadowing](#shadowing)
- [Data types](#data-types)
    - [Scalar types](#scalar-types)
    - [Compound types](#compound-types)
- [Functions](#functions)
    - [Parameters](#parameters)
    - [Statements and expressions](#statements-and-expressions)
    - [Functions with return values](#functions-with-return-values)
- [Comments](#comments)
- [Control flow](#control-flow)
    - [`if` expressions](#if-expressions)
    - [Repetition with loops](#repetition-with-loops)
- [What is ownership?](#what-is-ownership)
    - [Ownership rules](#ownership-rules)
    - [Variable scope](#variable-scope)
    - [The `String` type](#the-string-type)
    - [Memory and allocation](#memory-and-allocation)
    - [Ownership and functions](#ownership-and-functions)
    - [Return values and scope](#return-values-and-scope)
- [References and borrowing](#references-and-borrowing)
    - [Mutable references](#mutable-references)
    - [Dangling references](#dangling-references)
- [The slice type](#the-slice-type)
    - [String slices](#string-slices)
    - [Other slices](#other-slices)
- [Defining and instantiating structs](#defining-and-instantiating-structs)
    - [Using the field init shorthand](#using-the-field-init-shorthand)
    - [Creating instances from other instances with struct update syntax](#creating-instances-from-other-instances-with-struct-update-syntax)
    - [Using tuple structs without named fields to create different types](#using-tuple-structs-without-named-fields-to-create-different-types)
    - [Unit-like structs without any fields](#unit-like-structs-without-any-fields)
- [An Example Program Using Structs](#an-example-program-using-structs)
    - [Refactoring with Tuples](#refactoring-with-tuples)
    - [Refactoring with Structs: Adding More Meaning](#refactoring-with-structs-adding-more-meaning)
    - [Adding Useful Functionality with Derived Traits](#adding-useful-functionality-with-derived-traits)
- [Method Syntax](#method-syntax)
    - [Defining Methods](#defining-methods)
    - [Methods with More Parameters](#methods-with-more-parameters)
    - [Associated Functions](#associated functions)
    - [Multiple `impl` Blocks](#multiple-impl-blocks)

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
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // --snip--
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

### Comparing the guess to the secret number

```rust
    // --snip--

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

The `Ordering` type is another enum and has the variants `Less`, `Greater`, and `Equal`.

The `cmp()` compares two values and can be called on anything that can be compared.

We use a [`match{}`](https://doc.rust-lang.org/stable/book/ch06-02-match.html) expression to decide what to do next.
A `match{}` is made up of *arms*.
An arm consists of a *pattern* to match against,
    and the code that should be run if the value given to `match{}` fits that arm's pattern.

Rust has a strong, static type system.
When we wrote `let muit guess = String::new()`,
    Rust was able to infer that `guess` should be a `String` and didn't make us write the type.
The `secret_number` is a number type.
Rust cannot compare a string and a number type.

Ultimately, we want to convert the `String` into a real number type,
    so we can compare it numerically to the secret number.

```rust
    // --snip--

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    // --snip--
```

Rust allows us to shadow the previous value of `guess` with a new one.
*Shadowing* lets us reuse the `guess` variable name rather than forcing us to create two unique variable.

The `trim()` on a `String` instance will eliminate any whitespace at the beginning and end.
The [`parse()`](https://doc.rust-lang.org/stable/std/primitive.str.html#method.parse) on a `String` instance converts a string to another type.
We need to tell Rust the exact number type we want by using `let guess: u32`.
The colon(`:`) after `guess` tells Rust we'll annotate the variable's type.

The `parse()` returns a `Result` type.
We'll treat this `Result` the same way by using the `expect()` again.

### Allowing multiple guesses with loopig

The `loop` keyword creates an infinite loop.
We'll add a loop to give users more chances at guessing the number.

```rust
    // --snip--

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

The program will now ask for another guess forever.
It doesn't seem like the user can quit!

We want to stop the game when the correct number is guessed.

### Quitting after a correct guess

```rust
        // --snip--

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Adding the `break` line after `println!()` makes the program exit the loop when the user guesses the secret number correctly.

### Handling invalid input

Let's make the game ignore a non-number so the user can continue guessing.

```rust
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // --snip--
```

We switch from an `exprect()` to a `match{}` to move from crashing on an error to handling the error.

If `parse()` is able to successfully turn the string into a number,
    it will return an `Ok` that contains the resultant number.
That `Ok` value will match the first arm's pattern,
    and the `match{}` will just return the `num` value.
That number will end up right where we want it in the new `guess` variable we're creating.

If `parse()` is *not* able to turn the string into a number,
    it will return an `Err` that contains more information about the error.
The `Err` does match the second arm.
The underscore(`_`) is a catchall value,
    We're saying we want to match all `Err` values, no matter what information they have inside them.
The `continue` go to the next interation of the `loop`.
So, the program ignores all errors that `parse()` might encounter!

Let's delete the `println!()` that outputs the secret number.

## Variables and mutability

By default, variables are **immutable**.

```rust
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
```

You should receive an error message regarding an immutability error.
Because you tried to assign a second value to the immutable `x` variable.

You can make them mutable by adding `mut` in front of the variable name.
Adding `mut` also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable's value.

```rust
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
```

We're allowed to change the vaule bound to `x` from `5` to `6` when `mut` is used.

### Constants

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

There are a few differences between constants and variables.

Constants are **always** immutable.
When you declare constants using the `const`, you *must* annotate the type of the value.
Constants can be declared in any scope, including the global scope.
Constants are valid for the entire time a program runs,
    within the scope in which they were declared.
Constants may be set only to a constant expression,
    not the result of a value that could only be computed at runtime.

Rust's naming convention for constants is to use all uppercase with underscores between words.

### Shadowing

You can declare a new variable with the same name as a previous variable.
We can shadow a variable by using the same variable's name and repeating the use of the `let`.

```rust
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
        // The value of x in the inner scope is: 12
    }

    printl!("The value of x is: {x}");
    // The value of x is: 6
```

Shadowing is different from marking a variables as `mut`,
    because we'll get a compile-time error if we accidentally reassiign to the variable without using the `let`.
By using `let`, we can perform a few transformations on a value,
    but have the variable be immutable after those transformations have been completed.

The other difference between `mut` and shadowing is that because we're effectively creating a new variable when we use the `let` again,
    we can change the type of value but reuse the same name.

```rust
    let spaces = "    ";
    let spaces = spaces.len();

    /* compile error
    let mut spaces = "    ";
    spaces = spaces.len();
    */
```

## Data types

We'll look at two data type subsets: scalar and compound.

### Scalar types

A *scalar* type represents a single value.
Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

| Length  | Signed  | Unsigned |
|---------|---------|----------|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

Each variant can be either signed or unsigned and has an explicit size.
Additionally, the `isize` and `usize` types depend on the architecture.

| Number literals | Example       |
|-----------------|---------------|
| Decimal         | `98_222`      |
| Hex             | `0xff`        |
| Octal           | `0o77`        |
| Binary          | `0b1111_0000` |
| Byte(`u8` only) | `b'A'`        |

You can wirte interger literals in any of the forms showin this table.
Number literals that can be multiple numeric types allow a type suffix, such as `57u8`,
    to designate the type.
Number literals can also use `_` as a visual separator

Rust's float-point types are `f32` and `f64`.
The `f32` type is a single-precision float, and `f64` has double precision.

Rust's boolean types are `true` and `false`.
Booleans are one byte in size.
The boolean type in Rust is specified using `bool`.

```rust
    let c = 'z';
    let z: char = '???'; // with explicit type annotation
    let heart_eyed_cat = '????';
```

We specify `char` literals with **single quotes**.
Rust's `char` type is four bytes in size and represents a Unicode scalar value.

### Compound types

*Compound types* can group multiple values into one type.
Rust has two primitive compound types: tuples and arrays.

A *tuple* is a general way of grouping together a number of values with a variety of types into one compound type.
We create a tuple by writing a comma-separated list of values inside parentheses.
To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value.

```rust
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;
```

We can also access a tuple element directly by using a period(`.`) followed by the index of the value.

```rust
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
```

The tuple without any values has a special name, *unit*.
This value and its corresponding type are both written `()`.
And, represent an empty value or an empty return type.
Expressions implicitly return the unit value if they don't return any other value.

Another way to have a collection of multiple values in with an *array*.
Unlike a tuple, every element of an array must have the same type.
And, arrays have a fixed length.

An array isn't as flexible as the vector type.
So, arrays are more useful when you know the number of elements will not need to change.

You write the values in an array as a comma-separated list inside square brackets.
You can write an array's type using square brackets with the type of each element,
    a semicolon, and then the number of elements in the array.
You can also initialize an array to contain the same value for each element by specifying the initial value,
    followed by a semicolon, and then the length of the array in square brackets.

```rust
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // [3, 3, 3, 3, 3]
    let a = [3; 5];
```

## Functions

You've seen the `fn` keyword, which allows you to declare new functions.

Rust code uses *snake case* as the conventional style for function and variable names. (e.g. `do_some()`)

### Parameters

We can define functions to have *parameters*.
Technically, the concrete values are called *arguments*,
    but in casual conversation,
    people tend to use the words *parameter* and *argument* interchangeably.

```rust
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

In function signatures, you *must* decalre the type of each parameter.

### Statements and expressions

- **Statements** are instructions that perform some action and do not return a value.
- Expressions evaluate to a resultant value.

Function bodies are made up of a series of statements optionally ending in an expression.
Rust is an expression-based language,
    this is an important distinction to understand.

```rust
    let y = {
        let x = 3;
        x + 1
    };

    // The value of y is 4
    println!("The value of y is: {y}");
```

Note that the `x + 1` line doesn't have a semicolon at the end.
Expressions do not include ending semicolons.
If you add a semicolon to the end of an expression, you turn it into a statement,
    and it will then not return a value.

### Functions with return values

We don't name return values,
    but we must declare their type after an arrow(`->`).
In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
You can return early from a function by using the `return` keyword and specifying a value,
    but most functions return the last expression implicitly.

```rust
fn main() {
    let x = plus_one(5);

    // The value of x is 6
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

## Comments

```rust
// A line comment

/*
 * A block comment
 */
```

## Control flow

### `if` expressions

An `if` expression allows you to branch your code depending on conditions.

```rust
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
```

All `if` expressions start with the keyword `if`, followed by a condition.
Blocks of code associated with the conditions in `if` expressions are sometimes called *arms*.

If the condition isn't a `bool`, we'll get an error.

```rust
    let number = 3;

    // Compile error
    if nuimber {
        println!("number was true");
    }

    // We can change the `if` expression to ...
    if number != 0 {
        println!("number was something other than zero");
    }
```

#### Handling multiple conditions with `else if`

```rust
    let number = 5;

    if number % 4 == 0 {
        // Pass this block
    } else if number % 3 == 0 {
        // Run this block
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // Pass this block
    } else {
        // Pass this block
    }
```

Rust only executes the block for the first `true` condition,
    and once it finds one,
    it doesn't even check the rest.

Using too many `else if` expressions can clutter your code,
    so if you have more than one,
    you might want to refactor your code.

#### Using `if` in a `let` statemnet

We can use it on the right side of a `let` statement to assign the outcome to a variable.

```rust
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // The value of number is: 5
    println!("The value of number is: {number}");
```

The values that have the potential to be results from each arm of the `if` must be the same type.

### Repetition with loops

The `loop` keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.

You can place the `break` keyword within the loop to tell the program when to stop executing the loop.
Also you can use `continue` keyword,
    which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

#### Returning values from loops

You can add the value you want returned after the `break` expression you use to stop the loop.

```rust
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // The result is 20
    println!("The result is {result}");
```

#### Loop labels to disambiguate between multiple loops

If you have loops within loops, `break` and `continue` apply to the innermost loop at that point.
You can optionally specify a *loop label* on a loop that you can then use with `break` or `continue` to specify that those keywords apply to the labeled loop instead of the innermost loop.
Loop labels must begin with a single quote.

```rust
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
```

#### Conditional loops with `while`

A program will often need to evaluate a condition within a loop.
While the condition is `true`, the loop runs.

```rust
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
```

#### Looping through a collection with `for`

You can use a `for` loop and execute some code for each item in a collection.

```rust
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }
```

The safety and conciseness of `for` loops make them the most commonly used loop construct in Rust.
Even in situations in which you want to run some code a certain nuumber of times,
    as in the countdown example that used a `while` loop,
    most Rustaceans would use a `for` loop.

```rust
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
```

## What is ownership?

*Ownership* is a set of rules that govern how a Rust program manages memory.

### Ownership rules

Keep these rules in mind.

- Each value in Rust has on *owner*
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### Variable scope

```rust
    {                      // s is not valid here
        let s = "hello";   // s is valid from this point forward

        // Do stuff with s
    }                      // s is no longer valid
```

In other words, there are two important points in time here:

- When `s` comes into scope, it is valid.
- It remains valid until it goes *out of* scope.

### The `String` type

To illustrate the rules of ownership,
    we need a data type that is more complex.
The `String` type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
And, this kind of string *can* be mutated.

```rust
    let mut s = String::from("Hello");

    s.push_str(", world!");

    // Hello, world!
    println!("{}", s);
```

### Memory and allocation

With the `String` type,
    in order to support a mutable, growable piece of text,
    we need to allocate an amount of memory on the heap to hold the contents.

- The memory must be requested from the memory allocator at runtime.
- We need a way of returning this memory to the allocator when we're done with our `String`.

In Rust, the memory is automatically returned once the variable that owns it goes out of scope.

```rust
    {
        let s = String::from("Hello"); // s is valid

        // Do stuff with s
    }                                  // s is no longer valid
```

When a variable goes out of scope, Rust calls a special function for us.
This function is called [`drop`](https://doc.rust-lang.org/stable/std/ops/trait.Drop.html#tymethod.drop),
    and it's where the author of `String` can put the code to return the memory.

#### Variables and data interacting with move

```rust
    let s1 = String::from("Hello");
    let s2 = s1;

    // Compile error
    println!("{}, world!", s1);
```

Rust invalidate the first variable, instead of being called a shallow copy, it's known as a *move*.

#### Variables and data interactin with clone

If we *do* want to deeply copy the heap data of the `String`,
    we can use a common method called `clone`.

```rust
    let s1 = String::from("Hello");
    let s2 = s1.clone();

    // Compile error
    println!("s1 = {}, s2 = {}", s1, s2);
```

When you see a call to `clone`,
    you know that some arbitrary code is being executed and that code may be expensive.

#### Stack-only data: copy

```
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);
```

We don't have a call to `clone`, but `x` is still valid and wasn't moved into `y`.

The reason is that types such as integers that have a known size at compile time are stoed entirely on the stack,
    so copies of the actual values are quick to make.
That means there's no reason we would want to prevent `x` from being valid after we create the variable `y`.

### Ownership and functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.


```rust
fn main() {
    let s = String::from("Hello");

    takes_ownership(s);
    // s is no longer valid

    let x = 5;

    makes_copy(x);

    println!("x is still valid: {x}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

### Return values and scope

Returning values can also transfer ownership.

```rust
fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("Hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("Yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

What if we want to let a function use a value but not take ownership?
Rust has a feature for using a value without transferring ownership, called *references*.

## References and borrowing

A *reference* is like a pointer in that it's an address we can follow to access the data stored at that address.
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

```rust
fn main() {
    let s1 = String::from("Hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

The `&s1` syntax lets us create a reference that *refers* to the value of `s1` but does not own it.
Because it does not own it,
    the value it points to will not be dropped when the reference stops being used.

We call the action of creating a reference *borrowing*.

```rust
fn main() {
    let s = String::from("Hello");

    change(&s);
}

fn change(some_string: &String) {
    // Compile error
    some_string.push_str(", world!");
}
```

Just as variables are immutable by default, so are references.
We're not allowed to modify something we have a reference to.

### Mutable references

We can fix the code to allow us to modify a borrowed value.

```rust
fn main() {
    let mut s = String::from("Hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```

Mutable references have one big restriction:
    if you have a mutable reference to a value,
    you can have no other references to that value.

```rust
    let mut s = String::from("Hello");

    // Compile error
    let r1 = &mut s;
    let r2 = &mut s;

    println!("r1 = {}, r2 = {}", r1, r2);
```

Rust enforces a similar rule for combining mutable and immutable references.

```rust
    let mut s = String::from("Hello");

    // Compile error
    let r1 = &s;     // No problem
    let r2 = &s;     // No problem
    let r3 = &mut s; // Big problem

    println!("r1 = {}, r2 = {}, r3 = {}", r1, r2, r3);
```

Note that a reference's scope starts from where it is introduced and continues through the last time that reference is used.

```
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("r1 = {}, r2 = {}", r1, r2);

    let r3 = &mut s;         // No problem
    println!("r3 = {}", r3);
```

### Dangling references

In Rust, the compiler guarantees that references will never be dangling references:
    if you have a reference to some data,
    the compiler will ensure that the data will not go out of scope before the reference to the data does.

```rust
fn main() {
    let reference_to_nothing = dangle();
}

// Compile error
fn dangle() -> &String {
    let s = String::from("Hello");

    &s
}
```

> this function's return type contains a borrowed value,
>     but there is no value for it to be borrowed from

Because `s` is created inside `dangle`,
    when the code of `dangle` is finished,
    `s` will be deallocated.
But we tried to return a reference to it.

The solution here is to return the `String` directly.

```rust
fn no_dangle() -> String {
    let s = String::from("Hello");

    s
}
```

## The slice type

*Slice* let you reference a contiguous sequence of elements in a collection rather than the whole collection.
A slice is a kind of reference, so it does not have ownership.

Here???s a small programming problem:
    write a function that takes a string of words separated by spaces and returns the first word it finds in that string.
If the function doesn???t find a space in the string,
    the whole string must be one word,
    so the entire string should be returned.

We don't really have a way to talk about *part* of a string.
However, we could return the index of the end of the word, indicated by a space.

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

We'll convert our `String` to an array of bytes using the `as_bytes()`.
Next, we create an iterator over the array of bytes using the `iter()`.
The `enumerate()` wraps the result of `iter()` and returns each element as part of a tuple instead.

We now have a way to find out the index of the end of the first word in the string,
    but there's a problem.
We're returning a `usize` on its own,
    but it's only a meaningful number in the context of the &String.
There's no guarantee that it will still be valid in the future.

```rust
fn main() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);

    s.clear();

    // word is now invalid
}
```

### String slices

A *string slice* is a reference to part of a `String`.

```rust
    let s = Stirng::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[..2];
    let slice = &s[6..];

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];
```

Let's rewrite `first_word()` to return a slice.
The type that signifies *string slice* is written as `&str`.

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }

    &s[..]
}
```

Slice is a immutable refenrence.

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    // Compile error
    s.clear();

    println!("the first word is: {}", word);
}
```

Rust disallows the mutable reference in `clear()` and the immutable reference in `word` from existing at the same time, and compilation fails.

#### String literals as slices

Recall that we talked about literals being stored inside the binary.
Now that we know about slices, we can properly understand string literals.

```rust
    let s = "Hello, world!";
```

The type of `s` here is `&str`:
    it's a slice pointing to that specific point of the binary.
This is also why string literals are immutable.

#### String slices as parameters

Knowing that you can take slices of literals and `String` values leads us to one more improvement on `first_word()`, and that's its signature.

```rust
fn first_word(s: &String) -> &str {
```

A more experienced Rustacean would write the signature to:

```rust
fn first_word(s: &str) -> &str {
```

Because it allows us to use the same functioin on both `&String` values and `&str` values.

If we have a string slice, we can pass that directly.
If we have a `String`, we can pass a slice of the `String` or a reference to the `String`.
This flexibility takes advantage of *deref coercions*.

```rust
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);
    let word = first_word(&my_string);

    let my_string_literal = "hello world");

    let word = first_word(&my_string_literal[..]);
    let word = first_word(my_string_literal);
```

### Other slices

```rust
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
```

This slice has the type `&[i32]`.
It works the same way as string slices do.

## Defining and instantiating structs

Like tuples, the pieces of a struct cna be different types.
Unlike with tuples, in a struct you'll name each piece of data so it's clear what the values mean.

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    }
    
    user1.email = String::from("anotheremail@example.com");
}
```

Ruust doesn't allow us to mark only certain fields as mutable.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Using the field init shorthand

When the parameter names and the struct field names are exactly the same,
    we can use the *field init shorthand* syntax.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

### Creating instances from other instances with struct update syntax

It's often useful to create a new instance of a struct that includes most of the values from antoher instance,
    but changes some.

```
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
```

Note that the struct update syntax uses `=` list an assignment;
    this is because it moves the data.
In this example, we can no longer use `user`,
    because the `String` in the `username` field of `user1` was moved into `user2`.

### Using tuple structs without named fields to create different types

Rust also supports struct that look similar to tuples, called *tuple structs*.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

Note that the `black` and `origin` valuues are different types.

### Unit-like structs without any fields

You can also define structs that don't have any fields!
These are called *unit-like structs*.
Unit-like structs can be useful when you need to implement a trait on some type,
    but don'y have any data that you want to store in the type itself.

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

## An Example Program Using Structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

Take the width and height of a rectangle specified in pixels and calculate the area of the rectangle.

We can do more to make this code clear and readable.

```rust
fn area(width: u32, height: u32) -> u32 {
```

The `area()` is supposed to calcuate the area of one rectangle,
    but the function we wrote has two parameters,
    and it's not clear anywhere in our program that the parameters are related.

### Refactoring with Tuples

```rust
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

This version is less clear:
    tuples don't name their elements,
    so we have to index into the parts of the tuple,
    making out calculation less obvious.

This would be even harder for someone else to figure out and keep in mind if they were to use our code.
Because we haven't conveyed the meaning of our data in our code,
    it's now easier to introduce errors.

### Refactoring with Structs: Adding More Meaning

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

### Adding Useful Functionality with Derived Traits

The `println!` can do many kinds of formatting,
    and by default,
    the curly brackets tell `println!` to use formatting known as `Display`:
    outpuut intended for direct end  user consumption.
But with structs, the way `println!` should format the output is less clear because there are more display possibilities.
Due to this ambiguity, Rust doesn't try to guess what we want,
    and structs don't have a provided implementation of `Display` to use with `println!` and the `{}` placeholder.

Putting the specifier `:?` inside the curly brackets tells `println!` we want to use an output format called `Debug`.
The `Debug` trait enables us to print out struct in a way that is useful for developers so we can see its value while we're debugging our code.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

Rust *does* include functionality to print out debugging information,
    but we have to explicitly opt in to make that functionality available for ouur struct.
To do that, we add the outer atrribute `#[derive(Debug)]`.

When we have larger structs,
    it's usefule to have outuput that's a bit easier to read;
    in those cases, we can use `{:#?}` instead of `{:?}` in the `println!` string.

Another way to print out a valuue using the `Debug` format in to use the [`dbg!`](https://doc.rust-lang.org/stable/std/macro.dbg.html),
    which takes ownership of an expression (as opposed to `println!` that takes a reference),
    prints the file and line number of where that `dbg!` call occurs in your code along with the resulting valuue of that expression,
    and returns ownership of the value.

Here's an example where we're interested in the vlaue that gets assigned to the `width` field,
    as well as the value of the whole struct in `rect1`.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

Let's look at how we can continue to refactor this code by turning the `area()` *function* into an `area()` *method* defined on our `Rectangle`.

## Method Syntax

*Methods* are similar to function.
Unlike functions, methods are defined within the context of a struct,
    and their first parameter is always `self`,
    which represents the instance of the struct the method is being called on.

### Defining Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define the funtion within the context of `Rectanle`,
    we start an `impl` block for `Rectangle`.
Everything within this `impl` block will be associated with the `Rectangle` type.

Methods must have a parameter named `self` of type `Self` for their first parameter,
    so Rust lets you abbreviate this with only the name `self` in the first parameter spot.
Note that we still need to use the `&` in front of the `self` shorthands to indicate this method borrows the `Self` instance.
Methods can take ownership of `self`,
    borrow `self` immutably as we've done here,
    or borrow `self` mutably,
    just as they can any other paramter.

The main reason for using methods instead of functions,
    in addition to providing method syntax and not having to repeat the type of `self` in every method's signature,
    is for organization.

Note that we can choose to give a method the same name as one of the struct's fields.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

When we use `rect1.width()`, Rust knows we mean the method `width`.
When we don't use parentheses, Rust knows we mean the field `width`.

### Methods with More Parameters

```rust
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
```

### Associated Functions

All functions defined within an `impl` block are called *associated functions*.
We can define associated functions that don't have `self` as their first paramter.

```rust
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```

### Multiple `impl` Blocks

Each struct is allowed to have multiple `impl` blocks.

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

There's no reason to separate these methods into multiple `impl` blocks here,
    but this is valid syntax.
