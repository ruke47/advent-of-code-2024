use itertools::Itertools;
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Point2d};
use std::fs;

type Point = Point2d<i32>;

fn main() {
    part1();
    part2();
}

fn part1() {
    let codes = load_codes("d21/example1");
    let tenkey_paths = convert_codes(&codes, get_tenkey_path);

    print_paths(&codes, &tenkey_paths);

    let rad_paths = convert_codes(&tenkey_paths, get_cross_path);
    print_paths(&codes, &rad_paths);

    let frozen_paths = convert_codes(&rad_paths, get_cross_path);
    print_paths(&codes, &frozen_paths);

    // let lobby_paths = convert_codes(&frozen_paths, get_cross_path);
    // print_paths(&codes, &lobby_paths);

    let score: usize = codes.iter().zip(frozen_paths.iter())
        .map(|(code, path)| {
            let code_int: usize = code[..3].parse().unwrap();
            code_int * path.len()
        })
        .sum();

    println!("Part 1: {score}");
}

fn part2() {}

fn print_paths(codes: &Vec<String>, paths: &Vec<String>) {

    for (code, path) in codes.iter().zip(paths.iter()) {
        println!("{}: {} ({})", code, path, path.len());
    }
    println!();
}

fn convert_codes(codes: &Vec<String>, get_path_fn: impl Fn(char, char) -> String) -> Vec<String> {
    codes.iter()
        .map(|code| {
            let mut full_code = String::from("A");
            full_code.push_str(code);

            full_code.chars()
                .tuple_windows()
                .map(|(from, to)| get_path_fn(from, to))
                .join("")
        })
        .collect()
}

fn cross_instr(button: char) -> String {
    match button {
        'A' => String::from("A"),
        '^' => String::from("<A"),
        'v' => String::from("<vA"),
        '<' => String::from("v<<A"),
        '>' => String::from("vA"),
        _ => panic!("Can't go to {button} on a cross-pad!")
    }
}

fn cross_layout(button: char) -> Point {
    match button {
        'A' => Point2d::new(0, 0),
        '^' => Point2d::new(-1, 0),
        '>' => Point2d::new(0, 1),
        'v' => Point2d::new(-1, 1),
        '<' => Point2d::new(-2, 1),
        _ => panic!("Cross does not contain {button}!")
    }
}

fn get_cross_path(start: char, end:char) -> String {
    let mut path = String::new();
    let goal = cross_layout(end);

    let mut cur_point = cross_layout(start);
    while cur_point.y < goal.y {
        path.push('v');
        cur_point = cur_point + dir_delta(Down);
    }
    while cur_point.x > goal.x {
        path.push('<');
        cur_point = cur_point + dir_delta(Left);
    }
    while cur_point.x < goal.x {
        path.push('>');
        cur_point = cur_point + dir_delta(Right);
    }
    while cur_point.y > goal.y {
        path.push('^');
        cur_point = cur_point + dir_delta(Up);
    }

    path.push('A');
    path
}

fn tenkey_layout(button: char) -> Point {
    match button {
        'A' => Point2d::new(0, 0),
        '0' => Point2d::new(-1, 0),
        '3' => Point2d::new(0, -1),
        '2' => Point2d::new(-1, -1),
        '1' => Point2d::new(-2, -1),
        '6' => Point2d::new(0, -2),
        '5' => Point2d::new(-1, -2),
        '4' => Point2d::new(-2, -2),
        '9' => Point2d::new(0, -3),
        '8' => Point2d::new(-1, -3),
        '7' => Point2d::new(-2, -3),
        _ => panic!("Tenkey does not contain {button}!")
    }
}

fn get_tenkey_path(start: char, end: char) -> String {
    let mut path = String::new();
    let goal = tenkey_layout(end);

    let mut cur_point = tenkey_layout(start);
    while cur_point.x < goal.x {
        path.push('>');
        cur_point = cur_point + dir_delta(Right);
    }
    while cur_point.y > goal.y {
        path.push('^');
        cur_point = cur_point + dir_delta(Up);
    }
    while cur_point.x > goal.x {
        path.push('<');
        cur_point = cur_point + dir_delta(Left);
    }
    while cur_point.y < goal.y {
        path.push('v');
        cur_point = cur_point + dir_delta(Down);
    }

    path.push('A');
    path
}

fn load_codes(path: &str) -> Vec<String> {
    fs::read_to_string(path).unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect()
}