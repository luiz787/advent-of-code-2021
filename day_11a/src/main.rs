use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input");
    let mut grid = parse_input(input);

    let mut total_flashes = 0;
    for _ in 0..100 {
        total_flashes += step(&mut grid);
    }

    println!("{}", total_flashes);
}

fn step(grid: &mut [Vec<u16>]) -> u32 {
    let mut flashes_count = 0;
    for line in &mut *grid {
        for item in line {
            *item += 1;
        }
    }

    let mut flashing = VecDeque::new();
    for (x, line) in grid.iter().enumerate() {
        for (y, &v) in line.iter().enumerate() {
            if v > 9 {
                flashing.push_back((x, y));
            }
        }
    }

    while !flashing.is_empty() {
        let (x, y) = flashing.pop_front().unwrap();
        flashes_count += 1;
        grid[x][y] = 0;
        let neighbors = neighbors(grid, x, y);
        for ((nx, ny), nv) in neighbors {
            if nv != 0 {
                grid[nx][ny] += 1;
            }
            if grid[nx][ny] > 9 && !flashing.contains(&(nx, ny)) {
                flashing.push_back((nx, ny));
            }
        }
    }

    flashes_count
}

fn parse_input(input: &str) -> Vec<Vec<u16>> {
    input
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u16).collect())
        .collect()
}

fn neighbors(grid: &[Vec<u16>], i: usize, j: usize) -> Vec<((usize, usize), u16)> {
    let i = i as i32;
    let j = j as i32;
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1)
    ];

    deltas
        .iter()
        .map(|&(dx, dy)| (i + dx, j + dy))
        .filter(|&(nx, ny)| (0..10).contains(&nx) && (0..10).contains(&ny))
        .map(|(nx, ny)| ((nx as usize, ny as usize), grid[nx as usize][ny as usize]))
        .collect()
}
