use lib2d::Point2d;
use std::collections::{HashMap, HashSet};
use std::fs;

struct Map {
    max_x: i64,
    max_y: i64,
    antennas: HashMap<char, Vec<Point2d<i64>>>
}

impl Map {
    fn antinode_fits(&self, point: &Point2d<i64>) -> bool {
        point.x >= 0 && point.y >= 0 && point.x <= self.max_x && point.y <= self.max_y
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let map = load_map();
    let mut antinodes: HashSet<Point2d<i64>> = HashSet::new();
    map.antennas.values().for_each(|points| {
        let pairs = pairs(&points);
        for (a, b) in pairs {
            let delta = a - b;
            let lower = a + delta;
            let upper = b - delta;
            for point in [lower, upper] {
                if map.antinode_fits(&point) {
                    antinodes.insert(point);
                }
            }
        }
    });

    println!("Part 1: {:?}", antinodes.len());
}

fn part2() {
    let map = load_map();
    let mut antinodes: HashSet<Point2d<i64>> = HashSet::new();
    map.antennas.values().for_each(|points| {
        let pairs = pairs(&points);
        for (a, b) in pairs {
            let delta = a - b;
            
            let mut lower = a;
            loop {
                if map.antinode_fits(&lower) {
                    antinodes.insert(lower);
                    lower = lower + delta;
                } else {
                    break;
                }
            }
            
            let mut upper = b;
            loop {
                if map.antinode_fits(&upper) {
                    antinodes.insert(upper);
                    upper = upper - delta;
                } else {
                    break;
                }
            }
        }
    });

    println!("Part 2: {:?}", antinodes.len());

}

fn pairs<T: Clone>(list: &[T]) -> Vec<(T, T)> {
    let (first, rest) = list.split_first().unwrap();
    if rest.is_empty() {
        return vec![]
    }
    let mut list: Vec<_> = rest.iter()
        .map(|latter| ((*first).clone(), (*latter).clone()))
        .collect();

    let mut downstream = pairs(rest);
    list.append(&mut downstream);

    return list
}

fn load_map() -> Map {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut antennas = HashMap::new();

    fs::read_to_string("d08/input").unwrap()
        .lines().enumerate()
        .for_each(|(y, line)| {
            max_y = y as i64;
            line.chars().enumerate()
                .for_each(|(x, char)| {
                    if char != '.' {
                        let mut list = antennas.remove(&char).unwrap_or_else(|| vec![]);
                        list.push(Point2d::new(x as i64, y as i64));
                        antennas.insert(char, list);
                    }
                    max_x = x as i64;
                })
        });

    Map {max_x, max_y, antennas}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_works() {
        let singles = ['A', 'B', 'C'];
        let pairs = pairs(&singles);
        let expected = vec![('A', 'B'), ('A', 'C'), ('B', 'C')];
        assert_eq!(pairs, expected);
    }
}