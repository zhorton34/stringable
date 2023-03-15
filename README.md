# Stringable

Build Status
Crates.io
License: MIT

Stringable is a Rust crate that provides a simple and fluent interface for common string manipulations. It aims to make string handling more convenient and readable in Rust.

Features

Fluent interface for string manipulations
Easy-to-use methods for common string operations
Chainable methods for improved readability
Installation

Add this to your Cargo.toml file:

toml
Copy code
[dependencies]
stringable = "0.1.0"
Usage

rust
Copy code
use stringable::stringable::Stringable;

let original = "This is a test, and it's a good test.   ";

let result = Stringable::new(original)
    .replace("test", "example")
    .to_lowercase()
    .trim()
    .append(&[" Additional text."])
    .get_value()
    .to_string();

println!("{}", result); // Output: "this is a example, and it's a good example. additional text."
Documentation

API documentation is available here.

Available Methods

Here are some of the available methods in the Stringable struct:

new: Create a new Stringable instance
get_value: Get the current value of the Stringable instance
replace: Replace all occurrences of a substring with another substring
to_lowercase: Convert the string to lowercase
to_uppercase: Convert the string to uppercase
trim: Trim leading and trailing whitespace
append: Append one or more strings to the current string
after: Get the substring after the first occurrence of a given value
before: Get the substring before the first occurrence of a given value
after_last: Get the substring after the last occurrence of a given value
And more...
License

This project is licensed under the MIT License.




