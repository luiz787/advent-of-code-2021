fn main() {
    let input = include_str!("../input");

    let (dots, commands) = parse_input(input);

    let (rows, cols) = get_grid_size(&dots);

    let mut grid = create_grid(rows, cols);

    for (x,y) in dots {
        grid[y][x] = '#';
    }

    for &cmd in commands.iter() {
        let cmd = cmd.replace("fold along ", "");
        let (desc, pos) = cmd.split_once('=').unwrap();
        let pos: usize = pos.parse().unwrap();

        if desc == "y" {
            let up_lines: Vec<usize> = (0..pos).collect();
            let down_lines: Vec<usize> = (pos+1..rows).collect();

            if up_lines.len() >= down_lines.len() {
                let zip_pairs: Vec<_> = up_lines
                    .iter()
                    .take(down_lines.len())
                    .zip(down_lines.iter().rev())
                    .collect();

                for (original_line, merge_line) in zip_pairs {
                    for (pos, val) in grid[*merge_line].clone().iter().enumerate() {
                        if *val == '#' {
                            grid[*original_line][pos] = '#';
                        }
                    }
                }

                grid.resize(up_lines.len(), Vec::new());
            } else {
                let zip_pairs: Vec<_> = up_lines
                    .iter()
                    .zip(down_lines.iter().take(up_lines.len()).rev())
                    .collect();

                for (original_line, merge_line) in zip_pairs {
                    for (pos, val) in grid[*merge_line].clone().iter().enumerate() {
                        if *val == '#' {
                            grid[*original_line][pos] = '#';
                        }
                    }
                }

                grid.resize(up_lines.len(), Vec::new());
            }
        } else {
            let left_lines: Vec<usize> = (0..pos).collect();
            let right_lines: Vec<usize> = (pos+1..cols).collect();

            if left_lines.len() >= right_lines.len() {
                let zip_pairs: Vec<_> = left_lines
                    .iter()
                    .take(right_lines.len())
                    .zip(right_lines.iter().rev())
                    .collect();

                for (orig_col, merge_col) in zip_pairs {
                    for line in &mut grid {
                        if line[*merge_col] == '#' {
                            line[*orig_col] = '#';
                        }
                    }
                }

                for line in &mut grid {
                    line.resize(left_lines.len(), '.');
                }
            } else {
                let zip_pairs: Vec<_> = left_lines
                    .iter()
                    .zip(right_lines.iter().take(left_lines.len()).rev())
                    .collect();

                for (orig_col, merge_col) in zip_pairs {
                    for line in &mut grid {
                        if line[*merge_col] == '#' {
                            line[*orig_col] = '#';
                        }
                    }
                }

                for line in &mut grid {
                    line.resize(left_lines.len(), '.');
                }
            }
        }
    }
    
    for line in &grid {
        println!("{:?}", line.iter().collect::<String>());
    }
}

fn create_grid(rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::with_capacity(rows);
    for _ in 0..rows {
        let row: Vec<char> = vec!['.'; cols];
        grid.push(row);
    }
    grid
}

fn get_grid_size(dots: &[(usize, usize)]) -> (usize, usize) {
    let max_x = dots.iter().map(|&tuple| tuple.0).max().unwrap();
    let max_y = dots.iter().map(|&tuple| tuple.1).max().unwrap();

    (max_y + 1, max_x + 1)
}

fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<&str>) {
    let (dots, commands) = input.split_once("\n\n").unwrap();

    let dots: Vec<(usize, usize)> = dots
        .split('\n')
        .map(|line| {
            let (x, y) = line.trim().split_once(',').unwrap();

            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    
    let commands = commands.trim().split('\n').map(str::trim).collect();

    (dots, commands)
}
