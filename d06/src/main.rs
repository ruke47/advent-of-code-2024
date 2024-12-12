use crate::Tile::{Blocked, Free};
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Direction, Point2d};
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs;


#[derive(PartialEq)]
enum Tile {
    Free,
    Blocked
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (start_point, map) = load_map();
    let (looped, visited_points) = does_maze_loop(&start_point, &map, None);
    if looped {
        panic!("Part 1 looped, it shouldn't do that!");
    }
    let distinct_points: HashSet<_> = visited_points.into_iter()
        .map(|(point, _)| point)
        .collect();
    println!("Part 1: {:?}", distinct_points.len());
}

fn part2() {
    let (start_point, map) = load_map();
    let (_, vanilla_points) = does_maze_loop(&start_point, &map, None);
    
    // find just the points where we travel with no extra obstacles
    let vanilla_points: HashSet<_> = vanilla_points.into_iter()
        .map(|(point, _dir)| point)
        .collect();

    // only consider inserting an obstacle along the original path; other points won't do anything
    let loops = vanilla_points.iter()
        .filter(|point| {
            let (loops, _) = does_maze_loop(&start_point, &map, Some(*point));
            loops
        })
        .count();

    println!("Part 2: {loops}");
}

fn does_maze_loop(start_point: &Point2d<i32>,
                  map: &HashMap<Point2d<i32>, Tile>,
                  introduced_obstacle: Option<&Point2d<i32>>) -> (bool, HashSet<(Point2d<i32>, Direction)>) {
    let mut cur_pos = start_point.clone();
    let mut cur_dir = Up;
    let mut visited_points: HashSet<(Point2d<i32>, Direction)> = HashSet::new();
    visited_points.insert((cur_pos.clone(), cur_dir));
    loop {
        let new_point = cur_pos + dir_delta(cur_dir);

        if visited_points.contains(&(new_point, cur_dir)) {
            return (true, visited_points);
        }

        let destination_tile = if Some(&new_point) == introduced_obstacle {
            Some(&Blocked)
        } else {
            map.get(&new_point)
        };

        match destination_tile {
            None => {
                return (false, visited_points)
            },
            Some(Free) => {
                cur_pos = new_point;
                visited_points.insert((cur_pos.clone(), cur_dir));
            },
            Some(Blocked) => {
                cur_dir = turn_right(cur_dir);
            }
        }
    }
}

fn turn_right(current_direction: Direction) -> Direction {
    match current_direction {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up
    }
}

fn load_map() -> (Point2d<i32>, HashMap<Point2d<i32>, Tile>) {
    let mut start_pos: Option<Point2d<i32>> = None;
    let mut map = HashMap::new();
    fs::read_to_string("d06/input").unwrap()
        .lines().enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, char)| {
                    let point = Point2d::new(x as i32, y as i32);
                    let tile = match char {
                        '.' => Free,
                        '#' => Blocked,
                        '^' => {
                            start_pos = Some(point.clone());
                            Free
                        },
                        _ => panic!("Unrecognized tile: {:?}", char),
                    };
                    map.insert(point,  tile);
                })
        });

    (start_pos.unwrap(), map)
}