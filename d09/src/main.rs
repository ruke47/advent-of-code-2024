use crate::BlockType::FILE;
use std::fs;
use BlockType::FREE;

#[derive(PartialEq, Clone, Copy)]
enum BlockType {
    FILE,
    FREE,
}

struct FreeSpace {
    drive_index: usize,
    block_size: usize,
}

struct FileBlock {
    drive_index: usize,
    block_size: usize,
    block_id: usize,
}

struct Drive {
    frees: Vec<FreeSpace>,
    files: Vec<FileBlock>,
    drive_ptr: usize,
}

impl Drive {
    fn new() -> Self {
        Self {
            frees: vec![],
            files: vec![],
            drive_ptr: 0,
        }
    }

    fn insert_file(&mut self, block_size: usize) {
        let block_id = self.files.len();
        self.files.push(FileBlock {
            drive_index: self.drive_ptr,
            block_size,
            block_id,
        });
        self.drive_ptr += block_size;
    }

    fn insert_space(&mut self, block_size: usize) {
        self.frees.push(FreeSpace {
            drive_index: self.drive_ptr,
            block_size,
        });
        self.drive_ptr += block_size;
    }

    fn defrag(&mut self) {
        // for each file, starting at the back
        self.files.iter_mut().rev().for_each(|file| {
            // find the first space (starting at the front) that is as big as the file
            let first_space = self
                .frees
                .iter_mut()
                .find(|space| {
                    space.block_size >= file.block_size && space.drive_index < file.drive_index
                });
            if let Some(space) = first_space {
                // if we found a space big enough, move the file to where the space is
                file.drive_index = space.drive_index;
                // shrink the space by the size of the file
                space.block_size -= file.block_size;
                // and move the space forwards by the size of the file
                // (this may overlap with the next file if the space's size is 0)
                space.drive_index += file.block_size;
            }
        });

        // sort the files array based on their position within the drive
        self.files.sort_by_key(|file| file.drive_index);

        // spaces remained sorted during the defrag operation
    }

    fn checksum(&self) -> usize {
        self.files
            .iter()
            .map(|file| {
                let mut sum = 0;
                let this_file_end = file.drive_index + file.block_size;
                for i in file.drive_index..this_file_end {
                    sum += file.block_id * i;
                }
                sum
            })
            .sum()
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut array = get_array();
    let mut front_pointer: usize = 0;
    while front_pointer < array.len() {
        if array[front_pointer] > -1 {
            front_pointer += 1;
        } else {
            loop {
                let back_value = array.pop().unwrap();
                if back_value > -1 {
                    array[front_pointer] = back_value;
                    break;
                }
                if front_pointer >= array.len() {
                    break;
                }
            }
        }
    }

    let score: i64 = array.iter().enumerate().map(|(i, v)| i as i64 * v).sum();
    println!("Part 1: {score}");
}

fn part2() {
    let mut drive = get_drive();
    drive.defrag();
    let checksum = drive.checksum();

    println!("Part 2: {checksum}");
}

fn get_array() -> Vec<i64> {
    let mut cur_type = FILE;
    let mut cur_id = 0;
    let mut output = vec![];

    fs::read_to_string("d09/input")
        .unwrap()
        .chars()
        .for_each(|d| {
            let size = d.to_string().parse().unwrap();
            let block_id = match cur_type {
                FREE => -1,
                FILE => cur_id,
            };
            for _ in 0..size {
                output.push(block_id);
            }
            match cur_type {
                FREE => cur_type = FILE,
                FILE => {
                    cur_id += 1;
                    cur_type = FREE
                }
            }
        });

    output
}

fn get_drive() -> Drive {
    let mut drive = Drive::new();
    let mut cur_type = FILE;

    fs::read_to_string("d09/input")
        .unwrap()
        .chars()
        .for_each(|d| {
            let size: usize = d.to_string().parse().unwrap();
            match cur_type {
                FILE => {
                    drive.insert_file(size);
                    cur_type = FREE;
                }
                FREE => {
                    drive.insert_space(size);
                    cur_type = FILE
                }
            };
        });

    drive
}
