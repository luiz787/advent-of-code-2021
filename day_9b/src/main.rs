fn main() {
    let input = include_str!("../input");
    let grid = parse_input(input);

    let low_points = compute_low_points(&grid);

    let mut visited: Vec<Vec<bool>> = grid
        .iter()
        .map(|l| l.iter().map(|_| false).collect())
        .collect();

    let mut sizes: Vec<usize> = low_points
        .iter()
        .map(|&low_point| capture_all(&grid, &mut visited, low_point))
        .map(|v| v.len())
        .collect();
    
    sizes.sort_unstable();

    println!("{}", sizes
        .iter()
        .rev()
        .take(3)
        .product::<usize>());
}

fn compute_low_points(grid: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = Vec::new();
    for (line_number, line) in grid.iter().enumerate() {
        if line_number == 0 {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx) && lt_below(pos, grid, line_number, idx) {
                        low_points.push((line_number, idx));
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx) && lt_below(pos, grid, line_number, idx) {
                        low_points.push((line_number, idx));
                    }
                } else if lt_right(pos, line, idx)
                    && lt_left(pos, line, idx)
                    && lt_below(pos, grid, line_number, idx)
                {
                    low_points.push((line_number, idx));
                }
            }
        } else if line_number == grid.len() - 1 {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx) && lt_above(pos, grid, line_number, idx) {
                        low_points.push((line_number, idx));
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx) && lt_above(pos, grid, line_number, idx) {
                        low_points.push((line_number, idx));
                    }
                } else if lt_right(pos, line, idx)
                    && lt_left(pos, line, idx)
                    && lt_above(pos, grid, line_number, idx)
                {
                    low_points.push((line_number, idx));
                }
            }
        } else {
            for (idx, pos) in line.iter().enumerate() {
                if idx == 0 {
                    if lt_right(pos, line, idx)
                        && lt_above(pos, grid, line_number, idx)
                        && lt_below(pos, grid, line_number, idx)
                    {
                        low_points.push((line_number, idx));
                    }
                } else if idx == line.len() - 1 {
                    if lt_left(pos, line, idx)
                        && lt_above(pos, grid, line_number, idx)
                        && lt_below(pos, grid, line_number, idx)
                    {
                        low_points.push((line_number, idx));
                    }
                } else if lt_right(pos, line, idx)
                    && lt_left(pos, line, idx)
                    && lt_above(pos, grid, line_number, idx)
                    && lt_below(pos, grid, line_number, idx)
                {
                    low_points.push((line_number, idx));
                }
            }
        }
    }
    low_points
}

fn capture_all(
    grid: &[Vec<usize>],
    visited: &mut [Vec<bool>],
    (x, y): (usize, usize),
) -> Vec<(usize, usize)> {
    if visited[x][y] {
        return Vec::new();
    }
    visited[x][y] = true;
    let mut res = Vec::new();
    if grid[x][y] == 9 {
        return res;
    }
    res.push((x, y));
    if x == 0 {
        if y == 0 {
            res.append(&mut capture_all(grid, visited, (x + 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y + 1)));
        } else if y == grid[0].len() - 1 {
            res.append(&mut capture_all(grid, visited, (x + 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y - 1)));
        } else {
            res.append(&mut capture_all(grid, visited, (x + 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y + 1)));
            res.append(&mut capture_all(grid, visited, (x, y - 1)));
        }
    } else if x == grid.len() - 1 {
        if y == 0 {
            res.append(&mut capture_all(grid, visited, (x - 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y + 1)));
        } else if y == grid[0].len() - 1 {
            res.append(&mut capture_all(grid, visited, (x - 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y - 1)));
        } else {
            res.append(&mut capture_all(grid, visited, (x - 1, y)));
            res.append(&mut capture_all(grid, visited, (x, y + 1)));
            res.append(&mut capture_all(grid, visited, (x, y - 1)));
        }
    } else if y == 0 {
        res.append(&mut capture_all(grid, visited, (x - 1, y)));
        res.append(&mut capture_all(grid, visited, (x + 1, y)));
        res.append(&mut capture_all(grid, visited, (x, y + 1)));
    } else if y == grid[0].len() - 1 {
        res.append(&mut capture_all(grid, visited, (x - 1, y)));
        res.append(&mut capture_all(grid, visited, (x + 1, y)));
        res.append(&mut capture_all(grid, visited, (x, y - 1)));
    } else {
        res.append(&mut capture_all(grid, visited, (x - 1, y)));
        res.append(&mut capture_all(grid, visited, (x + 1, y)));
        res.append(&mut capture_all(grid, visited, (x, y + 1)));
        res.append(&mut capture_all(grid, visited, (x, y - 1)));
    }

    res
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
