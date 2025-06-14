# Reader Project

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

To use this project, you can run the binary from the command line. The main entry point is located in `src/main.rs`, which utilizes the utility functions defined in `src/utils.rs`.

### Example

1. Clone the repository:
   ```
   git clone <repository-url>
   cd reader
   ```

2. Build the project:
   ```
   cargo build
   ```

3. Run the binary:
   ```
   cargo run
   ```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.