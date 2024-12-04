use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (col1, col2) = read_pairs();

    let sum_diffs: u64 = col1.iter().zip(col2.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum();

    println!("Part1: {sum_diffs}")
}

fn part2() {
    let (col1, col2) = read_pairs();

    let mut right_counts: HashMap<u64, u64> = HashMap::new();
    for val in col2.iter() {
        let count = right_counts.get(val).unwrap_or(&0) + 1;
        right_counts.insert(*val, count);
    }

    let left_score: u64 = col1.iter()
        .map(|left_val| left_val * right_counts.get(left_val).unwrap_or(&0))
        .sum();

    println!("Part 2: {left_score}");
}

fn read_pairs() -> (Vec<u64>, Vec<u64>) {
    let mut col1: Vec<u64> = Vec::new();
    let mut col2: Vec<u64> = Vec::new();
    fs::read_to_string("d01/input")
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut parts = line.split("   ");

            col1.push(parts.next().unwrap().parse().unwrap());
            col2.push(parts.next().unwrap().parse().unwrap());
        });
    col1.sort();
    col2.sort();
    (col1, col2)
}