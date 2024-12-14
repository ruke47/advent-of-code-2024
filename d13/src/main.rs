use lazy_static::lazy_static;
use lib2d::Point2d;
use regex::Regex;
use std::fs;

lazy_static! {
    pub static ref MACHINE_DEF: Regex = Regex::new(
        r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)
Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)
Prize: X=(?<px>\d+), Y=(?<py>\d+)"
    )
    .unwrap();
}

type Point = Point2d<i64>;

#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let machines = read_machines(0);
    println!("Got {:?} machines", machines.len());
    let cost: usize = machines
        .iter()
        .map(get_working_counts)
        .flat_map(|list| list.iter().map(|(a, b)| a * 3 + b * 1).min())
        .sum();

    println!("Part 1: {cost}");
}

fn part2() {
    let big_time: i64 = 10000000000000;
    let machines = read_machines(big_time);

    let cost: i64 = machines
        .iter()
        .flat_map(|machine| {
            // wait a minute, this is just ALGEBRA!
            let numerator = (machine.prize.x * machine.b.y) - (machine.b.x * machine.prize.y);
            let denominator = (machine.a.x * machine.b.y) - (machine.b.x * machine.a.y);

            if numerator % denominator != 0 {
                return None;
            }

            let a_count = numerator / denominator;

            let b_numerator = machine.prize.x - (machine.a.x * a_count);

            if b_numerator % machine.b.x != 0 {
                // apparently this is fine
                println!(
                    "I got an a_count of {a_count}, but b_count was non integer! {:?}",
                    machine
                );
                return None;
            }

            let b_count = b_numerator / machine.b.x;
            let test_point = machine.a * a_count + machine.b * b_count;
            if test_point != machine.prize {
                panic!(
                    "I thought A={a_count} B={b_count} would give me {:?}, but I got {:?}",
                    machine.prize, test_point
                );
            }

            Some((a_count, b_count))
        })
        .map(|(a_count, b_count)| a_count * 3 + b_count)
        .sum();

    println!("Part 2: {cost}");
}

fn get_working_counts(machine: &Machine) -> Vec<(usize, usize)> {
    let mut working_counts = vec![];

    for a_count in 0..100 {
        let a_offset = machine.a * a_count;
        if a_offset.x > machine.prize.x || a_offset.y > machine.prize.y {
            break;
        }
        for b_count in 0..100 {
            let total_offset = a_offset + machine.b * b_count;
            if total_offset == machine.prize {
                working_counts.push((a_count as usize, b_count as usize));
                break;
            }

            if total_offset.x > machine.prize.x || total_offset.y > machine.prize.y {
                break;
            }
        }
    }

    working_counts
}

fn read_machines(prize_offset: i64) -> Vec<Machine> {
    MACHINE_DEF
        .captures_iter(&fs::read_to_string("d13/input").unwrap())
        .map(|caps| {
            let ax = caps["ax"].parse().unwrap();
            let ay = caps["ay"].parse().unwrap();
            let bx = caps["bx"].parse().unwrap();
            let by = caps["by"].parse().unwrap();
            let px: i64 = caps["px"].parse().unwrap();
            let py: i64 = caps["py"].parse().unwrap();

            Machine {
                a: Point2d::new(ax, ay),
                b: Point2d::new(bx, by),
                prize: Point2d::new(px + prize_offset, py + prize_offset),
            }
        })
        .collect()
}
