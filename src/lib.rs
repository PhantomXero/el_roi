/// Read: a rust crate that simplifies reading user input
// import input/output controls
use std::io::*;

// Error massage to let user know the error type
const ERR_MSG: &str = "Error reading user input";
const INVALID_OPTION: &str = "Invalid input data type";

/// Reading and returning a String
pub fn _read_string() -> String {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().to_string()
}

/// Reading and returning an Integer(i32)
pub fn _read_int() -> i32 {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<i32>().expect(INVALID_OPTION)
}

/// Reading and returning a Float(f64)
pub fn _read_float() -> f64 {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<f64>().expect(INVALID_OPTION)
}

/// Reading and returning a Boolean
pub fn _read_bool() -> bool {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    match user_res.trim().to_lowercase().as_str() {
        "true" | "yes" | "1" | "y" | "Y" => true,
        "false" | "no" | "0" | "n" | "N" => false,
        _ => panic!("{}", INVALID_OPTION),
    }
}

/// Reading and returning a character
pub fn _read_char() -> char {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().chars().next().expect(INVALID_OPTION)
}

/// Reading and returning a vector of integers
/// Separated by whitespace
pub fn _read_int_vec() -> Vec<i32> {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect(INVALID_OPTION))
        .collect()
}

/// Reading and returning a vector of floats
/// Separated by whitespace
pub fn _read_float_vec() -> Vec<f64> {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect(INVALID_OPTION))
        .collect()
}

/// Reading and returning a vector of strings
/// This function reads a line of input and splits it into words, returning a vector of strings.
/// This is useful for cases where you want to read multiple words or phrases from the user.
/// # Example: dialog, command line arguments, sentences, etc.
pub fn _read_string_vec() -> Vec<String> {
    let mut user_res = String::new();
    stdin().read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}
