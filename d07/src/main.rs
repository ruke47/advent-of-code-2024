use std::fs;

struct Equation {
    result: u64,
    values: Vec<u64>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let equations = load_equations();
    let sum: u64 = equations.iter()
        .filter(|equation| {
            let possible_results = get_possible_results(&equation.values, false);
            return possible_results.contains(&equation.result)
        })
        .map(|equation| equation.result)
        .sum();

    println!("Part 1: {sum}");
}

fn part2() {
    let equations = load_equations();
    let sum: u64 = equations.iter()
        .filter(|equation| {
            let possible_results = get_possible_results(&equation.values, true);
            return possible_results.contains(&equation.result)
        })
        .map(|equation| equation.result)
        .sum();

    println!("Part 2: {sum}");
}

fn get_possible_results(values: &[u64], allow_concat: bool) -> Vec<u64> {
    let (last, rest) = values.split_last().unwrap();
    if rest.is_empty() {
        vec![*last]
    } else {
        get_possible_results(rest, allow_concat).into_iter()
            .flat_map(|downstream_result| {
                if allow_concat {
                    let mut concatted = downstream_result.to_string();
                    concatted.push_str(&last.to_string());
                    let concatted: u64 = concatted.parse().unwrap();

                    vec![last + downstream_result, last * downstream_result, concatted].into_iter()
                } else {
                    vec![last + downstream_result, last * downstream_result].into_iter()
                }
            })
            .collect()
    }
}

fn load_equations() -> Vec<Equation> {
    fs::read_to_string("d07/input").unwrap()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let result = parts.next().unwrap().parse().unwrap();
            let values = parts.next().unwrap()
                .split(" ")
                .map(|value| value.parse().unwrap())
                .collect();

            return Equation {result, values};
        })
        .collect()
}
