use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Direction, Point2d};
use std::collections::HashSet;
use std::fs;

type Msize = i32;
type Point = Point2d<Msize>;

struct Maze {
    points: HashSet<Point>,
    start: Point,
    goal: Point
}

impl Maze {
    fn solve(&self) -> ExploredTile {
        let mut explored: HashSet<Pose> = HashSet::new();
        let mut unexplored: Vec<ExploredTile> = vec![];

        let start_pose = Pose { point: self.start, orientation: Right };

        unexplored.push(ExploredTile {
            pose: start_pose,
            score: 0,
            route: HashSet::from([self.start])
        });

        while !unexplored.is_empty() {
            // find the easiest-to-get-to unexplored tile
            unexplored.sort_by_key(|pose| -1 * pose.score as i64);
            let cur_tile = unexplored.pop().unwrap();

            // make sure we haven't already explored it
            if !explored.insert(cur_tile.pose) {
                continue;
            }

            // if it's the goal, we're done
            if cur_tile.pose.point == self.goal {
                return cur_tile;
            }

            // for each of the next moves you could make...
            for neighbor in [cur_tile.turn_left(), cur_tile.turn_right(), cur_tile.go_forward()] {
                // determine if this is a valid tile in the maze
                if !self.points.contains(&neighbor.pose.point) {
                    continue
                }
                // determine if we've already been here
                if explored.contains(&neighbor.pose) {
                    continue
                }
                // add it to our to-do list
                unexplored.push(neighbor);
            }
        }

        panic!("Didn't find the exit!");
    }

    fn solve_all(&self) -> Vec<ExploredTile> {
        let mut explored: HashSet<Pose> = HashSet::new();
        let mut unexplored: Vec<ExploredTile> = vec![];

        let start_pose = Pose { point: self.start, orientation: Right };

        unexplored.push(ExploredTile {
            pose: start_pose,
            score: 0,
            route: HashSet::from([self.start])
        });

        let mut solutions = vec![];
        let mut best_solution = None;

        while !unexplored.is_empty() {
            // find the easiest-to-get-to unexplored tile
            unexplored.sort_by_key(|pose| -1 * pose.score as i64);
            let mut cur_tile = unexplored.pop().unwrap();
            
            // split out all tiles that have the same pose as this tile
            let (sib_tiles, unrelated) = unexplored
                .into_iter()
                .partition(|tile| tile.pose == cur_tile.pose);
            
            // remove all related tiles from the unexplored list 
            unexplored = unrelated;
            
            // for any sibling tiles with the same score as this tile, merge their seen paths
            sib_tiles.into_iter()
                .filter(|sib| sib.score == cur_tile.score)
                .for_each(|equal_sib| {
                    equal_sib.route.into_iter()
                        .for_each(|sib_seen_tile| {
                            cur_tile.route.insert(sib_seen_tile);
                        });
                });

            // make sure we haven't already explored it
            if !explored.insert(cur_tile.pose) {
                continue;
            }

            // if we've found a solution, and this tile has a worse score,
            // we've found all of the solutions with the best score
            if let Some(best_solution) = best_solution {
                if cur_tile.score > best_solution {
                    break;
                }
            }

            // if it's the goal, add it to the pile
            if cur_tile.pose.point == self.goal {
                best_solution = Some(cur_tile.score);
                solutions.push(cur_tile);

                // we're not going to get a better solution by exploring from here,
                // but there might be another route
                continue;
            }

            // for each of the next moves you could make...
            for neighbor in [cur_tile.turn_left(), cur_tile.turn_right(), cur_tile.go_forward()] {
                // determine if this is a valid tile in the maze
                if !self.points.contains(&neighbor.pose.point) {
                    continue
                }
                // determine if we've already been here
                if explored.contains(&neighbor.pose) {
                    continue
                }
                // add it to our to-do list
                unexplored.push(neighbor);
            }
        }

        solutions
    }
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Pose {
    point: Point,
    orientation: Direction,
}

struct ExploredTile {
    pose: Pose,
    score: usize,
    route: HashSet<Point>
}

impl ExploredTile {
    fn turn_left(&self) -> ExploredTile {
        let new_dir = match self.pose.orientation {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
        };

        ExploredTile {
            pose: Pose { point: self.pose.point, orientation: new_dir },
            score: self.score + 1000,
            route: self.route.clone()
        }
    }

    fn turn_right(&self) -> ExploredTile {
        let new_dir = match self.pose.orientation {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        };

        ExploredTile {
            pose: Pose { point: self.pose.point, orientation: new_dir },
            score: self.score + 1000,
            route: self.route.clone()
        }
    }

    fn go_forward(&self) -> ExploredTile {
        let new_point = self.pose.point + dir_delta(self.pose.orientation);
        let mut new_route = self.route.clone();
        new_route.insert(new_point);

        ExploredTile {
            pose: Pose {
                point: new_point,
                orientation: self.pose.orientation
            },
            score: self.score + 1,
            route: new_route
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let maze = load_maze();
    let exit_tile = maze.solve();
    println!("Part 1: {:?}", exit_tile.score);
}



fn part2() {
    let maze = load_maze();
    let exit_tiles = maze.solve_all();
    let good_seats: HashSet<_> = exit_tiles.into_iter()
        .flat_map(|tile| tile.route.into_iter())
        .collect();

    println!("Part 2: {:?}", good_seats.len());
}



fn load_maze() -> Maze {
    let mut points: HashSet<Point> = HashSet::new();
    let mut start = None;
    let mut goal = None;

    fs::read_to_string("d16/input").unwrap()
        .lines().enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, tile)| {
                    let point = Point2d::new(x as Msize, y as Msize);
                    match tile {
                        '.' => {
                            points.insert(point);
                        },
                        'S' => {
                            points.insert(point);
                            start = Some(point);
                        },
                        'E' => {
                            points.insert(point);
                            goal = Some(point);
                        }
                        _ => {}
                    }
                })
        });

    Maze { points, start: start.unwrap(), goal: goal.unwrap() }
}
