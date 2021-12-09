fn main() {
    let input = include_str!("../input");
    let grid = parse_input(input);

    let mut risk_levels_sum = 0;

    for (line_number, line) in grid.iter().enumerate() {
        if line_number == 0 {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx) && lt_below(pos, &grid, line_number, idx) {
                        risk_levels_sum += pos + 1;
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx) && lt_below(pos, &grid, line_number, idx) {
                        risk_levels_sum += pos + 1;
                    }
                } else if lt_right(pos, line, idx) && lt_left(pos, line, idx) && lt_below(pos, &grid, line_number, idx) {
                    risk_levels_sum += pos + 1;
                }
            }
        } else if line_number == grid.len() - 1 {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx) && lt_above(pos, &grid, line_number, idx) {
                        risk_levels_sum += pos + 1;
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx) && lt_above(pos, &grid, line_number, idx) {
                        risk_levels_sum += pos + 1;
                    }
                } else if lt_right(pos, line, idx) && lt_left(pos, line, idx) && lt_above(pos, &grid, line_number, idx) {
                    risk_levels_sum += pos + 1;
                }
            }
        } else {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx) && lt_above(pos, &grid, line_number, idx) && lt_below(pos, &grid, line_number, idx){
                        risk_levels_sum += pos + 1;
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx) && lt_above(pos, &grid, line_number, idx) && lt_below(pos, &grid, line_number, idx){
                        risk_levels_sum += pos + 1;
                    }
                } else if lt_right(pos, line, idx) && lt_left(pos, line, idx) && lt_above(pos, &grid, line_number, idx) && lt_below(pos, &grid, line_number, idx) {
                    risk_levels_sum += pos + 1;
                }
            }
        }

    }

    println!("{}", risk_levels_sum);
}

fn lt_below(pos: &usize, grid: &[Vec<usize>], line_number: usize, idx: usize) -> bool {
    pos < &grid[line_number + 1][idx]
}

fn lt_above(pos: &usize, grid: &[Vec<usize>], line_number: usize, idx: usize) -> bool {
    pos < &grid[line_number - 1][idx]
}

fn lt_left(pos: &usize, line: &[usize], idx: usize) -> bool {
    pos < &line[idx - 1]
}

fn lt_right(pos: &usize, line: &[usize], idx: usize) -> bool {
    pos < &line[idx + 1]
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| (c as usize) - 48).collect())
        .collect()
}