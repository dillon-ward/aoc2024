use std::fs;

type List = Vec<i64>;

fn main() {
    let input = get_input("day2/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn is_safe(l: &List, s: i64) -> bool {
    for i in 0..l.len()-1 {
        let diff: i64 = l[i+1] - l[i];
        if diff.abs() < 1 || diff.abs() > 3 || diff.signum() != s {return false}
    }
    true
}

fn part1(input: &[List]) -> usize {
    input.iter().filter(|&l| is_safe(l, (l[l.len()-1] - l[0]).signum())).count()
}

fn part2(input: &[List]) -> usize {
    input.iter().filter(|&l| {
            let sign = (l[l.len()-1] - l[0]).signum();
            for i in 0..l.len() {
                let new_l = [&l[..i], &l[i+1..]].concat();
                if is_safe(&new_l, sign) {return true}
            }
            false
        }).count()
}

fn get_input(filename: &str) -> Vec<List> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.split_ascii_whitespace().map(|n| n.parse::<i64>().expect("{n} is not an integer"))
            .collect()
        })
        .collect()
}