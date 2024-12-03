use std::env;
use std::fs;

use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let input = fs::read_to_string(path).expect("Expected valid files");

    let mul_regex: Regex = Regex::new(r"(mul\((?P<first>[0-9]{1,3}),(?P<second>[0-9]{1,3})\))")
        .expect("expected valid regex");
    let mut result = 0;
    for (_, [_, first, second]) in mul_regex.captures_iter(&input).map(|c| c.extract()) {
        println!("Captures are {first} {second}");
        let first = first.parse::<i32>().unwrap();
        let second = second.parse::<i32>().unwrap();
        result += first * second;
    }
    println!("Result is {result}");
}
