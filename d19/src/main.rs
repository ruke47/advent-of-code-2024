use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let (towels, patterns) = load_towels();
    let possible = patterns
        .iter()
        .filter(|pattern| can_be_made_with_towels(pattern, &towels))
        .count();

    println!("Part 1: {possible}");
}

fn part2() {
    let (towels, patterns) = load_towels();

    let mut memo = HashMap::new();
    let sum: usize = patterns.iter()
        .map(|pattern| count_ways_that_can_be_made(pattern, &towels, &mut memo))
        .sum();

    println!("Part 2: {sum}");
}

fn can_be_made_with_towels(pattern: &str, towels: &Vec<String>) -> bool {
    towels.iter().any(|towel| {
        if pattern == towel {
            true
        } else if let Some(sub_pattern) = pattern.strip_prefix(towel) {
            can_be_made_with_towels(sub_pattern, towels)
        } else {
            false
        }
    })
}

fn count_ways_that_can_be_made(pattern: &str,
                               towels: &Vec<String>,
                               memo: &mut HashMap<String, usize>) -> usize {
    if let Some(count) = memo.get(pattern) {
        return *count;
    }
    
    let count = towels
        .iter()
        .map(|towel| {
            if pattern == towel {
                1
            } else if let Some(sub_pattern) = pattern.strip_prefix(towel) {
                count_ways_that_can_be_made(sub_pattern, towels, memo)
            } else {
                0
            }
        })
        .sum();
    memo.insert(pattern.to_string(), count);
    count
}

fn load_towels() -> (Vec<String>, Vec<String>) {
    let file = fs::read_to_string("d19/input").unwrap();

    let (towels, patterns) = file.split("\n\n").collect_tuple().unwrap();

    let towels = towels.split(", ").map(|s| s.to_string()).collect_vec();

    let patterns = patterns.lines().map(|s| s.to_string()).collect_vec();

    (towels, patterns)
}
