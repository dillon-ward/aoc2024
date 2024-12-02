use std::fs;

type List = Vec<u64>;

fn main() {
    let input = get_input("day1/input.txt");   
    let (left, right) = get_lists(&input);
    println!("Part 1: {}", part1(&left, &right));
    println!("Part 2: {}", part2(&left, &right));
}

fn part1(left: &[u64], right: &[u64]) -> u64 {
    left.iter().zip(right).map(|(ln, rn)| ln.abs_diff(*rn)).sum()
}

fn part2(left: &[u64], right: &[u64]) -> u64 {
    left.iter().map(|&ln| ln * right.iter().filter(|&n| *n == ln).count() as u64).sum()
}

fn get_input(filename: &str) -> Vec<List> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace().map(|n| n.parse::<u64>().expect("{n} is not an integer"))
            .collect()
        })
        .collect()
}

fn get_lists(input: &[List]) -> (List, List) {
    let (mut left, mut right) = input
        .iter()
        .fold((Vec::new(), Vec::new()), |(mut left, mut right), i| {
            left.push(i[0]);
            right.push(i[1]);

            (left, right)
        });

    left.sort();
    right.sort();

    (left, right)
}