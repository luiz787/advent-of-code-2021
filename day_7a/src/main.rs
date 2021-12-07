fn main() {
    let input = include_str!("../input").trim();
    let mut positions = parse_input(input);

    positions.sort_unstable();

    let optimal_position = if positions.len() % 2 == 1 {
        let index = positions.len() / 2;
        positions[index]
    } else {
        let index = positions.len() / 2 - 1;
        let index2 = positions.len() / 2;

        (positions[index] + positions[index2]) / 2
    };

    let fuel: i64 = positions
        .iter()
        .map(|v| i64::abs(v - optimal_position))
        .sum();
    
    println!("{}", fuel);
}

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(',')
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect()
}
