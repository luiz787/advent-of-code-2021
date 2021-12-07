fn main() {
    let input = include_str!("../input").trim();
    let positions = parse_input(input);

    let mut min_fuel = i64::MAX;

    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    for i in min_pos..max_pos + 1 {
        let curr_fuel: i64 = positions
            .iter()
            .map(|v| i64::abs(v - i))
            .map(|v| if v < 2 { v } else { v * (v+1) / 2 })
            .sum();

        min_fuel = min_fuel.min(curr_fuel);
    }
    
    println!("{}", min_fuel);
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect()
}
