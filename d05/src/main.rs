use std::collections::{HashMap, HashSet};
use std::fs;

struct Rule {
    pub before: u64,
    pub after: u64,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (rules, sections) = read_input();

    // for a given KEY, all VALUES must occur *after* key
    let after_map = build_after_map(rules);

    let mut score = 0;
    for section in sections {
        if is_valid(&after_map, &section) {
            let middle = section.len() / 2;
            score += section[middle];
        }
    }

    println!("Part 1: {score}");
}

fn build_after_map(rules: Vec<Rule>) -> HashMap<u64, HashSet<u64>> {
    let mut after_map: HashMap<u64, HashSet<u64>> = HashMap::new();

    for rule in rules {
        let mut list = after_map
            .remove(&rule.before)
            .unwrap_or_else(|| HashSet::new());
        list.insert(rule.after);
        after_map.insert(rule.before, list);
    }
    after_map
}

fn is_valid(after_map: &HashMap<u64, HashSet<u64>>, section: &Vec<u64>) -> bool {
    let mut seen_pages = HashSet::new();
    for &page in section.iter() {
        seen_pages.insert(page);
        match after_map.get(&page) {
            None => {}
            Some(invalid_befores) => {
                if invalid_befores.is_disjoint(&seen_pages) {
                    return false;
                }
            }
        }
    }
    true
}

fn part2() {}

fn read_input() -> (Vec<Rule>, Vec<Vec<u64>>) {
    let file_str = fs::read_to_string("d05/input").unwrap();
    let mut parts = file_str.split("\n\n");

    let rules: Vec<Rule> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split("|").collect();
            Rule {
                before: parts[0].parse().unwrap(),
                after: parts[1].parse().unwrap(),
            }
        })
        .collect();

    // parse the sections
    let sections: Vec<Vec<u64>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();

    (rules, sections)
}
