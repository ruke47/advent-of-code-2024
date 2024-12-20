use itertools::Itertools;
use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Direction, Point2d};
use std::cell::RefCell;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

type Coord = i32;
type Point = Point2d<Coord>;

#[derive(Eq, PartialEq, Debug)]
struct SlidyCrate {
    id: usize,
    loc: RefCell<Point>,
    width: Coord,
}

impl Hash for SlidyCrate {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.id)
    }
}


impl SlidyCrate {
    fn gps(&self) -> Coord {
        let point = self.loc.borrow();
        point.x + (point.y * 100)
    }

    fn blocks(&self, point: &Point) -> bool {
        let loc = self.loc.borrow();
        let x_range = loc.x..(loc.x + self.width);

        x_range.contains(&point.x) && loc.y == point.y
    }

    fn points(&self) -> Vec<Point> {
        let loc = self.loc.borrow();
        (0..self.width)
            .map(|dx| Point2d::new(loc.x + dx, loc.y))
            .collect()
    }

    fn shift(&self, direction: &Direction) {
        let destination = *self.loc.borrow() + dir_delta(*direction);
        self.loc.replace(destination);
    }
}

#[derive(Debug)]
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
        let mut impacted_boxes = HashSet::new();
        if self.could_move(bot, &instr, &mut impacted_boxes) {
            impacted_boxes.iter().for_each(|b| {
                b.shift(&instr);
            })
        }
        true
    }

    fn could_move<'a>(&'a self, mover: &'a SlidyCrate, instr: &Direction,
                  impacted_boxes: &mut HashSet<&'a SlidyCrate>) -> bool {
        impacted_boxes.insert(mover);

        let destinations = mover.points().into_iter()
            .map(|point| point + dir_delta(*instr))
            .collect_vec();

        // can't negotiate with a wall
        if destinations.iter().any(|destination| self.walls.contains(destination)) {
            return false;
        }

        let blockers = self.boxes.iter()
            // figure out if some box is in our way
            .filter(|b| {
                destinations.iter().any(|destination| b.blocks(destination))
            })
            // but it's fine if we slide into another point that we contain
            .filter(|b| *b != mover)
            .collect_vec();

        blockers.iter().all(|b| self.could_move(b, instr, impacted_boxes))
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut board = load_board(1);
    loop {
        if !board.tick() {
            break;
        }
    }

    let sum: Coord = board.boxes.iter().map(|b| b.gps()).sum();

    println!("Part 1: {sum}");
}

fn part2() {
    let mut board = load_board(2);
    loop {
        if !board.tick() {
            break;
        }
    }

    let sum: Coord = board.boxes.iter().map(|b| b.gps()).sum();

    println!("Part 2: {sum}");
}

fn load_board(width: Coord) -> Board {
    let mut walls = HashSet::new();
    let mut boxes = Vec::new();
    let mut bot: Option<Point> = None;

    let (map_str, instruction_str) = fs::read_to_string("d15/input")
        .unwrap()
        .split("\n\n")
        .map(|p| p.to_string())
        .collect_tuple()
        .unwrap();
    let mut crate_id = 1;
    map_str.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, char)| {
            let point = Point2d::new(
                width * x as Coord,
                y as Coord
            );
            match char {
                '#' => {
                    for i in 0..width {
                        let offset_point = Point2d::new(i, 0) + point;
                        walls.insert(offset_point);
                    }
                }
                'O' => {
                    boxes.push(SlidyCrate {
                        id: crate_id,
                        loc: RefCell::new(point),
                        width
                    });
                    crate_id += 1;
                },
                '@' => bot = Some(point),
                _ => {}
            }
        })
    });

    let moves = instruction_str
        .chars()
        .flat_map(|dir| match dir {
            '^' => Some(Up),
            'v' => Some(Down),
            '>' => Some(Right),
            '<' => Some(Left),
            _ => None,
        })
        // reverse so we can pop
        .rev()
        .collect();

    Board {
        walls,
        boxes,
        bot: SlidyCrate {
            id: 0,
            loc: RefCell::new(bot.unwrap()),
            width: 1
        },
        instructions: moves,
    }
}
