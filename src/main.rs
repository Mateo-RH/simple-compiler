use std::fs;

mod lib;
use lib::*;

fn main() {
    let content = fs::read_to_string("test.src").expect("Error reading file");
    let tokens = tokenize(content);
    println!("{:?}", tokens);
}
