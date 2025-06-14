# El Roi Project

[![Crates.io](https://img.shields.io/crates/v/el_roi.svg)](https://crates.io/crates/el_roi)
[![Docs.rs](https://docs.rs/el_roi/badge.svg)](https://docs.rs/el_roi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

El Roi is a Rust library that simplifies reading user input from the command line. It provides ergonomic, prompt-driven functions for reading strings, integers, floats, booleans, characters, and vectors of these types. The crate is designed for usability, clarity, and testability.

## Purpose

El Roi abstracts away the repetitive patterns of reading and parsing user input, allowing developers to easily and safely gather data from users in CLI applications.

## Features

- Prompt the user with a custom question for each input
- Read integers (`i32`), floating-point numbers (`f64`), booleans, characters, and vectors of these types
- Consistent, easy-to-use API for all supported types
- Testable design: parsing logic is separated for easy unit testing

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

Import and use the utility functions in your Rust code. Each function takes a prompt string, which is displayed to the user before reading input.

### Basic Example

```rust
use el_roi::{read_string, read_int, read_float, read_bool, read_char, read_int_vec, read_float_vec, read_string_vec};

fn main() {
    let name = read_string("Enter your name");
    let age = read_int("Enter your age");
    let pi = read_float("Enter the value of pi");
    let likes_rust = read_bool("Do you like Rust? (true/false)");
    let initial = read_char("Enter the first letter of your name");
    let numbers = read_int_vec("Enter some numbers separated by spaces");
    let floats = read_float_vec("Enter some floats separated by spaces");
    let words = read_string_vec("Enter some words separated by spaces");
    println!("Hello, {}! Age: {} Pi: {} Likes Rust: {} Initial: {} Numbers: {:?} Floats: {:?} Words: {:?}",
        name, age, pi, likes_rust, initial, numbers, floats, words);
}
```

### Function Reference

- `read_string(prompt: &str) -> String` — Read a string from the user
- `read_int(prompt: &str) -> i32` — Read an integer
- `read_float(prompt: &str) -> f64` — Read a floating-point number
- `read_bool(prompt: &str) -> bool` — Read a boolean (`true`/`false`, `yes`/`no`, `1`/`0`)
- `read_char(prompt: &str) -> char` — Read a single character
- `read_int_vec(prompt: &str) -> Vec<i32>` — Read a vector of integers (space-separated)
- `read_float_vec(prompt: &str) -> Vec<f64>` — Read a vector of floats (space-separated)
- `read_string_vec(prompt: &str) -> Vec<String>` — Read a vector of strings (space-separated)

### Example: Reading Multiple Types

```rust
use el_roi::{read_string, read_int, read_bool};

fn main() {
    let username = read_string("Username");
    let age = read_int("Age");
    let is_admin = read_bool("Are you an admin? (yes/no)");
    println!("User: {}, Age: {}, Admin: {}", username, age, is_admin);
}
```

## Testing

All parsing logic is separated into private helper functions, making it easy to test with in-memory buffers. See the crate's tests for examples.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.