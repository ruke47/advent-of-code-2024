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
        .map(|point| pathfind(&map, point, &vec![]))
        .map(|list_of_paths| {
            let distinct_ends: HashSet<Coord> = list_of_paths.iter()
                .flat_map(|path| path.last().map(|c| c.clone()))
                .collect();

            distinct_ends
        })
        .map(|points| points.len())
        .sum();

    println!("Part 1: {score}");
}

fn part2() {
    let map = read_map();
    let trailheads: Vec<&Coord> = map.iter()
        .filter(|(_p, &h)| h == 0)
        .map(|(p, _h)| p)
        .collect();

    let score: usize = trailheads.iter()
        .map(|point| pathfind(&map, point, &vec![]))
        .map(|paths| paths.len())
        .sum();

    println!("Part 2: {score}");
}

fn pathfind(map: &HashMap<Coord, Height>,
            from_point: &Coord,
            path: &Vec<Coord>) -> Vec<Vec<Coord>> {
    let self_height = map.get(from_point).unwrap();

    let mut own_path = path.clone();
    own_path.push(from_point.clone());

    if *self_height == 9 {
        return vec![own_path];
    }

    let mut child_paths = vec![];
    for dir in [Up, Down, Left, Right] {
        let neighbor_point = *from_point + dir_delta(dir);
        if let Some(&neighbor_height) = map.get(&neighbor_point) {
            if neighbor_height == self_height + 1 {
                let downstream = pathfind(map, &neighbor_point, &own_path);
                child_paths.extend(downstream);
            }
        }
    }

    child_paths
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