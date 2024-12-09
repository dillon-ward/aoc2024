use std::fs;
use std::collections::HashSet;

type List = Vec<String>;
type Pos = (isize, isize);
type FnGetAntinodes = fn(&List, Pos, Pos) -> Vec<Pos>;

fn main() {
    let input = get_input("day8/input.txt");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn check_bounds(input: &List, pos: Pos) -> bool {
    let len: (isize, isize) = (input[0].len() as isize, input.len() as isize);
        if pos.0 >= 0 && pos.0 < len.0 && pos.1 >= 0 && pos.1 < len.1 {
            return true
        }

    false
}

fn get_antinodes_single(input: &List, a1: Pos, a2: Pos) -> Vec<Pos> {
    let mut antinodes = vec![];
    let ant1 = (2 * a1.0 - a2.0, 2 * a1.1 - a2.1);
    let ant2 = (2 * a2.0 - a1.0, 2 * a2.1 - a1.1);
    if check_bounds(input, ant1) {antinodes.push(ant1);}
    if check_bounds(input, ant2) {antinodes.push(ant2);}

    antinodes
}

fn get_antinodes_many(input: &List, a1: Pos, a2: Pos) -> Vec<Pos> {
    let mut antinodes = vec![];
    let diff = (a1.0 - a2.0, a2.1 - a1.1);
    let mut ant1 = a1;
    let mut ant2 = a2;
    while check_bounds(input, ant1) {
        antinodes.push(ant1);
        ant1 = (ant1.0 + diff.0, ant1.1 - diff.1);
    }
    while check_bounds(input, ant2) {
        antinodes.push(ant2);
        ant2 = (ant2.0 - diff.0, ant2.1 + diff.1);
    }

    antinodes
}

fn get_locations(input: &List, get_antinodes: FnGetAntinodes) -> usize {
    let mut locations = HashSet::new();
    for (i, s) in input.iter().enumerate() {
        let mut pos = 0;
        while let Some(p) =  s[pos..].find(|c| c != '.') {
            let x = pos + p;
            let antenna = s.chars().nth(x).unwrap();
            for (j, t) in input[i+1..].iter().enumerate() {
                match t.find(antenna) {
                    Some(q) => {
                        for ant in 
                            get_antinodes(input, (x as isize, i as isize), (q as isize, (j + i + 1) as isize)) {
                                locations.insert(ant);
                        }
                    }
                    None => {continue}
                }
            }
            pos += p + 1;
        }
    }

    locations.len()
}

fn part1(input: &List) -> usize {get_locations(input, get_antinodes_single)}

fn part2(input: &List) -> usize {get_locations(input, get_antinodes_many)}

fn get_input(filename: &str) -> List {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            l.chars()
            .collect()
        })
        .collect()
}