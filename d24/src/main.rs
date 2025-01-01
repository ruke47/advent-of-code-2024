use crate::RuleType::{AND, OR, XOR};
use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

enum RuleType {
    AND,
    OR,
    XOR
}

struct Rule {
    inputs: (String, String),
    rule_type: RuleType,
    output: String
}

impl Rule {
    fn run(&self, a: bool, b: bool) -> bool {
        match self.rule_type {
            AND => a && b,
            OR => a || b,
            XOR => a ^ b
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (mut vals, mut rules) = load_game();

    while !rules.is_empty() {
        let len = rules.len();
        let mut unmatched_rules = vec![];
        for rule in rules {
            let (ref a, ref b) = rule.inputs;
            let a = vals.get(a);
            let b = vals.get(b);
            if let (Some(a), Some(b)) = (a, b) {
                let result = rule.run(*a, *b);
                vals.insert(rule.output.clone(), result);
            } else {
                unmatched_rules.push(rule);
            }
        }
        if len == unmatched_rules.len() {
            panic!("Didn't make any progress, still {len} unmatched rules!");
        }
        rules = unmatched_rules;
    }

    let z_val: i64 = vals.iter()
        .filter(|(key, _val)| key.starts_with('z'))
        .sorted_by_key(|(key, _val)| *key)
        .map(|(_key, value)| value)
        .enumerate()
        .map(|(i, val)| {
            if *val {
                1 << i
            } else {
                0
            }
        })
        .sum();

    println!("Part 1: {z_val}");
}

fn part2() {

}

fn load_game() -> (HashMap<String, bool>, Vec<Rule>) {
    let (input_str, rules_str) = fs::read_to_string("d24/input")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect_tuple()
        .unwrap();

    let inputs = input_str.lines()
        .map(|line| {
            let (name, val) = line.split(": ")
                .collect_tuple()
                .unwrap();
            let val = match val {
                "0" => false,
                "1" => true,
                _ => panic!("Invalid bool {}", val)
            };

            (name.to_string(), val)
        })
        .collect();

    let rules = rules_str
        .lines()
        .map(|line| {
            let (a, op, b, _, o) = line.split(" ").collect_tuple().unwrap();
            Rule {
                inputs: (a.to_string(), b.to_string()),
                rule_type: rule_from_str(op),
                output: o.to_string()
            }
        })
        .collect();

    (inputs, rules)
}

fn rule_from_str(val: &str) -> RuleType {
    match val {
        "AND" => AND,
        "OR" => OR,
        "XOR" => XOR,
        _ => panic!("Invalid rule type {val}")
    }
}