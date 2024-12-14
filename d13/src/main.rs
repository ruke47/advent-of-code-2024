use lazy_static::lazy_static;
use lib2d::Point2d;
use num::integer::Roots;
use regex::Regex;
use std::fs;

lazy_static! {
    pub static ref MACHINE_DEF: Regex = Regex::new(r"Button A: X\+(?<ax>\d+), Y\+(?<ay>\d+)
Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)
Prize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
}

type Point = Point2d<i128>;

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
    let cost: usize = machines.iter()
        .map(get_working_counts)
        .flat_map(|list| list.iter()
            .map(|(a, b)| a * 3 + b * 1)
            .min())
        .sum();

    println!("Part 1: {cost}");
}

fn part2() {
    let big_time: i128 = 0;
    let machines = read_machines(big_time);

    let cost: i128 = machines.iter()
        .flat_map(|machine| {
            // we're just using the quadratic formula
            let qa = machine.a.x * machine.a.x;
            let qb = machine.a.x * machine.prize.y;
            let qc = machine.prize.x * machine.b.y;

            // b^2 - 4ac
            let sqrt_part = (qb * qb) - (4 * qa * qc);

            // can't take the square root of a negative
            if sqrt_part < 0 {
                return None
            }

            // return early if sqrt(b^2 - 4ac) isn't an integer
            let root = sqrt_part.sqrt();
            if root * root != sqrt_part {
                return None;
            }

            // -b +/- sqrt(b^2 - 4ac)
            let numerators = [1 * qb + root, -1 * qb - root];
            let denominator = 2 * qa;

            numerators.into_iter()
                .flat_map(|numerator| {
                    if numerator < 0 {
                        None
                    } else if numerator % denominator == 0 {
                        Some(numerator / denominator)
                    } else {
                        None
                    }
                })
                .flat_map(|a_count| {
                    let b_count = (machine.prize.x - (machine.a.x * a_count)) / machine.b.x;
                    if b_count < 0 {
                        return None
                    }
                    let sum_point = (machine.a * a_count) + (machine.b * b_count);
                    if sum_point != machine.prize {
                        panic!("I did math bad! Expected {:?} but got {:?} with A={a_count} B={b_count}",
                               machine.prize, sum_point);
                    }
                    return Some((a_count, b_count))
                })
                .map(|(a_count, b_count)| a_count * 3 + b_count)
                .min()
        })
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

fn read_machines(prize_offset: i128) -> Vec<Machine> {
    MACHINE_DEF.captures_iter(&fs::read_to_string("d13/example").unwrap())
        .map(|caps| {
            let ax = caps["ax"].parse().unwrap();
            let ay = caps["ay"].parse().unwrap();
            let bx = caps["bx"].parse().unwrap();
            let by = caps["by"].parse().unwrap();
            let px: i128 = caps["px"].parse().unwrap();
            let py: i128 = caps["py"].parse().unwrap();

            Machine {
                a: Point2d::new(ax, ay),
                b: Point2d::new(bx, by),
                prize: Point2d::new(px + prize_offset, py + prize_offset),
            }
        })
        .collect()
}