# El Roi Project

[![Crates.io](https://img.shields.io/crates/v/el_roi.svg)](https://crates.io/crates/el_roi)
[![Docs.rs](https://docs.rs/el_roi/badge.svg)](https://docs.rs/el_roi)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

El Roi is a Rust library that simplifies reading user input from the command line. It provides ergonomic functions for reading strings, integers, floats, booleans, characters, and vectors of these types, with both prompted (`prompt_*`) and unprompted APIs for stream-style input. The crate is designed for usability, clarity, and testability.

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
el_roi = "0.2.0"
```

Or, if using a local path during development:

```toml
el_roi = { path = "../el_roi" }
```

## Usage

Import and use the utility functions in your Rust code. Prompted functions use `prompt_*` names (e.g., `prompt_int`) while unprompted functions keep the original names (e.g., `read_int`). This makes it easy to build both question/answer flows and constant input streams like CLI games.

### Basic Example

```rust
use el_roi::{
    prompt_bool, prompt_char, prompt_float, prompt_float_vec, prompt_int, prompt_int_vec,
    prompt_string, prompt_string_vec,
};

fn main() {
    let name = prompt_string("Enter your name");
    let age = prompt_int("Enter your age");
    let pi = prompt_float("Enter the value of pi");
    let likes_rust = prompt_bool("Do you like Rust? (true/false)");
    let initial = prompt_char("Enter the first letter of your name");
    let numbers = prompt_int_vec("Enter some numbers separated by spaces");
    let floats = prompt_float_vec("Enter some floats separated by spaces");
    let words = prompt_string_vec("Enter some words separated by spaces");
    println!("Hello, {}! Age: {} Pi: {} Likes Rust: {} Initial: {} Numbers: {:?} Floats: {:?} Words: {:?}",
        name, age, pi, likes_rust, initial, numbers, floats, words);
}
```

### Function Reference

Prompted reads will re-prompt on invalid input and print an example of valid input.

- `prompt_string(prompt: &str) -> String` — Read a string with a prompt
- `prompt_int(prompt: &str) -> i32` — Read an integer with a prompt
- `prompt_float(prompt: &str) -> f64` — Read a floating-point number with a prompt
- `prompt_bool(prompt: &str) -> bool` — Read a boolean with a prompt (`true`/`false`, `yes`/`no`, `1`/`0`)
- `prompt_char(prompt: &str) -> char` — Read a single character with a prompt
- `prompt_int_vec(prompt: &str) -> Vec<i32>` — Read a vector of integers with a prompt (space-separated)
- `prompt_float_vec(prompt: &str) -> Vec<f64>` — Read a vector of floats with a prompt (space-separated)
- `prompt_string_vec(prompt: &str) -> Vec<String>` — Read a vector of strings with a prompt (space-separated)
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
use el_roi::{prompt_bool, prompt_int, prompt_string};

fn main() {
    let username = prompt_string("Username");
    let age = prompt_int("Age");
    let is_admin = prompt_bool("Are you an admin? (yes/no)");
    println!("User: {}, Age: {}, Admin: {}", username, age, is_admin);
}
```

## Testing

All parsing logic is separated into private helper functions, making it easy to test with in-memory buffers. See the crate's tests for examples.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
