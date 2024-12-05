use std::fs;

type List = Vec<u64>;

fn main() {
    let (ordering, updates) = get_input("day5/input.txt");
    let part1_res = part1(&ordering, &updates);
    println!("Part 1: {}", part1_res);
    println!("Part 2: {}", part2(&ordering, &updates, part1_res));
}

fn get_procs(ordering: &[List], n: u64) -> List {
    ordering
                .iter()
                .filter(|&l| l[0] == n)
                .map(|l| l[1])
                .collect()
}

fn part1(ordering: &[List], updates: &[List]) -> u64 {
    let mut result = 0;
    for u in updates {
        let len = u.len();
        let mut valid = true;
        for i in 0..len {
            let orders = get_procs(ordering, u[i]);
            for n in u.iter().skip(i+1) {
                if !orders.contains(n) {
                    valid = false;
                    break
                }
            }
            if !valid {break}
        }
        if valid {result += u[len/2]}
    }
    result
}

fn part2(ordering: &[List], updates: &[List], part1_res: u64) -> u64 {
    let mut result = 0;
    for u in updates {
        let len = u.len();
        let mut new_u = u.clone();
        for i in 0..len/2+1 {
            loop {
                let mut valid = true;
                let orders = get_procs(ordering, new_u[i]);
                for j in i+1..len {
                    if !orders.contains(&new_u[j]) {
                        new_u.swap(i, j);
                        valid = false;
                        break
                    }
                }
                if valid {break}
            }
        }
        result += new_u[len/2];
    }
    result - part1_res
}

fn get_input(filename: &str) -> (Vec<List>, Vec<List>) {
    let mut ordering = vec![];
    let mut updates = vec![];
    let file = fs::read_to_string(filename).expect("Should have read file");
    let lines = file.lines();
    let mut parse_updates = false;
    for line in lines{
        if parse_updates {
            updates.push(line.split(",").map(|n| n.parse::<u64>().expect("{n} is not an integer")).collect());
            continue
        }
        if line.is_empty() {
            parse_updates = true;
            continue
        }
        ordering.push(line.split("|").map(|n| n.parse::<u64>().expect("{n} is not an integer")).collect());
    }

    (ordering, updates)
}
