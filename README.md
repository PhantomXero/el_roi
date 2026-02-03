# El Roi Project

[![Crates.io](https://img.shields.io/crates/v/el_roi.svg)](https://crates.io/crates/el_roi)
[![Docs.rs](https://docs.rs/el_roi/badge.svg)](https://docs.rs/el_roi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

El Roi is a Rust library that simplifies reading user input from the command line. It provides ergonomic functions for reading strings, integers, floats, booleans, characters, and vectors of these types, with both prompted (`*_p`) and unprompted APIs for stream-style input. The crate is designed for usability, clarity, and testability.

## Purpose

El Roi abstracts away the repetitive patterns of reading and parsing user input, allowing developers to easily and safely gather data from users in CLI applications.

## Features

- Prompted and unprompted input variants for each supported type
- Read integers (`i32`), floating-point numbers (`f64`), booleans, characters, and vectors of these types
- Consistent, easy-to-use API for all supported types
- Testable design: parsing logic is separated for easy unit testing
- Invalid inputs print an error with an example and re-prompt (prompted APIs)

## Installation

Add El Roi as a dependency in your project's `Cargo.toml`:

```toml
el_roi = "0.1.0"
```

Or, if using a local path during development:

```toml
el_roi = { path = "../el_roi" }
```

## Usage

Import and use the utility functions in your Rust code. Prompted functions end with `p` (e.g., `read_intp`) while unprompted functions keep the original names (e.g., `read_int`). This makes it easy to build both question/answer flows and constant input streams like CLI games.

### Basic Example

```rust
use el_roi::{
    read_boolp, read_charp, read_floatp, read_float_vecp, read_intp, read_int_vecp, read_stringp,
    read_string_vecp,
};

fn main() {
    let name = read_stringp("Enter your name");
    let age = read_intp("Enter your age");
    let pi = read_floatp("Enter the value of pi");
    let likes_rust = read_boolp("Do you like Rust? (true/false)");
    let initial = read_charp("Enter the first letter of your name");
    let numbers = read_int_vecp("Enter some numbers separated by spaces");
    let floats = read_float_vecp("Enter some floats separated by spaces");
    let words = read_string_vecp("Enter some words separated by spaces");
    println!("Hello, {}! Age: {} Pi: {} Likes Rust: {} Initial: {} Numbers: {:?} Floats: {:?} Words: {:?}",
        name, age, pi, likes_rust, initial, numbers, floats, words);
}
```

### Function Reference

Prompted reads will re-prompt on invalid input and print an example of valid input.

- `read_stringp(prompt: &str) -> String` — Read a string with a prompt
- `read_intp(prompt: &str) -> i32` — Read an integer with a prompt
- `read_floatp(prompt: &str) -> f64` — Read a floating-point number with a prompt
- `read_boolp(prompt: &str) -> bool` — Read a boolean with a prompt (`true`/`false`, `yes`/`no`, `1`/`0`)
- `read_charp(prompt: &str) -> char` — Read a single character with a prompt
- `read_int_vecp(prompt: &str) -> Vec<i32>` — Read a vector of integers with a prompt (space-separated)
- `read_float_vecp(prompt: &str) -> Vec<f64>` — Read a vector of floats with a prompt (space-separated)
- `read_string_vecp(prompt: &str) -> Vec<String>` — Read a vector of strings with a prompt (space-separated)
- `read_string() -> String` — Read a string without a prompt (stream-friendly)
- `read_int() -> i32` — Read an integer without a prompt
- `read_float() -> f64` — Read a floating-point number without a prompt
- `read_bool() -> bool` — Read a boolean without a prompt
- `read_char() -> char` — Read a single character without a prompt
- `read_int_vec() -> Vec<i32>` — Read a vector of integers without a prompt (space-separated)
- `read_float_vec() -> Vec<f64>` — Read a vector of floats without a prompt (space-separated)
- `read_string_vec() -> Vec<String>` — Read a vector of strings without a prompt (space-separated)

### Example: Reading Multiple Types

```rust
use el_roi::{read_boolp, read_intp, read_stringp};

fn main() {
    let username = read_stringp("Username");
    let age = read_intp("Age");
    let is_admin = read_boolp("Are you an admin? (yes/no)");
    println!("User: {}, Age: {}, Admin: {}", username, age, is_admin);
}
```

## Testing

All parsing logic is separated into private helper functions, making it easy to test with in-memory buffers. See the crate's tests for examples.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
