use itertools::Itertools;
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Direction, Point2d};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;

type Coord = i32;
type Point = Point2d<Coord>;

struct SlidyCrate {
    loc: RefCell<Point>,
}

impl SlidyCrate {
    fn gps(&self) -> Coord {
        let point = self.loc.borrow();
        point.x + (point.y * 100)
    }
}

struct Board {
    walls: HashSet<Point>,
    boxes: Vec<SlidyCrate>,
    bot: SlidyCrate,
    instructions: Vec<Direction>,
}

impl Board {
    fn tick(&mut self) -> bool {
        if self.instructions.is_empty() {
            return false;
        }

        let instr = self.instructions.pop().unwrap();
        let bot = &self.bot;
        self.try_move(bot, &instr);
        true
    }

    fn try_move(&self, mover: &SlidyCrate, instr: &Direction) -> bool {
        let destination = *mover.loc.borrow() + dir_delta(*instr);

        // can't negotiate with a wall
        if self.walls.contains(&destination) {
            return false;
        }

        // figure out if some box is in our way
        let blocker = self.boxes.iter()
            .find(|b| *b.loc.borrow() == destination);

        match blocker {
            Some(blocker_crate) => {
                // if there's a blocker...
                if self.try_move(blocker_crate, instr) {
                    // if the blocker can be moved, move into their place
                    mover.loc.replace(destination);
                    true
                } else {
                    // if the blocker cannot be moved, we cannot be moved
                    false
                }
            },
            None => {
                // if there's no blocker, we can move into the free space
                mover.loc.replace(destination);
                true
            }
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut board = load_board();
    loop {
        if !board.tick() {
            break
        }
    }

    let sum: Coord = board.boxes.iter()
        .map(|b| b.gps())
        .sum();

    println!("Part 1: {sum}");
}

fn part2() {}

fn load_board() -> Board {
    let mut walls = HashSet::new();
    let mut boxes = Vec::new();
    let mut bot: Option<Point> = None;

    let (map_str, instruction_str) = fs::read_to_string("d15/input")
        .unwrap()
        .split("\n\n")
        .map(|p| p.to_string())
        .collect_tuple()
        .unwrap();

    map_str.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate()
                .for_each(|(x, char)| {
                    let point = Point2d::new(x as Coord, y as Coord);
                    match char {
                        '#' => { walls.insert(point); }
                        'O' => boxes.push(SlidyCrate {loc: RefCell::new(point) } ),
                        '@' => bot = Some(point),
                        _ => {}
                    }
                })
        });

    let moves = instruction_str.chars()
        .flat_map(|dir| {
            match dir {
                '^' => Some(Up),
                'v' => Some(Down),
                '>' => Some(Right),
                '<' => Some(Left),
                _ => None
            }
        })
        // reverse so we can pop
        .rev()
        .collect();

    Board {
        walls,
        boxes,
        bot: SlidyCrate {loc: RefCell::new(bot.unwrap()) },
        instructions: moves
    }
}