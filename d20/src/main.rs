use itertools::Itertools;
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Direction, Point2d};
use std::collections::{HashMap, HashSet};
use std::fs;

type Msize = i32;
type Point = Point2d<Msize>;

struct Maze {
    start: Point,
    goal: Point,
    tiles: HashSet<Point>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let maze = load_maze();

    // build up a distance of each existing point in the maze from the end
    let tile_scores = get_non_cheaty_scores(&maze);

    let mut cheat_savings: HashMap<(Point, Direction), i32> = HashMap::new();
    for tile in maze.tiles.iter() {
        for dir in [Up, Down, Left, Right] {
            let delta = dir_delta(dir);
            let neighbor = *tile + delta;
            let next = neighbor + delta;

            if !maze.tiles.contains(&neighbor) && maze.tiles.contains(&next) {
                let self_score = tile_scores.get(tile).unwrap();
                let next_score = tile_scores.get(&next).unwrap();
                if self_score > next_score {
                    cheat_savings.insert((*tile, dir), self_score - next_score - 2);
                }
            }
        }
    }

    let at_least_100 = cheat_savings.values()
        .filter(|value| **value >= 100)
        .count();
    println!("Part 1: {at_least_100}");
}

fn get_non_cheaty_scores(maze: &Maze) -> HashMap<Point, i32> {
    let mut maze_map = HashMap::new();
    let mut cur_tile = maze.goal;
    let mut cur_score = 0;
    loop {
        maze_map.insert(cur_tile, cur_score);
        cur_score += 1;

        if cur_tile == maze.start {
            break;
        }

        for dir in [Up, Down, Left, Right] {
            let neighbor = cur_tile + dir_delta(dir);
            if maze.tiles.contains(&neighbor) && !maze_map.contains_key(&neighbor) {
                cur_tile = neighbor;
                break;
            }
        }
    }

    maze_map
}

fn part2() {
    let maze = load_maze();

    // build up a map of all the places you can go within 20 seconds
    let mut cheaty_deltas = get_cheaty_deltas(20);

    // remove the boring places (no movement, definitely inside a wall
    let no_movement = Point2d::new(0, 0);
    cheaty_deltas.remove(&no_movement);
    for dir in [Up, Down, Left, Right] {
        let uninteresting_delta = dir_delta(dir);
        cheaty_deltas.remove(&uninteresting_delta);
    }

    // build up a distance of each existing point in the maze from the end
    let tile_scores = get_non_cheaty_scores(&maze);

    let mut good_cheats = 0;
    for tile in maze.tiles.iter() {
        for (delta, cheat_cost) in cheaty_deltas.iter() {
            let neighbor = *tile + *delta;

            if maze.tiles.contains(&neighbor) {
                let self_score = tile_scores.get(tile).unwrap();
                let neighbor_score = tile_scores.get(&neighbor).unwrap();
                let savings = self_score - neighbor_score - cheat_cost;
                if savings >= 100 {
                    good_cheats += 1;
                }
            }
        }
    }

    println!("Part 2: {good_cheats}");
}

fn get_cheaty_deltas(cheat_seconds: i32) -> HashMap<Point, i32> {
    let mut delta_map = HashMap::new();
    let mut unexplored = vec![(Point2d::new(0,0), 0)];

    loop {
        unexplored.sort_by_key(|(_point, score)| -1 * score);
        let (cur_point, cur_score) = unexplored.pop().unwrap();

        if cur_score > cheat_seconds {
            break;
        }

        if delta_map.keys().contains(&cur_point) {
            continue;
        }
        delta_map.insert(cur_point, cur_score);

        for dir in [Up, Down, Left, Right] {
            let new_point = cur_point + dir_delta(dir);
            unexplored.push((new_point, cur_score + 1))
        }
    }

    delta_map
}

fn load_maze() -> Maze {
    let mut start = None;
    let mut goal = None;
    let mut tiles = HashSet::new();

    fs::read_to_string("d20/input").unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, val)| {
                let point = Point2d::new(x as Msize, y as Msize);
                match val {
                    'S' => {
                        tiles.insert(point);
                        start = Some(point);
                    },
                    'E' => {
                        tiles.insert(point);
                        goal = Some(point);
                    },
                    '.' => {
                        tiles.insert(point);
                    },
                    _ => {}
                }
            });
        });

    Maze {
        start: start.unwrap(),
        goal: goal.unwrap(),
        tiles
    }
}