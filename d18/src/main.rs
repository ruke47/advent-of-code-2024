use itertools::Itertools;
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Point2d};
use std::collections::HashSet;
use std::fs;

type Bsize = i32;
type Point = Point2d<Bsize>;

#[derive(Eq, PartialEq, Clone, Hash)]
struct ExploredTile {
    point: Point,
    route: Vec<Point>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let arena_size = 71;
    let bad_point_len = 1024;

    let mut maze = HashSet::new();
    for y in 0..arena_size {
        for x in 0..arena_size {
            maze.insert(Point2d::new(x, y));
        }
    }
    let goal = Point2d::new(arena_size - 1, arena_size - 1);

    load_bytes()
        .iter()
        .take(bad_point_len)
        .for_each(|point| {
            maze.remove(point);
        });
    if let Some(score) = run_maze(goal, &maze) {
        println!("Part 1: {}", score.route.len());
    } else {
        println!("I got lost");
    }
}

fn part2() {
    let arena_size = 71;

    let mut maze = HashSet::new();
    for y in 0..arena_size {
        for x in 0..arena_size {
            maze.insert(Point2d::new(x, y));
        }
    }
    let goal = Point2d::new(arena_size - 1, arena_size - 1);

    let mut prior_route: Option<HashSet<Point>> = None;
    for bad_byte in load_bytes() {
        maze.remove(&bad_byte);
        // if there's a prior route
        if let Some(ref prior_route) = prior_route {
            // and this byte did not impact the route
            if !prior_route.contains(&bad_byte) {
                // keep on deleting points without re-solving the maze
                continue;
            }
        }

        if let Some(tile) = run_maze(goal, &maze) {
            // update the route to be the points used to solve this maze
            prior_route = Some(HashSet::from_iter(tile.route.into_iter()));
        } else {
            // there's no route out, we've solved it.
            println!("Part 2: {},{}", bad_byte.x, bad_byte.y);
            break;
        }
    }
}

fn run_maze(goal: Point, maze: &HashSet<Point>) -> Option<ExploredTile> {
    let mut explored = HashSet::new();
    let mut unexplored: Vec<ExploredTile> = vec![ExploredTile {
        point: Point2d::new(0, 0),
        route: vec![],
    }];
    while !unexplored.is_empty() {
        unexplored.sort_by_key(|et| -1 * et.route.len() as i32);
        let cur_tile = unexplored.pop().unwrap();

        // we've already been here
        if !explored.insert(cur_tile.point) {
            continue;
        }

        if cur_tile.point == goal {
            return Some(cur_tile);
        }

        for dir in [Up, Down, Left, Right] {
            let neighbor = cur_tile.point + dir_delta(dir);
            if explored.contains(&neighbor) {
                continue;
            }
            if !maze.contains(&neighbor) {
                continue;
            }
            let mut new_route = cur_tile.route.clone();
            new_route.push(neighbor);
            unexplored.push(ExploredTile {
                point: neighbor,
                route: new_route
            });
        }
    }
    None
}

fn load_bytes() -> Vec<Point> {
    fs::read_to_string("d18/input").unwrap()
        .lines()
        .map(|line| {
            let (x, y) = line.split(",")
                .map(|n| n.parse().unwrap())
                .collect_tuple()
                .unwrap();
            Point2d::new(x, y)
        })
        .collect_vec()

}