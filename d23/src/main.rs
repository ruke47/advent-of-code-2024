use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let pair_list = load_pairs();
    let direct_neighbors = get_neighbors(&pair_list);

    let mut thruples = HashSet::new();
    for (node, neighbors) in direct_neighbors.iter() {
        for neighbor in neighbors.iter() {
            // only look in increasing order to prevent duplicates
            if neighbor < node {
                continue;
            }
            let neighbors2 = direct_neighbors.get(neighbor).unwrap();
            for neighbor2 in neighbors2.iter() {
                // prevent duplicates
                if neighbor2 < neighbor {
                    continue;
                }
                // if my neighbor's neighbor is my neighbor, we're a thruple
                let neighbors3 = direct_neighbors.get(neighbor2).unwrap();
                if neighbors3.contains(node) {
                    thruples.insert((node, neighbor, neighbor2));
                }
            }
        }
    }

    let admin_counts = thruples
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count();
    println!("Part 1: {admin_counts}");
}

fn get_neighbors(pairs: &Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut direct_neighbors = HashMap::new();

    for (a, b) in pairs.iter() {
        let mut vec = direct_neighbors.remove(a).unwrap_or_else(|| vec![]);
        vec.push(b.clone());
        direct_neighbors.insert(a.clone(), vec);

        let mut vec = direct_neighbors.remove(b).unwrap_or_else(|| vec![]);
        vec.push(a.clone());
        direct_neighbors.insert(b.clone(), vec);
    }

    direct_neighbors
}

fn part2() {
    let pair_list = load_pairs();
    let direct_neighbors = get_neighbors(&pair_list);

    let mut complete_networks = vec![];
    for node in direct_neighbors.keys() {
        let cur_members = vec![node.clone()];
        let potential_members = to_set(direct_neighbors.get(node).unwrap());
        find_networks(cur_members, potential_members, &direct_neighbors, &mut complete_networks);
    }

    let longest = complete_networks
        .iter()
        .max_by_key(|network| network.len())
        .unwrap()
        .iter()
        .join(",");

    println!("Part 2: {longest}")
}

fn find_networks(
    current_members: Vec<String>,
    potential_members: HashSet<String>,
    neighbor_map: &HashMap<String, Vec<String>>,
    complete_networks: &mut Vec<Vec<String>>,
) {
    for new_member in potential_members.iter() {
        // only build up networks in ascending order
        // this is going to result in near duplicates, ie [a,b,c] and [b,c] will both show up as complete networks
        // but this will prevent any true duplicates & save a lot of effort
        let before_any = current_members.iter().any(|current| new_member < current);
        if before_any {
            continue;
        }

        let mut new_current_members = current_members.clone();
        new_current_members.push(new_member.clone());

        // find the new member's neighbors
        let neighbors = to_set(neighbor_map.get(new_member).unwrap());

        // the potential members for the newly formed network is the intersection of the old
        // network's potential members and the newest member's neighbors
        // (helpfully, this always removes the newest member, who is never their own neighbor)
        let new_potential_members: HashSet<String> = potential_members.intersection(&neighbors)
            .map(|s| s.clone())
            .collect();

        // if this doesn't leave anyone else we could add, the network can no longer grow
        if new_potential_members.is_empty() {
            new_current_members.sort();
            complete_networks.push(new_current_members);
            if complete_networks.len() % 100 == 0 {
                println!("Found {}", complete_networks.len());
            }
        } else {
            // look recursively
            find_networks(new_current_members, new_potential_members, neighbor_map, complete_networks);
        }
    }
}

fn to_set(list: &Vec<String>) -> HashSet<String> {
    list.iter()
        .map(|s| s.clone())
        .collect()
}

fn load_pairs() -> Vec<(String, String)> {
    fs::read_to_string("d23/input")
        .unwrap()
        .lines()
        .map(|line| {
            let pair: (String, String) = line
                .split("-")
                .map(|s| s.to_string())
                .collect_tuple()
                .unwrap();
            pair
        })
        .collect()
}
