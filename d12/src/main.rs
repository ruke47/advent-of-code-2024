use lib2d::Direction::{Down, Left, Right, Up};
use lib2d::{dir_delta, Point2d};
use std::collections::{HashMap, HashSet};
use std::fs;

type Point = Point2d<i32>;

#[derive(Debug)]
struct Region {
    letter: char,
    points: HashSet<Point>,
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = load_map();
    let distinct_regions = find_regions(&map);

    println!("Found {:?} distinct regions:", distinct_regions.len());

    let cost: usize = distinct_regions
        .iter()
        .map(|region| {
            let area = region.points.len();
            let perimeter = find_perimeter(&region.points);

            area * perimeter
        })
        .sum();

    println!("Part 1: {cost}");
}

fn find_regions(map: &HashMap<Point, char>) -> Vec<Region> {
    let mut distinct_regions = vec![];
    let mut unmatched_points = HashSet::new();
    unmatched_points.extend(map.keys());

    while !unmatched_points.is_empty() {
        // grab any point
        let point = unmatched_points.iter().next().unwrap();
        let letter = *map.get(point).unwrap();
        let mut region_points = HashSet::new();
        paint_region(point, &map, &mut region_points);
        region_points.iter().for_each(|region_point| {
            unmatched_points.remove(region_point);
        });

        distinct_regions.push(Region {
            letter,
            points: region_points,
        });
    }

    distinct_regions
}

fn paint_region(point: &Point, map: &HashMap<Point, char>, region: &mut HashSet<Point>) {
    // we've already done this
    if region.contains(point) {
        return;
    }

    // we are always in our own region
    region.insert(point.clone());

    let own_val = map.get(point).unwrap();

    for dir in [Up, Down, Left, Right] {
        let neighbor = *point + dir_delta(dir);
        if let Some(neighbor_val) = map.get(&neighbor) {
            if neighbor_val == own_val {
                paint_region(&neighbor, map, region);
            }
        }
    }
}

fn find_perimeter(points: &HashSet<Point>) -> usize {
    points.iter()
        .map(|point| {
            let mut edges: usize = 0;
            // for each direction, if the region does not contain another point in that direction,
            // that direction counts as an edge
            for dir in [Up, Down, Left, Right] {
                if !points.contains(&(*point + dir_delta(dir))) {
                    edges += 1;
                }
            }
            edges
        })
        .sum()
}

fn part2() {
    let map = load_map();
    let regions = find_regions(&map);

    let cost: usize = regions.iter()
        .map(|region| {
            let size = region.points.len();
            let vertices = count_vertices(&region.points);

            println!("Region {:?}: {size} x {vertices}: {:?}", region.letter, region.points);
            size * vertices
        })
        .sum();

    println!("Part 2: {cost}");
}

fn count_vertices(points: &HashSet<Point>) -> usize {
    let corner_dirs = [(Up, Left), (Up, Right), (Down, Left), (Down, Right)];
    points.iter()
        .map(|point| {
            let mut corners = 0;

            for (d1, d2) in corner_dirs.iter() {
                let neighbor1 = *point + dir_delta(*d1);
                let neighbor2 = *point + dir_delta(*d2);

                // if a point is alone in both an up/down direction and a left/right direction,
                // that means that it's a convex corner in that diagonal
                if !points.contains(&neighbor1) && !points.contains(&neighbor2) {
                    corners += 1;
                }
                
                // if a point has neighbors in both the up/down + left/right direction, but 
                // no neighbor in that diagonal, it's a concave corner ini that diagonal
                let diag = *point + dir_delta(*d1) + dir_delta(*d2);
                if points.contains(&neighbor1) && points.contains(&neighbor2) 
                    && !points.contains(&diag) {
                    corners += 1
                }
            }

            corners
        })
        .sum()
}

fn load_map() -> HashMap<Point, char> {
    let mut map = HashMap::new();

    fs::read_to_string("d12/input")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, char)| {
                map.insert(Point2d::new(x as i32, y as i32), char);
            });
        });

    map
}
