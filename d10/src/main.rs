use lib2d::Direction::{Down, Left, Right};
use lib2d::{dir_delta, Direction, Point2d};
use std::collections::{HashMap, HashSet};
use std::fs;
use Direction::Up;

type Coord = Point2d<i32>;
type Height = usize;

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = read_map();
    let trailheads: Vec<&Coord> = map.iter()
        .filter(|(_p, &h)| h == 0)
        .map(|(p, _h)| p)
        .collect();

    let score: usize = trailheads.iter()
        .map(|point| pathfind(&map, point))
        .map(|points| points.len())
        .sum();

    println!("Part 1: {score}");
}

fn pathfind(map: &HashMap<Coord, Height>, from_point: &Coord) -> HashSet<Coord> {
    let self_height = map.get(from_point).unwrap();
    let mut peaks = HashSet::new();

    if *self_height == 9 {
        peaks.insert(from_point.clone());
        return peaks;
    }

    for dir in [Up, Down, Left, Right] {
        let neighbor_point = *from_point + dir_delta(dir);
        if let Some(&neighbor_height) = map.get(&neighbor_point) {
            if neighbor_height == self_height + 1 {
                let downstream = pathfind(map, &neighbor_point);
                peaks.extend(downstream);
            }
        }
    }

    peaks
}

fn part2() {

}

fn read_map() -> HashMap<Coord, Height> {
    let mut map = HashMap::new();

    fs::read_to_string("d10/input").unwrap()
        .lines().enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, d)| {
                    let height = d.to_string().parse().unwrap();
                    map.insert(Point2d::new(x as i32, y as i32), height);
                });
        });

    map
}