use std::fs;
use std::collections::HashSet;

type List = Vec<String>;
type Dir = (isize, isize);

const DIRS: [Dir; 4] = [
    ( 0, -1), // N
    ( 1,  0), // E
    ( 0,  1), // S
    (-1,  0)  // W
];

fn turn(d: Dir) -> Dir {
    match d {
        ( 0, -1) => ( 1,  0),
        ( 1,  0) => ( 0,  1),
        ( 0,  1) => (-1,  0),
        (-1,  0) => ( 0, -1),
        _ => panic!()
    }
}

fn main() {
    let input = get_input("day6/input.txt");
    let init_pos = get_init_pos(&input);
    println!("Part 1: {}", part1(&input, init_pos));
    println!("Part 2: {}", part2(&input, init_pos));
}

fn get_init_pos(input: &List) -> Dir {
    for (i, s) in input.iter().enumerate() {
        match s.find("^") {
            Some(p) => {
                return (p as isize, i as isize)
            }
            None => continue
        }
    }
    panic!()
}

fn calculate_new_position(x: isize, y: isize, d: Dir) -> (isize, isize) {
    (x + d.0, y + d.1)
}

fn is_within_bounds(new_x: isize, new_y: isize, len: Dir) -> bool {
    new_x >= 0 && new_x < len.0 && new_y >= 0 && new_y < len.1
}

fn is_obstacle(input: &List, new_x: isize, new_y: isize) -> bool {
    input[new_y as usize].chars().nth(new_x as usize) == Some('#')
}

fn get_path(input: &List, (gx, gy): Dir) -> Vec<Dir> {
    let mut path = vec![];
    path.push((gx, gy));
    let (mut x, mut y) = (gx, gy);
    let len = (input[0].len() as isize, input.len() as isize);
    let mut d = DIRS[0];
    loop {
        let (new_x, new_y) = calculate_new_position(x, y, d);
        if !is_within_bounds(new_x, new_y, len) {
            break;
        }
        if is_obstacle(input, new_x, new_y) {
            d = turn(d);
            continue;
        }
        x = new_x;
        y = new_y;
        if !path.contains(&(x, y)) {
            path.push((x, y));
        }
    }
    path
}

fn check_loop(input: &List, dir: &mut Dir, ox: isize, oy: isize, pos: &mut Dir, len: Dir) -> bool {
    let mut pivots = HashSet::new();
    let (mut x, mut y) = *pos;
    *pos = (ox, oy);
    let mut d = *dir;
    let (new_x, new_y) = calculate_new_position(x, y, d);
    if is_within_bounds(new_x, new_y, len) && is_obstacle(input, new_x, new_y) {
        *dir = turn(d);
    }
    loop {
        let (new_x, new_y) = calculate_new_position(x, y, d);
        if !is_within_bounds(new_x, new_y, len) {
            return false;
        }
        if is_obstacle(input, new_x, new_y) || (new_x, new_y) == (ox, oy) {
            if pivots.contains(&((x, y), d)) {
                return true;
            }
            pivots.insert(((x, y), d));
            d = turn(d);
            continue;
        }
        x = new_x;
        y = new_y;
    }
}

fn part1(input: &List, init_pos: Dir) -> usize {
    get_path(input, init_pos).len()
}

fn part2(input: &List, init_pos: Dir) -> u64 {
    let mut path = get_path(input, init_pos);
    path.remove(0);
    let mut dir = DIRS[0];
    let mut pos = init_pos;
    let len = (input[0].len() as isize, input.len() as isize);
    path.iter()
        .filter(|(x, y)| check_loop(input, &mut dir, *x, *y, &mut pos, len))
        .count() as u64
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