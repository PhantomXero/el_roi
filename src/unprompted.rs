use std::io::stdin;

use crate::utils::{
    read_bool_loop, read_char_loop, read_float_loop, read_float_vec_loop, read_int_retrry,
    read_int_vec_loop, read_string_from, read_string_vec_from,
};

/// Function to get String input from the user input without a prompt
pub fn read_string() -> String {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_from(&mut handle)
}

/// Function to get an integer(i32) input from the user input without a prompt
pub fn read_int() -> i32 {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_retrry(&mut handle, None)
}

/// Function to get a float(f64) input from the user input without a prompt
pub fn read_float() -> f64 {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_loop(&mut handle, None)
}

/// Function to get a boolean input from the user input without a prompt
pub fn read_bool() -> bool {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_bool_loop(&mut handle, None)
}

/// Function to get a character input from the user input without a prompt
pub fn read_char() -> char {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_char_loop(&mut handle, None)
}

/// Function to get a vector of integers(i32) input from the user input without a prompt
pub fn read_int_vec() -> Vec<i32> {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_vec_loop(&mut handle, None)
}

/// Function to get a vector of floats(f64) input from the user input without a prompt
pub fn read_float_vec() -> Vec<f64> {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_vec_loop(&mut handle, None)
}

/// Function to get a vector of strings input from the user input without a prompt
pub fn read_string_vec() -> Vec<String> {
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_vec_from(&mut handle)
}
