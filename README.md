Undup
=====

A simple library that consists of utility functions for removing duplicate characters in a string.

Example
-------
You can look at `examples/simple.rs` and run it using `cargo run --example simple`.

or

`
extern crate undup;

use undup::undup_chars;

fn main() {
    let input = "The   Quick     Brown Fox       Jumps  Over The    Lazy Dog.........";
    let output = undup_chars(&input, vec![' ', '.']);
    println!("{}", output);
}
`

License
-------
GPL-2.0
