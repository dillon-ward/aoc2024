use std::fs;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("day3/input.txt").expect("Should have read file");
    println!("Part 1: {}", day1(&input));
    println!("Part 2: {}", day2(&input))
}

fn day1(input: &str) -> u64 {
    let mut result: u64 = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    for (_, [m1, m2]) in re.captures_iter(input).map(|c| c.extract()) {
        let n1: u64 = m1.parse().expect("Should have parsed to u64");
        let n2: u64 = m2.parse().expect("Should have parsed to u64");
        result += n1 * n2;
    }

    result
}

fn day2(input: &str) -> u64 {
    let mut pos = 0;
    let mut new_input = String::with_capacity(input.len());

    loop {

        match input[pos..].find("don't()") {
            Some(p) => {
                new_input.push_str(&input[pos..(pos + p)]);
                pos += p;
            }
            None => {
                new_input.push_str(&input[pos..]);
                break;
            }
        }

        match input[pos..].find("do()") {
            Some(p) => {
                pos += p;
            }
            None => {
                break;
            }
        }
    }

    day1(&new_input)
}