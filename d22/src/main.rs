use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

type Change4 = (i64, i64, i64, i64);
struct MonkeySecret {
    secret: i64,
    gen: usize,
    change_map: HashMap<Change4, i64>,
    change_history: Vec<i64>
}

static debug: bool = false;

impl MonkeySecret {
    fn new(initial_secret: i64) -> MonkeySecret {
        MonkeySecret {
            secret: initial_secret,
            gen: 0,
            change_map: HashMap::new(),
            change_history: vec![]
        }
    }

    fn run(&mut self, count: usize) {
        for _ in 0..count {
            self.tick();
        }
    }

    fn tick(&mut self) {
        let initial_price = self.secret % 10;

        let new_val = self.secret * 64;
        self.mix(new_val);
        self.prune();

        let new_val = self.secret / 32;
        self.mix(new_val);
        self.prune();

        let new_val = self.secret * 2048;
        self.mix(new_val);
        self.prune();

        let final_price = self.secret % 10;
        let delta_price = final_price - initial_price;

        if self.change_history.len() >= 4 {
            self.change_history.remove(0);
        }

        self.change_history.push(delta_price);
        if debug {
            println!("{}: {} ({})", self.secret, final_price, delta_price);
        }
        if self.change_history.len() == 4 {
            let delta4: Change4 = self.change_history
                .iter()
                .map(|i| *i)
                .collect_tuple().unwrap();
            if debug {
                println!("{:?}", delta4);
            }
            if !self.change_map.contains_key(&delta4) {
                self.change_map.insert(delta4, final_price);
            }
        }

        self.gen += 1;
    }

    fn mix(&mut self, val:i64) {
        self.secret = self.secret ^ val;
    }

    fn prune(&mut self) {
        self.secret = self.secret % 16777216;
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut monkeys = load_monkeys();

    let sum: i64 = monkeys.iter_mut()
        .map(|monkey| {
            monkey.run(2000);
            monkey.secret
        })
        .sum();
    println!("Part 1: {sum}");
}

fn part2() {
    let mut monkeys = load_monkeys();
    for monkey in monkeys.iter_mut() {
        monkey.run(2000);
    }

    let mut candidates: HashSet<Change4> = HashSet::new();
    for monkey in monkeys.iter() {
        candidates.extend(monkey.change_map.keys())
    }

    println!("Looking at {} candidates", candidates.len());
    let candidate_map: HashMap<Change4, i64> = candidates.iter()
        .map(|candidate| {
            let score: i64 = monkeys.iter()
                .map(|monkey| monkey.change_map.get(candidate).unwrap_or(&0))
                .sum();
            (*candidate, score)
        })
        .collect();

    let (changes, score) = candidate_map.iter()
        .max_by_key(|(_change, score)| **score)
        .unwrap();

    println!("Part 2: {:?} - {}", changes, score);
}

fn load_monkeys() -> Vec<MonkeySecret> {
    fs::read_to_string("d22/input")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .map(|val| MonkeySecret::new(val))
        .collect()
}