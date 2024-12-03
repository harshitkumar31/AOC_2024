use std::env;
use std::fs;

use regex::bytes::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    let input = fs::read(path).expect("Expected valid files");
    let mut enabled = true;

    let mul_regex: Regex =
        Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don't\(\))").expect("expected valid regex");

    let result = mul_regex
        .captures_iter(&input)
        .filter(|capture| {
            if capture.get(0).unwrap().as_bytes() == b"do()" {
                enabled = true;
                return false;
            } else if capture.get(0).unwrap().as_bytes() == b"don't()" {
                enabled = false;
            }
            enabled
        })
        .map(|capture| {
            atoi::atoi::<usize>(capture.get(2).unwrap().as_bytes()).unwrap()
                * atoi::atoi::<usize>(capture.get(3).unwrap().as_bytes()).unwrap()
        })
        .sum::<usize>();
    println!("Result is {result}");
}
