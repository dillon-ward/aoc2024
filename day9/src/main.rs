use std::fs;

fn main() {
    let input = get_input("day9/input.txt");
    let mut blocks = get_blocks(&input);
    println!("Part 1: {}", part1(&blocks));
    println!("Part 2: {}", part2(&mut blocks));
}

fn get_blocks(input: &str) -> Vec<i64> {
    let mut blocks = vec![];
    let mut id = 0;

    for (i, c) in input.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..n {blocks.push(id)}
            id += 1;
        }
        else {
            for _ in 0..n {blocks.push(-1_i64)}
        }
    }

    blocks
}

fn part1(blocks: &[i64]) -> u64 {
    let mut checksum = 0;
    let mut last_i = blocks.len() - 1;

    for (i, n) in blocks.iter().enumerate() {
        while blocks[last_i] == -1_i64 {last_i -= 1;}
        if i > last_i {break}
        if *n == -1_i64 {
            checksum += i as i64 * blocks[last_i];
            last_i -= 1;
            continue
        }
        checksum += i as i64 * n
    }

    checksum as u64
}

fn part2(blocks: &mut [i64]) -> u64 {
    let mut checksum = 0;
    let mut last_i: isize = (blocks.len() - 1) as isize;

    while last_i >= 0 {

        while last_i >= 0 && blocks[last_i as usize] == -1_i64 {last_i -= 1}
        if last_i < 0 {break}

        let n = blocks[last_i as usize];
        let end_file_i = last_i;

        while last_i >= 0 && blocks[last_i as usize] == n {last_i -= 1}
        if last_i < 0 {break}

        last_i += 1;

        let mut found_space = false;
        let mut start_i = 0;
        let mut check_space = false;

        for i in 0..last_i {

            if check_space && blocks[i as usize] != -1 {
                check_space = false;
                continue
            }

            if !check_space && blocks[i as usize] == -1 {
                check_space = true;
                start_i = i;
            }

            if blocks[i as usize] == -1 && i - start_i == end_file_i - last_i {
                blocks[start_i as usize..(i+1) as usize].fill(n);
                blocks[last_i as usize..(end_file_i+1) as usize].fill(-1);
                found_space = true;
            } 

            if found_space {break}
        }

        last_i -= 1;
    }

    for (i, n) in blocks.iter().enumerate() {
        if *n != -1_i64 {checksum += i as i64 * n}
    }

    checksum as u64
}

fn get_input(filename: &str) -> String {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .collect()
}