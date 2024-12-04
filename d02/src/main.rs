use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = read_input();
    
    let safe_count = lines.into_iter()
        .filter(|line| is_safe(line, 0, false))
        .count();
    
    println!("Part 1: {safe_count}")
}

fn part2() {
    let safe_count = read_input().into_iter()
        .filter(|line| is_safe(line, 1, true))
        .count();
    println!("Part 2: {safe_count}");
}

fn is_safe(line: &[i32], skips: u32, log: bool) -> bool {
    let (first, remaining) = line.split_first().unwrap();
    let increasing = *first < remaining[0];
    let mut prior = first;



    for (idx, current) in remaining.iter().enumerate() {
        let (cur_min, cur_max) = get_bounds(*prior, increasing);
        if *current < cur_min || *current > cur_max {
            if log {
                println!("INVALID[{skips}]: {:?} - {current} @ {idx}", line);
            }
            if skips > 0 {
                // try skipping this one
                let without_this = [&line[..idx+1], &line[(idx+2)..]].concat();
                if is_safe(&without_this, skips - 1, log) {
                   return true;
                }
                // maybe the problem was the prior one instead
                // ie: 1 2 5 3 4 - 5 is valid, but should be removed, and we don't find out until 3
                let without_prior = [&line[..idx], &line[idx+1..]].concat();
                if is_safe(&without_prior, skips - 1, log) {
                    return true;
                }
                // maybe the problem was that the 1st entry  set us on the wrong incr/decr path
                if idx == 1 && is_safe(&line[1..], skips -1, log) {
                    return true;
                }
            }
            return false
        }
        prior = current;
    }

    return true
}

fn get_bounds(prior: i32, increasing: bool) -> (i32, i32) {
    if increasing {
        (prior + 1, prior + 3)
    } else {
        (prior - 3, prior - 1)
    }
}

fn read_input() -> Vec<Vec<i32>> {
    fs::read_to_string("d02/input")
        .unwrap().lines()
        .map(|line| {
            line.split(" ")
                .map(|i| i.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_out_of_order_incr() {
        let arr = [2, 1, 2, 3, 4];
        assert_eq!(is_safe(&arr, 1, true), true);
    }

    #[test]
    fn first_out_of_order_decr() {
        let arr = [8, 9, 8, 7, 6];
        assert_eq!(is_safe(&arr, 1, true), true);
    }

    #[test]
    fn mid_out_of_order_incr() {
        let arr = [1, 2, 3, 5, 4, 5];
        assert_eq!(is_safe(&arr, 1, true), true);
    }

    #[test]
    fn mid_out_of_order_decr() {
        let arr = [9, 8, 10, 7, 6];
        assert_eq!(is_safe(&arr, 1, true), true);
    }

    #[test]
    fn last_out_of_order_incr() {
        let arr = [1, 2, 3, 4, 1];
        assert_eq!(is_safe(&arr, 1, true), true);
    }

    #[test]
    fn last_out_of_order_decr() {
        let arr = [9, 8, 7, 6, 9];
        assert_eq!(is_safe(&arr, 1, true), true);
    }
}