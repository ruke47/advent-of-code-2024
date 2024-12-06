use lib2d::{corners, Point2d};
use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let puzzle = read_puzzle();
    let (min, max) = corners(puzzle.keys()).unwrap();

    let deltas = [
        Point2d::new(-1, -1),
        Point2d::new(-1, 0),
        Point2d::new(-1, 1),
        Point2d::new(0, -1),
        Point2d::new(0, 1),
        Point2d::new(1, -1),
        Point2d::new(1, 0),
        Point2d::new(1, 1)
    ];

    let mut count = 0;
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let cur_pos = Point2d::new(x, y);
            for delta in deltas {
                if check_xmas(&puzzle, &cur_pos, &delta) {
                    count += 1
                }
            }
        }
    }

    println!("Part 1: {count}");
}

fn check_xmas(puzzle: &HashMap<Point2d<i32>, char>,
              cur_pos: &Point2d<i32>,
              delta: &Point2d<i32>) -> bool {
    let mut cur_pos = cur_pos.clone();
    for letter in "XMAS".chars() {
        if puzzle.get(&cur_pos) != Some(&letter) {
            return false;
        }
        cur_pos = cur_pos + *delta;
    }

    return true;
}

fn part2() {
    let puzzle = read_puzzle();
    let (min, max) = corners(puzzle.keys()).unwrap();

    let deltas = [
        Point2d::new(-1, -1),
        Point2d::new(-1, 1),
        Point2d::new(1, -1),
        Point2d::new(1, 1)
    ];

    let mut count = 0;
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let cur_point = Point2d::new(x, y);
            if puzzle.get(&cur_point) == Some(&'A') {
                let mases = deltas.iter()
                    .filter(|delta| is_mas(&puzzle, &cur_point, delta))
                    .count();
                if mases == 2 {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {count}");
}

fn is_mas(puzzle: &HashMap<Point2d<i32>, char>, 
          cur_point: &Point2d<i32>, 
          delta: &Point2d<i32>) -> bool {
    let m_pos = *cur_point + *delta;
    let s_pos = *cur_point - *delta;
    
    puzzle.get(&m_pos) == Some(&'M') && puzzle.get(&s_pos) == Some(&'S')
}

fn read_puzzle() -> HashMap<Point2d<i32>, char> {
    fs::read_to_string("d04/input").unwrap()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| (Point2d::new(x as i32, y as i32), char))
        })
        .collect()
}