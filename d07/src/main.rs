use std::fs;

struct Equation {
    result: i64,
    values: Vec<i64>
}

fn main() {
    part1();
    part2();
}

fn part1() {
    let equations = load_equations();
    let sum: i64 = equations.iter()
        .filter(|equation| {
            let possible_results = get_possible_results(&equation.values);
            // println!("{:?}", possible_results);
            return possible_results.contains(&equation.result)
        })
        .map(|equation| equation.result)
        .sum();

    println!("Part 1: {sum}");
}

fn part2() {

}

fn get_possible_results(values: &[i64]) -> Vec<i64> {
    let (last, rest) = values.split_last().unwrap();
    if rest.is_empty() {
        vec![*last]
    } else {
        get_possible_results(rest).into_iter()
            .flat_map(|downstream_result| {
                [last + downstream_result, last * downstream_result].into_iter()
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
