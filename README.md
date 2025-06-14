# El Roi Project

This project is a simple command-line utility for reading various types of user input in Rust. It provides functions to read integers, floats, booleans, characters, and vectors of these types.

## Purpose

The main purpose of this project is to simplify the process of reading user input from the command line. It abstracts away the common patterns of reading and parsing input, allowing developers to easily gather data from users.

## Features

- Read integers (`i32`)
- Read floating-point numbers (`f64`)
- Read booleans (`true`/`false`)
- Read characters
- Read vectors of integers, floats, and strings

## Usage

Add this crate as a dependency in your project's `Cargo.toml`:

```toml
el_roi = "0.1.0"
```

Then, import and use the utility functions in your Rust code:

```rust
use el_roi::your_function_name;

fn main() {
    // Example usage
    let value = your_function_name();
    println!("Value: {}", value);
}
```

Replace `your_function_name` with the actual function you want to use from the crate.

### Example

1. Add the dependency to your `Cargo.toml`:
   ```toml
   el_roi = { path = "../el_roi" }
   ```
   (Adjust the path as needed, or use the version from crates.io if published.)

2. Use the library in your code:
   ```rust
   use el_roi::read_int;

   fn main() {
       let number = read_int();
       println!("You entered: {}", number);
   }
   ```

3. Or, if the crate is published on crates.io, add it to your `Cargo.toml` like this:
   ```toml
   el_roi = "0.1.0"
   ```

   Then use it in your code as shown above:
   ```rust
   use el_roi::read_int;

   fn main() {
       let number = read_int();
       println!("You entered: {}", number);
   }
   ```