use std::fs;

type List = Vec<String>;

fn main() {
    let input = get_input("day4/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn get_xmas(l: &List, i: usize, p:usize) -> u64 {
    let mut finds = 0;

    if i >= 3 && l[i-1].chars().nth(p) == Some('M') && l[i-2].chars().nth(p) == Some('A') && l[i-3].chars().nth(p) == Some('S') {
        finds += 1;
    }

    if i >= 3 && p <= l[i].len() - 4 && l[i-1].chars().nth(p+1) == Some('M') && l[i-2].chars().nth(p+2) == Some('A') && l[i-3].chars().nth(p+3) == Some('S') {
        finds += 1;
    }

    if p <= l[i].len() - 4 && l[i][p+1..p+4] == *"MAS" {
        finds += 1;
    }

    if i <= l.len() - 4 && p <= l[i].len() - 4 && l[i+1].chars().nth(p+1) == Some('M') && l[i+2].chars().nth(p+2) == Some('A') && l[i+3].chars().nth(p+3) == Some('S') {
        finds += 1;
    }

    if i <= l.len() - 4 && l[i+1].chars().nth(p) == Some('M') && l[i+2].chars().nth(p) == Some('A') && l[i+3].chars().nth(p) == Some('S') {
        finds += 1;
    }

    if i <= l.len() - 4 && p >= 3 && l[i+1].chars().nth(p-1) == Some('M') && l[i+2].chars().nth(p-2) == Some('A') && l[i+3].chars().nth(p-3) == Some('S') {
        finds += 1;
    }

    if p >= 3 && l[i][p-3..p] == *"SAM" {
        finds += 1;
    }

    if i >= 3 && p >= 3 && l[i-1].chars().nth(p-1) == Some('M') && l[i-2].chars().nth(p-2) == Some('A') && l[i-3].chars().nth(p-3) == Some('S') {
        finds += 1;
    }

    finds
}

fn get_x_mas(l: &List, i: usize, p:usize) -> u64 {
    if i >= 1 && i <= l.len() - 2 && p >= 1 && p <= l.len() - 2 &&
                ((l[i-1].chars().nth(p-1) == Some('M') && l[i+1].chars().nth(p+1) == Some('S')) || (l[i-1].chars().nth(p-1) == Some('S') && l[i+1].chars().nth(p+1) == Some('M'))) &&
                ((l[i-1].chars().nth(p+1) == Some('M') && l[i+1].chars().nth(p-1) == Some('S')) || (l[i-1].chars().nth(p+1) == Some('S') && l[i+1].chars().nth(p-1) == Some('M'))) {return 1}
    0
}

fn part1(input: &List) -> u64 {
    let mut result = 0;
    for i in 0..input.len() {
        let mut pos = 0;
        while let Some(p) = input[i][pos..].find('X') {
            result += get_xmas(input, i, pos + p);
            pos += p + 1;
        }
    }
    result
}

fn part2(input: &List) -> u64 {
    let mut result = 0;
    for i in 0..input.len() {
        let mut pos = 0;
        while let Some(p) = input[i][pos..].find('A') {
            result += get_x_mas(input, i, pos + p);
            pos += p + 1;
        }
    }
    result
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.split("")
            .collect()
        })
        .collect()
}