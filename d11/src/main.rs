use std::collections::HashMap;
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut stones = load_stones();
    for _ in 0..25 {
        let mut new_stones = vec![];
        for stone in stones.iter() {
            let stone_str = stone.to_string();
            if stone == &0 {
                new_stones.push(1);
            } else if stone_str.len() % 2 == 0 {
                let half = stone_str.len()/2;
                new_stones.push(stone_str[..half].parse().unwrap());
                new_stones.push(stone_str[half..].parse().unwrap());
            } else {
                new_stones.push(stone * 2024)
            }
        }
        stones = new_stones;
    }

    println!("Part 1: {:?}", stones.len());
}

type NumberTicks = (u64, usize);
fn part2() {
    let stones = load_stones();
    let mut memo: HashMap<NumberTicks, usize> = HashMap::new();
    
    let size: usize = stones.iter()
        .map(|stone| calculate_size(*stone, 75, &mut memo))
        .sum();
    
    println!("Part 2: {size}")
}

fn calculate_size(stone_value: u64, number_ticks: usize, memo: &mut HashMap<NumberTicks, usize>) -> usize {
    // no ticks left means this stone isn't splitting
    if number_ticks == 0 {
        return 1;
    }
    
    // if we have already calculated how large this stone will be after N ticks, use that
    if let Some(known) = memo.get(&(stone_value, number_ticks)) {
        return *known;
    }
    
    // if this stone is a '0' 
    if stone_value == 0 {
        // calculate how big a '1' would be with 1 less tick
        let size = calculate_size(1, number_ticks - 1, memo);
        
        // memorize and return this value
        memo.insert((stone_value, number_ticks), size);
        return size;
    }
    
    // if this stone has an even number of digits
    let stone_str = stone_value.to_string();
    if stone_str.len() % 2 == 0 {
        // split it in half
        let half = stone_str.len() / 2;
        let child1 = stone_str[..half].parse().unwrap();
        let child2 = stone_str[half..].parse().unwrap();
        
        // figure out how large each child would be with 1 fewer tick
        let size1 = calculate_size(child1, number_ticks - 1, memo);
        let size2 = calculate_size(child2, number_ticks - 1, memo);
        
        // sum the 2 children sizes, and memorize it
        let sum = size1 + size2;
        memo.insert((stone_value, number_ticks), sum);
        
        return sum;
    }
    
    // otherwise, etc
    let size = calculate_size(stone_value * 2024, number_ticks - 1, memo);
    memo.insert((stone_value, number_ticks), size);
    
    size
}

fn load_stones() -> Vec<u64> {
    fs::read_to_string("d11/input").unwrap()
        .split(" ")
        .map(|n| n.parse().unwrap())
        .collect()
}