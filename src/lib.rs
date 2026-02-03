//! # El Roi
//!
//! [![Crates.io](https://img.shields.io/crates/v/el_roi.svg)](https://crates.io/crates/el_roi)
//! [![Docs.rs](https://docs.rs/el_roi/badge.svg)](https://docs.rs/el_roi)
//! [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//!
//! `el_roi` is a Rust crate that simplifies reading user input from the command line. It provides ergonomic functions for reading strings, integers, floats, booleans, characters, and vectors of these types, with both prompted (`*_p`) and unprompted APIs for stream-style input.
//!
//! ## Features
//! - Prompted and unprompted input variants for each supported type
//! - Read integers (`i32`), floats (`f64`), booleans, characters, and vectors of these types
//! - Testable design: all parsing logic is separated for easy unit testing
//! - Prompted reads re-prompt on invalid input and show an example of valid input
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
//! ```no_run
//! use el_roi::{
//!     read_boolp, read_charp, read_floatp, read_float_vecp, read_intp, read_int_vecp, read_stringp,
//!     read_string_vecp,
//! };
//!
//! fn main() {
//!     let name = read_stringp("Enter your name");
//!     let age = read_intp("Enter your age");
//!     let pi = read_floatp("Enter the value of pi");
//!     let likes_rust = read_boolp("Do you like Rust? (true/false)");
//!     let initial = read_charp("Enter the first letter of your name");
//!     let numbers = read_int_vecp("Enter some numbers separated by spaces");
//!     let floats = read_float_vecp("Enter some floats separated by spaces");
//!     let words = read_string_vecp("Enter some words separated by spaces");
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

mod prompted;
mod unprompted;
mod utils;

pub use prompted::{
    read_boolp, read_charp, read_floatp, read_float_vecp, read_intp, read_int_vecp, read_stringp,
    read_string_vecp,
};
pub use unprompted::{
    read_bool, read_char, read_float, read_float_vec, read_int, read_int_vec, read_string,
    read_string_vec,
};

// TODO: Add more functions to read different data types
// TODO: Error handling for invalid inputs

// Tests
#[cfg(test)]
mod tests {
    use crate::utils::{
        read_bool_from, read_char_from, read_float_from, read_float_vec_from, read_int_from,
        read_int_vec_from, read_string_from, read_string_vec_from,
    };
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
        let input = b"Anime is cool\n";
        let mut cursor = Cursor::new(input);
        assert_eq!(
            read_string_vec_from(&mut cursor),
            vec!["Anime", "is", "cool"]
        );
    }
}
