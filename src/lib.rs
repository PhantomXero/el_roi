//! # El Roi
//!
//! [![Crates.io](https://img.shields.io/crates/v/el_roi.svg)](https://crates.io/crates/el_roi)
//! [![Docs.rs](https://docs.rs/el_roi/badge.svg)](https://docs.rs/el_roi)
//! [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//!
//! `el_roi` is a Rust crate that simplifies reading user input from the command line. It provides ergonomic, prompt-driven functions for reading strings, integers, floats, booleans, characters, and vectors of these types, with a focus on usability and testability.
//!
//! ## Features
//! - Prompt the user with a custom question for each input
//! - Read integers (`i32`), floats (`f64`), booleans, characters, and vectors of these types
//! - Testable design: all parsing logic is separated for easy unit testing
//!
//! ## Usage
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! el_roi = "0.1.0"
//! ```
//!
//! ### Example
//!
//! ```rust
//! use el_roi::{read_string, read_int, read_float, read_bool, read_char, read_int_vec, read_float_vec, read_string_vec};
//!
//! fn main() {
//!     let name = read_string("Enter your name");
//!     let age = read_int("Enter your age");
//!     let pi = read_float("Enter the value of pi");
//!     let likes_rust = read_bool("Do you like Rust? (true/false)");
//!     let initial = read_char("Enter the first letter of your name");
//!     let numbers = read_int_vec("Enter some numbers separated by spaces");
//!     let floats = read_float_vec("Enter some floats separated by spaces");
//!     let words = read_string_vec("Enter some words separated by spaces");
//!     println!("Hello, {}! Age: {} Pi: {} Likes Rust: {} Initial: {} Numbers: {:?} Floats: {:?} Words: {:?}",
//!         name, age, pi, likes_rust, initial, numbers, floats, words);
//! }
//! ```
//!
//! ## Testing
//!
//! All parsing logic is separated into private helper functions, making it easy to test with in-memory buffers. See the crate's tests for examples.
//!
//! ## License
//!
//! MIT
use std::io::*;

// Error massage to let user know the error type
const ERR_MSG: &str = "Error reading user input";
const INVALID_OPTION: &str = "Invalid input data type";

// TODO: Add more functions to read different data types
// TODO: Error handling for invalid inputs

// Public API: user-facing functions (read from stdin)
// Function to get String input from the user input
pub fn read_string(prompt: &str) -> String {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_from(&mut handle)
}

/// Function to get an integer(i32) input from the user input
pub fn read_int(prompt: &str) -> i32 {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_from(&mut handle)
}

/// Function to get a float(f64) input from the user input
pub fn read_float(prompt: &str) -> f64 {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_from(&mut handle)
}

/// Function to get a boolean input from the user input
pub fn read_bool(prompt: &str) -> bool {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_bool_from(&mut handle)
}

/// Function to get a character input from the user input
pub fn read_char(prompt: &str) -> char {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_char_from(&mut handle)
}

/// Function to get a vector of integers(i32) input from the user input
pub fn read_int_vec(prompt: &str) -> Vec<i32> {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_vec_from(&mut handle)
}

/// Function to get a vector of floats(f64) input from the user input
pub fn read_float_vec(prompt: &str) -> Vec<f64> {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_vec_from(&mut handle)
}

/// Function to get a vector of strings input from the user input
pub fn read_string_vec(prompt: &str) -> Vec<String> {
    question(prompt);

    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_vec_from(&mut handle)
}

// Private helpers: *_from functions for testability
// Helper to covert user input from a buffered reader to a String
fn read_string_from<R: BufRead>(reader: &mut R) -> String {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().to_string()
}

// Helper to convert user input from a buffered reader to an i32
fn read_int_from<R: BufRead>(reader: &mut R) -> i32 {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<i32>().expect(INVALID_OPTION)
}

// Helper to convert user input from a buffered reader to a f64
fn read_float_from<R: BufRead>(reader: &mut R) -> f64 {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<f64>().expect(INVALID_OPTION)
}

// Helper to convert user input from a buffered reader to a bool
fn read_bool_from<R: BufRead>(reader: &mut R) -> bool {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    match user_res.trim().to_lowercase().as_str() {
        "true" | "yes" | "1" | "y" | "Y" => true,
        "false" | "no" | "0" | "n" | "N" => false,
        _ => panic!("{}", INVALID_OPTION),
    }
}

// Helper to convert user input from a buffered reader to a char
// Note: This function assumes the user will input a single character followed by a newline
fn read_char_from<R: BufRead>(reader: &mut R) -> char {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().chars().next().expect(INVALID_OPTION)
}

// Helper to convert user input from a buffered reader to a vector of integers
// Note: This function assumes the user will input space-separated integers
// Example input: "1 2 3 4 5"
fn read_int_vec_from<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect(INVALID_OPTION))
        .collect()
}

// Helper to convert user input from a buffered reader to a vector of floats
// Note: This function assumes the user will input space-separated floats
fn read_float_vec_from<R: BufRead>(reader: &mut R) -> Vec<f64> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect(INVALID_OPTION))
        .collect()
}

// Helper to convert user input from a buffered reader to a vector of strings
// Note: This function assumes the user will input space-separated strings
fn read_string_vec_from<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

// Helper function to print a question prompt
fn question(prompt: &str) {
    println!("{}: ", prompt);
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_read_string_from() {
        let input = b"Hello, World!\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_string_from(&mut cursor), "Hello, World!");
    }

    #[test]
    fn test_read_int_from() {
        let input = b"42\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_int_from(&mut cursor), 42);
    }

    #[test]
    fn test_read_float_from() {
        let input = b"3.14\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_float_from(&mut cursor), 3.14);
    }

    #[test]
    fn test_read_bool_from_true() {
        let input = b"true\n";
        let mut cursor = Cursor::new(input);
        assert!(read_bool_from(&mut cursor));
    }

    #[test]
    fn test_read_bool_from_false() {
        let input = b"no\n";
        let mut cursor = Cursor::new(input);
        assert!(!read_bool_from(&mut cursor));
    }

    #[test]
    fn test_read_char_from() {
        let input = b"a\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_char_from(&mut cursor), 'a');
    }

    #[test]
    fn test_read_int_vec_from() {
        let input = b"1 2 3 4 5\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_int_vec_from(&mut cursor), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_read_float_vec_from() {
        let input = b"1.1 2.2 3.3\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_float_vec_from(&mut cursor), vec![1.1, 2.2, 3.3]);
    }

    #[test]
    fn test_read_string_vec_from() {
        let input = b"foo bar baz\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(read_string_vec_from(&mut cursor), vec!["foo", "bar", "baz"]);
    }
}
