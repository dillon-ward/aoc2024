use std::fs;

type List = Vec<String>;

fn main() {
    let input = get_input("day4/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(l: &List) -> u64 {
    let mut result = 0;
    for i in 0..l.len() {
        let mut pos = 0;
        while let Some(p) = l[i][pos..].find('X') {
            pos += p;
            
            // N
            if  i >= 3 && 
                l[i-1].chars().nth(pos) == Some('M') && 
                l[i-2].chars().nth(pos) == Some('A') && 
                l[i-3].chars().nth(pos) == Some('S') {result += 1}

            // NE
            if  i >= 3 && pos <= l[i].len() - 4 && 
                l[i-1].chars().nth(pos+1) == Some('M') && 
                l[i-2].chars().nth(pos+2) == Some('A') && 
                l[i-3].chars().nth(pos+3) == Some('S') {result += 1}

            // E
            if  pos <= l[i].len() - 4 && 
                l[i][pos+1..pos+4] == *"MAS" {result += 1}

            // SE
            if  i <= l.len() - 4 && pos <= l[i].len() - 4 &&
                l[i+1].chars().nth(pos+1) == Some('M') && 
                l[i+2].chars().nth(pos+2) == Some('A') && 
                l[i+3].chars().nth(pos+3) == Some('S') {result += 1}

            // S
            if  i <= l.len() - 4 && 
                l[i+1].chars().nth(pos) == Some('M') && 
                l[i+2].chars().nth(pos) == Some('A') && 
                l[i+3].chars().nth(pos) == Some('S') {result += 1}

            // SW
            if  i <= l.len() - 4 && pos >= 3 && 
                l[i+1].chars().nth(pos-1) == Some('M') && 
                l[i+2].chars().nth(pos-2) == Some('A') && 
                l[i+3].chars().nth(pos-3) == Some('S') {result += 1}

            // W
            if  p >= 3 && 
                l[i][pos-3..pos] == *"SAM" {result += 1;}

            // NW
            if  i >= 3 && pos >= 3 && 
                l[i-1].chars().nth(pos-1) == Some('M') && 
                l[i-2].chars().nth(pos-2) == Some('A') && 
                l[i-3].chars().nth(pos-3) == Some('S') {result += 1}

            pos += 1;
        }
    }
    result
}

fn part2(l: &List) -> u64 {
    let mut result = 0;
    for i in 1..l.len()-1 {
        let mut pos = 1;
        while let Some(p) = l[i][pos..].find('A') {
            pos += p;

            if  pos <= l.len() - 2 &&
                (   (l[i-1].chars().nth(pos-1) == Some('M') && l[i+1].chars().nth(pos+1) == Some('S')) || 
                    (l[i-1].chars().nth(pos-1) == Some('S') && l[i+1].chars().nth(pos+1) == Some('M'))  ) &&
                (   (l[i-1].chars().nth(pos+1) == Some('M') && l[i+1].chars().nth(pos-1) == Some('S')) || 
                    (l[i-1].chars().nth(pos+1) == Some('S') && l[i+1].chars().nth(pos-1) == Some('M'))  ) {result += 1}

            pos += 1;
        }
    }
    result
}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.chars()
            .collect()
        })
        .collect()
}