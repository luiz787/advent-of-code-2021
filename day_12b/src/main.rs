use std::{collections::{HashMap, VecDeque}};

fn main() {
    let input = include_str!("../input");
    let map = parse_input(input);

    let mut path = VecDeque::new();
    path.push_front("start");
    
    println!("{}", count_paths(&map, "start", "end"));
}

fn count_paths(map: &HashMap<&str, Vec<&str>>, from: &str, to: &str) -> u64 {
    let mut visited = VecDeque::new();
    visited.push_front(from);
    
    dfs_count(map, from, from, to, &visited, false)
}

fn dfs_count(map: &HashMap<&str, Vec<&str>>, current: &str, from: &str, to: &str, visited: &VecDeque<&str>, used_double_visit: bool) -> u64 {
    if current == to {
        return 1
    }

    let mut sum = 0;
    for &neighbor in map.get(current).unwrap() {
        if neighbor.to_lowercase() != neighbor || !visited.contains(&neighbor) {
            let mut neighbor_visited = visited.clone();
            neighbor_visited.push_front(neighbor);
            sum += dfs_count(map, neighbor, from, to, &neighbor_visited, used_double_visit);
        } else if !used_double_visit && ![from, to].contains(&neighbor) {
            let mut neighbor_visited = visited.clone();
            neighbor_visited.push_front(neighbor);
            sum += dfs_count(map, neighbor, from, to, &neighbor_visited, true);
        }
    }

    sum
}

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    let lines = input
        .split('\n')
        .map(str::trim)
        .filter(|&l| !l.is_empty());

    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in lines {
        let (first, second) = line.split_once('-').unwrap();

        map.entry(first).or_default().push(second);
        map.entry(second).or_default().push(first);
    }

    map
}
