use itertools::Itertools;
use std::fs;

type RegSize = usize;

#[derive(Debug, Clone)]
struct MicroProcessor {
    pc: usize,
    reg_a: RegSize,
    reg_b: RegSize,
    reg_c: RegSize,
    memory: Vec<RegSize>,
    output: Vec<RegSize>,
}

impl MicroProcessor {
    fn new(reg_a: RegSize, reg_b: RegSize, reg_c: RegSize, memory: Vec<RegSize>) -> MicroProcessor {
        MicroProcessor {
            reg_a,
            reg_b,
            reg_c,
            memory,
            output: vec![],
            pc: 0
        }
    }

    fn run(&mut self, debug: bool) {
        if debug {
            println!("{:?}", self);
        }
        while self.tick(debug) {
            if debug {
                println!("{:?}", self);
            }
        }
    }

    fn tick(&mut self, debug: bool) -> bool {
        // if we can't pull 2 values out of memory, halt
        if self.memory.len() <= self.pc {
            return false;
        }

        // load operation from memory
        let opcode = self.memory[self.pc];
        let literal_operand = self.memory[self.pc + 1];
        let combo_operand = self.decode_operand(literal_operand);
        self.pc += 2;

        if debug {
            let opcode_name = Self::opcode_name(opcode);
            println!("Got {opcode_name}[{opcode}] L: {literal_operand} C:{combo_operand}");
        }

        match opcode {
            0 => self.adv(combo_operand),
            1 => self.bxl(literal_operand),
            2 => self.bst(combo_operand),
            3 => self.jnz(literal_operand),
            4 => self.bxc(combo_operand),
            5 => self.out(combo_operand),
            6 => self.bdv(combo_operand),
            7 => self.cdv(combo_operand),
            _ => panic!("Got opcode {opcode} with state: {:?}", self),
        }

        true
    }

    fn opcode_name(opcode: RegSize) -> String {
        match opcode {
            0 => String::from("adv"),
            1 => String::from("bxl"),
            2 => String::from("bst"),
            3 => String::from("jnz"),
            4 => String::from("bxc"),
            5 => String::from("out"),
            6 => String::from("bdv"),
            7 => String::from("cdv"),
            _ => String::from("ERR"),
        }
    }

    fn adv(&mut self, operand: RegSize) {
        self.reg_a = self.dv(operand);
    }

    fn bxl(&mut self, operand: RegSize) {
        self.reg_b = self.reg_b ^ operand;
    }

    fn bst(&mut self, operand: RegSize) {
        self.reg_b = operand % 8;
    }

    fn jnz(&mut self, operand: RegSize) {
        if self.reg_a != 0 {
            self.pc = operand;
        }
    }

    fn bxc(&mut self, _operand: RegSize) {
        self.reg_b = self.reg_b ^ self.reg_c;
    }

    fn out(&mut self, operand: RegSize) {
        let value = operand % 8;
        self.output.push(value);
    }

    fn bdv(&mut self, operand: RegSize) {
        self.reg_b = self.dv(operand);
    }

    fn cdv(&mut self, operand: RegSize) {
        self.reg_c = self.dv(operand);
    }

    fn dv(&self, operand: RegSize) -> RegSize {
        let denominator = (2 as RegSize).pow(operand as u32);

        self.reg_a / denominator
    }

    fn decode_operand(&self, operand: RegSize) -> RegSize {
        match operand {
            0..4 => operand,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Got the forbidden combo operand 7: {:?}", self),
            _ => panic!("Got fully illegal operand {operand}: {:?}", self),
        }
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut compy = load_machine();
    compy.run(false);
    let output = compy.output.iter().join(",");
    println!("Part 1: {output}");
}

fn part2() {
    let base_compy = load_machine();

    let mut candidate_values = vec![0];
    // the last output only cares about a % 8
    // the second to only cares about a % (8^2), etc
    for back_digits in 1..=base_compy.memory.len() {
        // take the last N digits as our target output
        let target_output = base_compy.memory.iter()
            .cloned()
            .rev()
            .take(back_digits)
            .rev()
            .collect_vec();

        let mut new_candidates = vec![];

        // for each value that would produce the correct (n-1) tail digits,
        // multiply that value by 8, try adding 0-7,
        // and see which of _those_ values work for (n) tails digits
        for old_candidate in candidate_values {
            for i in 0..8 {
                let new_candidate = 8 * old_candidate + i;
                let mut clone_compy = base_compy.clone();
                clone_compy.reg_a = new_candidate;
                clone_compy.run(false);
                if clone_compy.output == target_output {
                    new_candidates.push(new_candidate);
                }
            }
        }
        candidate_values = new_candidates;
    }

    let min_a = candidate_values.iter().min().unwrap();
    println!("Part 2: {min_a}");
}

fn load_machine() -> MicroProcessor {
    let text = fs::read_to_string("d17/input").unwrap();
    let (reg_a, reg_b, reg_c, _, program) = text
        .lines()
        .collect_tuple()
        .unwrap();

    let (_, reg_a) = reg_a.split(": ").collect_tuple().unwrap();
    let (_, reg_b) = reg_b.split(": ").collect_tuple().unwrap();
    let (_, reg_c) = reg_c.split(": ").collect_tuple().unwrap();
    let (_, program) = program.split(": ").collect_tuple().unwrap();

    let reg_a = reg_a.parse().unwrap();
    let reg_b = reg_b.parse().unwrap();
    let reg_c = reg_c.parse().unwrap();
    let program = program.split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    MicroProcessor::new(reg_a, reg_b, reg_c, program)
}
