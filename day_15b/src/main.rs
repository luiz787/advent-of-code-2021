use std::{collections::BinaryHeap, cmp::Ordering};



#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = include_str!("../input");
    let mut grid = parse_input(input);

    let original_rows = grid.len();
    let original_cols = grid[0].len();

    for i in 0..4 {
        for line in &mut grid {
            let mut next_chunk: Vec<u8> = line[original_cols * i..original_cols * (i+1)]
                .iter()
                .map(|&elem| map_next(elem))
                .collect();
            line.append(&mut next_chunk);
        }
    }

    for _ in 0..4 {
        let lines = &grid[grid.len() - original_rows..grid.len()];

        let mut new_lines: Vec<Vec<u8>> = lines
            .iter()
            .map(|line| line.iter().map(|&elem| map_next(elem)).collect())
            .collect();
        
        grid.append(&mut new_lines);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut adj = vec![Vec::new(); rows * cols];

    for (i, line) in grid.iter().enumerate() {
        for (j, _item) in line.iter().enumerate() {
            let neighbors: Vec<_> = neighbors(&grid, i, j)
                .iter()
                .map(|&(nx, ny)| to_pos(cols, nx, ny))
                .collect();
            adj[to_pos(cols, i, j)] = neighbors;
        }
    }

    println!(
        "{}",
        shortest_path(&grid, cols, &adj, 0, adj.len() - 1).unwrap()
    );
}

fn map_next(val: u8) -> u8 {
    match val {
        9 => 1,
        v => v + 1
    }
}

fn shortest_path(
    grid: &[Vec<u8>],
    cols: usize,
    adj_list: &[Vec<usize>],
    start: usize,
    end: usize,
) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();
    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if position == end {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for neighbor in &adj_list[position] {
            let (i, j) = to_coord(cols, *neighbor);
            let neighbor_cost = grid[i][j] as usize;
            let next = State { cost: cost + neighbor_cost, position: *neighbor };
            if next.cost < dist[*neighbor] {
                heap.push(next);
                dist[*neighbor] = next.cost;
            }
        }
    }

    None
}

fn to_pos(cols: usize, i: usize, j: usize) -> usize {
    i * cols + j
}

fn to_coord(cols: usize, coord: usize) -> (usize, usize) {
    (coord / cols, coord % cols)
}

fn neighbors(grid: &[Vec<u8>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let max_y = (grid.len() - 1) as i64;
    let max_x = (grid[0].len() - 1) as i64;
    let deltas: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    deltas
        .iter()
        .map(|&(dx, dy)| (i as i64 + dx, j as i64 + dy))
        .filter(|&(nx, ny)| nx >= 0 && nx <= max_x && ny >= 0 && ny <= max_y)
        .map(|(nx, ny)| (nx as usize, ny as usize))
        .collect()
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
