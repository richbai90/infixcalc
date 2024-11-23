# Rust CLI Calculator

A command-line calculator implemented in Rust using the Shunting Yard algorithm for parsing and evaluating mathematical expressions.

## Features

- Evaluates mathematical expressions provided as command-line arguments
- Supports basic arithmetic operations: addition, subtraction, multiplication, division
- Handles parentheses for expression grouping
- Provides clear error messages for invalid inputs

## Installation

Ensure you have Rust and Cargo installed on your system. Then, clone this repository and build the project:

```bash
git clone https://github.com/yourusername/rust-cli-calculator.git
cd rust-cli-calculator
cargo build --release
```

## Usage
Run the calculator from the command line, providing your mathematical expression as an argument:

```bash
cargo run -- "your expression here"
```

For example:

```bash
cargo run -- "3 + 4 * 2" # Result: 11
```

## Supported Operations
1. Addition: +
2. Subtraction: -
3. Multiplication: *
4. Division: /
5. Modulo: %
6. Generic Base Exponentiation: ^
7. Base 10 Exponentiation: E
8. Parentheses: ( and )

## Error Handling
The calculator will provide error messages for invalid inputs, such as:
* Unbalanced parentheses
* Invalid operators
* Insufficient operands

## Dependencies
* clap for parsing command-line arguments
* Custom shunting_yard library for expression parsing and evaluation


## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the GPL 3.0 License - see the LICENSE file for details.
