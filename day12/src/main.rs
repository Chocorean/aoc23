use std::fs;

use day12::*;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut res = step1(&content, false);
    println!("{res}");
    res = step2(&content);
    println!("{res}");
}
