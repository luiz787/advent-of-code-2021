use std::collections::HashMap;

fn main() {
    let input = include_str!("../input").trim();
    let mut map = parse_input(input);
    

    let iterations = 80;

    for _ in 0..iterations {
        let mut new_state = HashMap::new();
        for i in 0..9 {
            new_state.insert(i, 0);
        }

        for i in (1..9).rev() {
            let prev_val = *map.get(&i).unwrap();
            new_state.insert(i - 1, prev_val);
        }

        let zeroes = *map.get(&0).unwrap();
        new_state.insert(6, new_state.get(&6).unwrap() + zeroes);
        new_state.insert(8, new_state.get(&8).unwrap() + zeroes);
        
        map = new_state;
    }

    println!("{}", map.values().sum::<u64>());
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut map: HashMap<u64, u64> = HashMap::new();
    for i in 0..9 {
        map.insert(i, 0);
    }

    let fish: Vec<u64> = input
        .trim()
        .split(',')
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect();

    for f in fish {
        map.insert(f, map.get(&f).unwrap() + 1);
    }
    map
}
