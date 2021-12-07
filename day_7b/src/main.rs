fn main() {
    let input = include_str!("../input").trim();
    let positions = parse_input(input);

    let optimal_position = positions
        .iter()
        .sum::<i64>() as f64 / positions.len() as f64;
        
    if optimal_position.fract() == 0.0 {
        let fuel = calculate_fuel_cost(&positions, optimal_position as i64);

        println!("{}", fuel);
    } else {
        let fuel_floor = calculate_fuel_cost(&positions, optimal_position.floor() as i64);
        let fuel_ceiling = calculate_fuel_cost(&positions, optimal_position.ceil() as i64);
        
        println!("{}", fuel_ceiling.min(fuel_floor));
    }
}

fn calculate_fuel_cost(positions: &[i64], target: i64) -> i64 {
    positions
        .iter()
        .map(|v| i64::abs(v - target))
        .map(|v| if v < 2 { v } else { v * (v+1) / 2 })
        .sum()
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect()
}
