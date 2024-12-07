use std::fs;
use bit_iter::BitIter;

type Eq = (u64, Vec<u64>);

fn main() {
    let input = get_input("day7/input.txt");
    let p1_res = part1(&input);
    println!("Part 1: {}", p1_res);
    println!("Part 2: {}", part2(&input, p1_res));
}

fn check_eq((test_val, nums): Eq) -> bool {
    let tests = 2_u64.pow((nums.len()-1).try_into().unwrap());
    for i in 0..tests {
        let mut result = 0;
        let mut pos = 0;
        for b in BitIter::from(i) {
            result += nums[pos..b+1].iter().sum::<u64>();
            result *= nums[b+1]; 
            pos = b + 2;
        }
        result += nums[pos..].iter().sum::<u64>();
        if result == test_val {return true}
    }

    false
}

fn check_eq_concat((test_val, nums): Eq) -> bool {
    let tests = 2_u64.pow((nums.len()-1).try_into().unwrap());
    for i in 1..tests {
        let mut results = vec![0];
        let mut pos = 0;
        for b in BitIter::from(i) {
            let mut temp_results = vec![];
            for r in results {
                let temp: u64 = r + nums[pos..b+1].iter().sum::<u64>();
                temp_results.push(temp * nums[b+1]);
                temp_results.push((temp.to_string() + &nums[b+1].to_string()).parse::<u64>().expect("{n} is not an integer"))
            }
            pos = b + 2;
            results = temp_results;
        }
        for mut r in results {
            r += nums[pos..].iter().sum::<u64>();
            if r == test_val {return true}
        }
    }

    false
}

fn part1(input: &[Eq]) -> u64 {
    input
        .iter()
        .filter(|&e| check_eq(e.clone()))
        .map(|e| e.0)
        .sum()
}

fn part2(input: &[Eq], p1_res: u64) -> u64 {
    input
        .iter()
        .filter(|&e| !check_eq(e.clone()))
        .filter(|&e| check_eq_concat(e.clone()))
        .map(|e| e.0)
        .sum::<u64>() + p1_res
}

fn get_input(filename: &str) -> Vec<Eq> {
    fs::read_to_string(filename).expect("Should have read file")
        .lines()
        .map(|l| {
            match l.find(":") {
                Some(p) => {
                    (l[..p].parse::<u64>().expect("{n} is not an integer"),
                    l[p+1..].split_ascii_whitespace().map(|n| n.parse::<u64>().expect("{n} is not an integer")).collect())
                }
                None => {panic!()}
            }
        })
        .collect()
}