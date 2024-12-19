use itertools::Itertools;
use lib2d::Point2d;
use std::collections::HashSet;
use std::{fs, io};

type CoordSize = i32;
type Point = Point2d<CoordSize>;

struct Bot {
    position: Point,
    velocity: Point,
}

impl Bot {
    fn new(position: Point, velocity: Point) -> Bot {
        Bot { position, velocity }
    }

    fn tick(&mut self, grid_size: &Point) {
        self.position = (self.position + self.velocity) % *grid_size;
    }

    fn get_quadrant(&self, grid_size: &Point) -> Option<usize> {
        let half_grid_x = grid_size.x / 2;
        let half_grid_y = grid_size.y / 2;
        if self.position.x == half_grid_x || self.position.y == half_grid_y {
            return None;
        }

        if self.position.y < half_grid_y {
            if self.position.x < half_grid_x {
                Some(1)
            } else {
                Some(2)
            }
        } else {
            if self.position.x < half_grid_x {
                Some(3)
            } else {
                Some(4)
            }
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut bots = load_bots();
    let grid_size = Point2d::new(101, 103);

    let counts = bots.iter_mut()
        .flat_map(|bot| {
            for _ in 0..100 {
                bot.tick(&grid_size);
            }
            let quad = bot.get_quadrant(&grid_size);
            // println!("Bot in {:?} at {:?}", quad, bot.position);
            quad
        })
        .counts_by(|quad| quad);

    let score: usize = counts.values().product();
    println!("Part 1: {score}");
}

fn part2() {
    // I don't know what a christmas tree looks like?
    let mut bots = load_bots();
    let grid_size = &Point2d::new(101, 103);
    
    let mut stdin = io::stdin();
    let input = & mut String::new();
    let mut i = 0;
    
    let mut seen_arrangements = HashSet::new();
    loop {
        i += 1;
        let points: HashSet<Point> = bots.iter_mut()
            .map(|bot| {
                bot.tick(grid_size);
                bot.position
            })
            .collect();
        for y in 0..grid_size.y {
            for x in 0..grid_size.x {
                let char =  if points.contains(&Point2d::new(x, y)) {'*'} else {' '};
                print!("{char}");
            }
            println!();
        }
        println!("\n\n{i}");
        
        let pt_array = points.into_iter().sorted().collect_vec();
        let duplicate = !seen_arrangements.insert(pt_array);
        if duplicate {
            println!("Duplicate!");
            break;
        }
        stdin.read_line(input).unwrap();
    }
}

fn load_bots() -> Vec<Bot> {
    fs::read_to_string("d14/input").unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(" ");
            let pos = parts.next().unwrap().to_string();
            let vel = parts.next().unwrap().to_string();

            Bot::new(parse_point(&pos), parse_point(&vel))
        })
        .collect()
}

fn parse_point(pair_str: &str) -> Point {
    let mut parts = pair_str[2..].split(",");
    let x: CoordSize = parts.next().unwrap().parse().unwrap();
    let y: CoordSize = parts.next().unwrap().parse().unwrap();

    Point2d::new(x, y)
}