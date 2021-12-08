fn main() {
    let input = include_str!("../input").trim();
    let words = parse_input(input);
    let size = words
        .iter()
        .filter(|&&w| [2, 4, 3, 7].contains(&w.len())).copied()
        .count();

    println!("{}", size);
}

fn parse_input(input: &str) -> Vec<&str> {
    let lines: Vec<&str> = input
        .trim()
        .split('\n')
        .collect();

    lines
        .iter()
        .map(|&line| line.split(" | ").collect::<Vec<&str>>())
        .flat_map(|v| {
            let &line_end = v.get(1).unwrap();

            line_end
                .split(' ')
                .collect::<Vec<&str>>()
        })
        .collect()
}
