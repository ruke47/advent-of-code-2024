use std::fs;

#[derive(Debug)]
struct Key {
    heights: Vec<i32>,
    id: usize
}

#[derive(Debug)]
struct Lock {
    pins: Vec<i32>,
    id: usize
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let (keys, locks) = get_locks();
    for key in keys.iter() {
        println!("Key: {:?}", key);
    }
    for lock in locks.iter() {
        println!("Lock {:?}", lock);
    }

    let fits: usize = locks.iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| {
                    for i in 0..5 {
                        if key.heights[i] + lock.pins[i] > 5 {
                            return false;
                        }
                    }
                    return true;
                })
                .count()
        })
        .sum();
    
    println!("Part 1: {fits}");
}

fn part2() {

}

fn get_locks() -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];
    fs::read_to_string("d25/input")
        .unwrap()
        .split("\n\n")
        .for_each(|blob| {
            if blob.starts_with(".....") {
                 keys.push(parse_key(blob));
            }
            if blob.starts_with("#####") {
                locks.push(parse_lock(blob));
            }
        });

    (keys, locks)
}

fn parse_key(blob: &str) -> Key {
    let mut heights = vec![None, None, None, None, None];
    for (line_idx, line) in blob.lines().enumerate() {
        let line_height = 6 - line_idx as i32;
        for (char_idx, val) in line.chars().enumerate() {
            if val == '#' {
                if heights[char_idx] == None {
                    heights[char_idx] = Some(line_height);
                }
            }
        }
    }

    let heights = heights.into_iter().map(|op| op.unwrap()).collect();
    let id = parse_id(&heights);
    Key { heights, id }
}

fn parse_lock(blob: &str) -> Lock {
    let mut pins = vec![None, None, None, None, None];
    for (line_idx, line) in blob.lines().enumerate() {
        let line_height = line_idx as i32 - 1;
        for (char_idx, val) in line.chars().enumerate() {
            if val == '.' {
                if pins[char_idx] == None {
                    pins[char_idx] = Some(line_height);
                }
            }
        }
    }

    let pins = pins.into_iter().map(|op| op.unwrap()).collect();
    let id = parse_id(&pins);
    Lock { pins, id }
}

fn parse_id(list: &Vec<i32>) -> usize {
    let mut rev = list.clone();
    rev.reverse();
    rev.iter().enumerate()
        .map(|(i, val)| *val as usize * 10_usize.pow(i as u32))
        .sum()
}