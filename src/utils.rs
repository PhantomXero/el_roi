use std::io::BufRead;
use std::str::FromStr;

// Error massage to let user know the error type
const ERR_MSG: &str = "Error reading user input";
const INVALID_OPTION: &str = "Invalid input data type";
const BOOL_EXAMPLE: &str = "true (or yes/1)";
const CHAR_EXAMPLE: &str = "a";
const INT_EXAMPLE: &str = "42";
const FLOAT_EXAMPLE: &str = "3.14";
const INT_VEC_EXAMPLE: &str = "1 2 3";
const FLOAT_VEC_EXAMPLE: &str = "1.0 2.5 3.14";

pub(crate) fn print_invalid_input(example: &str) {
    println!("{INVALID_OPTION}. Example: {example}");
}

pub(crate) fn print_prompt(prompt: &str) {
    println!("{}: ", prompt);
}

/// Helper to covert user input from a buffered reader to a String
pub(crate) fn read_string_from<R: BufRead>(reader: &mut R) -> String {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().to_string()
}

/// Helper to convert user input from a buffered reader to a value via FromStr
pub(crate) fn read_value_from<R: BufRead, T: FromStr>(reader: &mut R) -> T
where
    T::Err: std::fmt::Debug,
{
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<T>().expect(INVALID_OPTION)
}

pub(crate) fn parse_from_reader<R: BufRead, T: FromStr>(reader: &mut R) -> Result<T, T::Err> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().parse::<T>()
}

/// Helper to convert user input from a buffered reader to an i32
pub(crate) fn read_int_from<R: BufRead>(reader: &mut R) -> i32 {
    read_value_from(reader)
}

/// Helper to convert user input from a buffered reader to a f64
pub(crate) fn read_float_from<R: BufRead>(reader: &mut R) -> f64 {
    read_value_from(reader)
}

/// Helper to convert user input from a buffered reader to a bool
pub(crate) fn read_bool_from<R: BufRead>(reader: &mut R) -> bool {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    match user_res.trim().to_lowercase().as_str() {
        "true" | "yes" | "1" | "y" => true,
        "false" | "no" | "0" | "n" => false,
        _ => panic!("{}", INVALID_OPTION),
    }
}

/// Helper to convert user input from a buffered reader to a char
/// Note: This function assumes the user will input a single character followed by a newline
pub(crate) fn read_char_from<R: BufRead>(reader: &mut R) -> char {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res.trim().chars().next().expect(INVALID_OPTION)
}

/// Helper to convert user input from a buffered reader to a vector of integers
/// Note: This function assumes the user will input space-separated integers
/// Example: "14 6 2025"
pub(crate) fn read_int_vec_from<R: BufRead>(reader: &mut R) -> Vec<i32> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect(INVALID_OPTION))
        .collect()
}

/// Helper to convert user input from a buffered reader to a vector of floats
/// Note: This function assumes the user will input space-separated floats
/// Example: "1.0 2.5 3.14"
pub(crate) fn read_float_vec_from<R: BufRead>(reader: &mut R) -> Vec<f64> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<f64>().expect(INVALID_OPTION))
        .collect()
}

/// Helper to convert user input from a buffered reader to a vector of strings
/// Note: This function assumes the user will input space-separated strings
/// Example: "This is so cool"
pub(crate) fn read_string_vec_from<R: BufRead>(reader: &mut R) -> Vec<String> {
    let mut user_res = String::new();
    reader.read_line(&mut user_res).expect(ERR_MSG);
    user_res
        .trim()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

pub(crate) fn read_int_retry<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> i32 {
    loop {
        match parse_from_reader(reader) {
            Ok(value) => return value,
            Err(_) => {
                print_invalid_input(INT_EXAMPLE);
                if let Some(prompt) = prompt {
                    print_prompt(prompt);
                }
            }
        }
    }
}

pub(crate) fn read_float_loop<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> f64 {
    loop {
        match parse_from_reader(reader) {
            Ok(value) => return value,
            Err(_) => {
                print_invalid_input(FLOAT_EXAMPLE);
                if let Some(prompt) = prompt {
                    print_prompt(prompt);
                }
            }
        }
    }
}

pub(crate) fn read_bool_loop<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> bool {
    loop {
        let mut user_res = String::new();
        reader.read_line(&mut user_res).expect(ERR_MSG);
        match user_res.trim().to_lowercase().as_str() {
            "true" | "yes" | "1" | "y" => return true,
            "false" | "no" | "0" | "n" => return false,
            _ => {
                print_invalid_input(BOOL_EXAMPLE);
                if let Some(prompt) = prompt {
                    print_prompt(prompt);
                }
            }
        }
    }
}

pub(crate) fn read_char_loop<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> char {
    loop {
        let mut user_res = String::new();
        reader.read_line(&mut user_res).expect(ERR_MSG);
        if let Some(value) = user_res.trim().chars().next() {
            return value;
        }
        print_invalid_input(CHAR_EXAMPLE);
        if let Some(prompt) = prompt {
            print_prompt(prompt);
        }
    }
}

pub(crate) fn read_int_vec_loop<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> Vec<i32> {
    loop {
        let mut user_res = String::new();
        reader.read_line(&mut user_res).expect(ERR_MSG);
        let parsed: Result<Vec<i32>, _> = user_res
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i32>())
            .collect();
        match parsed {
            Ok(values) => return values,
            Err(_) => {
                print_invalid_input(INT_VEC_EXAMPLE);
                if let Some(prompt) = prompt {
                    print_prompt(prompt);
                }
            }
        }
    }
}

pub(crate) fn read_float_vec_loop<R: BufRead>(reader: &mut R, prompt: Option<&str>) -> Vec<f64> {
    loop {
        let mut user_res = String::new();
        reader.read_line(&mut user_res).expect(ERR_MSG);
        let parsed: Result<Vec<f64>, _> = user_res
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<f64>())
            .collect();
        match parsed {
            Ok(values) => return values,
            Err(_) => {
                print_invalid_input(FLOAT_VEC_EXAMPLE);
                if let Some(prompt) = prompt {
                    print_prompt(prompt);
                }
            }
        }
    }
}
