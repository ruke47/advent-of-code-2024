use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Clone)]
struct Rule {
    pub before: u64,
    pub after: u64,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    // part1 is now covered in part 2
}

fn part2() {
    let (rules, sections) = read_input();

    let mut sorted_score = 0;
    let mut unsorted_score = 0;
    for section in sections {
        let ordered_pages = get_ordered_pages(&rules, &section);
        let page_to_index: HashMap<&u64, usize> = pages_to_index_map(&ordered_pages);
        
        let mut sorted_section = section.clone();
        sorted_section.sort_by_key(|page| page_to_index.get(page));

        let middle = sorted_section[sorted_section.len() / 2];

        match section.eq(&sorted_section) {
            true => sorted_score += middle,
            false => unsorted_score += middle
        }
    }

    println!("Part 2: already sorted: {sorted_score} re-sorted: {unsorted_score}");
}

fn pages_to_index_map(ordered_pages: &Vec<u64>) -> HashMap<&u64, usize> {
    ordered_pages.iter()
        .enumerate()
        .map(|(idx, page)| (page, idx))
        .collect()
}

fn get_ordered_pages(rules: &Vec<Rule>, section: &Vec<u64>) -> Vec<u64> {
    // find all distinct page names
    let mut unordered_pages: HashSet<u64> = section.iter().map(|i| i.clone()).collect();;

    // remove all rules that don't matter
    let mut rules: Vec<&Rule> = rules.iter()
        .filter(|rule| {
            unordered_pages.contains(&rule.after) && unordered_pages.contains(&rule.before)
        })
        .collect();
    
    let mut ordered_pages: Vec<u64> = vec![];
    while !unordered_pages.is_empty() {
        // figure out which pages come after something else
        let right_pages: HashSet<u64> = rules.iter()
            .map(|rule| rule.after.clone())
            .collect();

        // figure out which page doesn't come after any remaining pages
        let nothing_before: HashSet<_> = unordered_pages.difference(&right_pages).collect();
        if nothing_before.len() != 1 {
            panic!("I fucked up! The pages with nothing before them are {:?}", nothing_before);
        }
        let next_page = nothing_before.into_iter().next().unwrap().clone();

        // remove the page from the list
        unordered_pages.remove(&next_page);
        // remove any rules that say things must come after this page; we know that!
        rules = rules.into_iter().filter(|rule| rule.before != next_page).collect();
        // add this page to the ordered list
        ordered_pages.push(next_page);
    }

    ordered_pages
}

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
