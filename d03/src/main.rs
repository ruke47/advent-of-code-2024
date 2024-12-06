use crate::Instruction::{MUL, START, STOP};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs;

lazy_static! {
    pub static ref MUL_INSTR: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pub static ref ENABLE_INSTR: Regex = Regex::new(r"do\(\)").unwrap();
    pub static ref DISABLE_INSTR: Regex = Regex::new(r"don't\(\)").unwrap();
}

enum Instruction {
    MUL(u64),
    START,
    STOP
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = read_input();
    let sum: u64 = MUL_INSTR.captures_iter(&input)
        .map(|hit| get_mul_value(&hit))
        .sum();

    println!("Part 1: {sum}");
}

fn part2() {
    let input = read_input();
    
    // index, instruction
    let mut all_instr: Vec<(usize, Instruction)> = vec![];
    MUL_INSTR.captures_iter(&input)
        .map(|hit| (hit.get(0).unwrap().start(), MUL(get_mul_value(&hit))))
        .for_each(|instr| all_instr.push(instr));
    ENABLE_INSTR.captures_iter(&input)
        .map(|hit| (hit.get(0).unwrap().start(), START))
        .for_each(|instr| all_instr.push(instr));
    DISABLE_INSTR.captures_iter(&input)
        .map(|hit| (hit.get(0).unwrap().start(), STOP))
        .for_each(|instr| all_instr.push(instr));
    
    // sort all instructions by index
    all_instr.sort_by_key(|i| i.0);
    
    // state machine
    let mut sum = 0;
    let mut enabled = true;
    all_instr.iter().for_each(|(_, instruction)| match instruction {
        START => enabled = true,
        STOP => enabled = false,
        MUL(val) => if enabled {
            sum += val;
        }
    });
    
    println!("Part 2: {sum}");
}

fn get_mul_value(hit: &Captures) -> u64 {
    let val1: u64 = hit.get(1).unwrap().as_str().parse().unwrap();
    let val2: u64 = hit.get(2).unwrap().as_str().parse().unwrap();
    val1 * val2
}

fn read_input() -> String {
    fs::read_to_string("d03/input").unwrap()
}