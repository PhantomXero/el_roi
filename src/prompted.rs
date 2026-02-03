use std::io::stdin;

use crate::utils::{
    print_prompt, read_bool_loop, read_char_loop, read_float_loop, read_float_vec_loop,
    read_int_retrry, read_int_vec_loop, read_string_from, read_string_vec_from,
};

/// Public API: user-facing functions (read from stdin)
/// Function to get String input from the user input with a prompt
pub fn read_stringp(prompt: &str) -> String {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_from(&mut handle)
}

/// Function to get an integer(i32) input from the user input with a prompt
pub fn read_intp(prompt: &str) -> i32 {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_retrry(&mut handle, Some(prompt))
}

/// Function to get a float(f64) input from the user input with a prompt
pub fn read_floatp(prompt: &str) -> f64 {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_loop(&mut handle, Some(prompt))
}

/// Function to get a boolean input from the user input with a prompt
pub fn read_boolp(prompt: &str) -> bool {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_bool_loop(&mut handle, Some(prompt))
}

/// Function to get a character input from the user input with a prompt
pub fn read_charp(prompt: &str) -> char {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_char_loop(&mut handle, Some(prompt))
}

/// Function to get a vector of integers(i32) input from the user input with a prompt
pub fn read_int_vecp(prompt: &str) -> Vec<i32> {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_int_vec_loop(&mut handle, Some(prompt))
}

/// Function to get a vector of floats(f64) input from the user input with a prompt
pub fn read_float_vecp(prompt: &str) -> Vec<f64> {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_float_vec_loop(&mut handle, Some(prompt))
}

/// Function to get a vector of strings input from the user input with a prompt
pub fn read_string_vecp(prompt: &str) -> Vec<String> {
    print_prompt(prompt);
    let stdin = stdin();
    let mut handle = stdin.lock();
    read_string_vec_from(&mut handle)
}
