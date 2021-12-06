use std::vec;

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<u32>>
}

impl Grid {

    fn new(size: u32) -> Grid {
        let mut lines: Vec<Vec<u32>> = Vec::with_capacity(size as usize);

        for _ in 0..size {
            let line: Vec<u32> = vec![0; size as usize];
            lines.push(line);
        }

        Grid { cells: lines }
    }

    fn count(&mut self, point: &Point) {
        let col = &mut self.cells[point.y as usize];
        col[point.x as usize] += 1;
    }

    fn points_above_one(&self) -> u32 {
        let mut sum = 0;

        for line in &self.cells {
            for &item in line {
                if item > 1 {
                    sum += 1;
                }
            }
        }

        sum
    }
}

#[derive(Debug)]
struct HydrothermalVent {
    start: Point,
    end: Point,
}

impl HydrothermalVent {
    fn all_spanning_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            let min_x = self.min_x();
            let max_x = self.max_x();

            (min_x..max_x + 1)
                .map(|x| Point { x, y: self.min_y() })
                .collect()
        } else if self.is_vertical() {
            let min_y = self.min_y();
            let max_y = self.max_y();

            (min_y..max_y + 1)
                .map(|y| Point { x: self.min_x(), y })
                .collect()
        } else if self.start.x >= self.end.x && self.start.y >= self.end.y {
            // step direction:  up-left
            let xs: Vec<u32> = (self.end.x..self.start.x + 1).rev().collect();
            let ys: Vec<u32> = (self.end.y..self.start.y + 1).rev().collect();

            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        } else if self.start.x >= self.end.x && self.start.y < self.end.y {
            // step direction: down-left

            let xs: Vec<u32> = (self.end.x..self.start.x + 1).rev().collect();
            let ys: Vec<u32> = (self.start.y..self.end.y + 1).collect();
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        } else if self.start.x < self.end.x && self.start.y >= self.end.y {
            // step direction: up-right
            let xs: Vec<u32> = (self.start.x..self.end.x + 1).collect();
            let ys: Vec<u32> = (self.end.y..self.start.y + 1).rev().collect();
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        } else {
            // step direction: down-right
            let xs: Vec<u32> = (self.start.x..self.end.x + 1).collect();
            let ys: Vec<u32> = (self.start.y..self.end.y + 1).collect();
            xs.iter()
                .zip(ys.iter())
                .map(|(x, y)| Point { x: *x, y: *y })
                .collect()
        }
    }

    fn min_x(&self) -> u32 {
        self.start.x.min(self.end.x)
    }

    fn max_x(&self) -> u32 {
        self.start.x.max(self.end.x)
    }

    fn min_y(&self) -> u32 {
        self.start.y.min(self.end.y)
    }

    fn max_y(&self) -> u32 {
        self.start.y.max(self.end.y)
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn parse(line: &str) -> Point {
        let points: Vec<u32> = line
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .take(2)
            .collect();

        Point {
            x: points[0],
            y: points[1],
        }
    }
}

fn main() {
    let input = include_str!("../input").trim();

    let hydrothermal_vents = parse_input(input);
    let mut grid = Grid::new(compute_grid_size(&hydrothermal_vents));

    for hydrothermal_vent in hydrothermal_vents {
        let points = hydrothermal_vent.all_spanning_points();

        for point in points {
            grid.count(&point);
        }
    }

    println!("{}", grid.points_above_one());
}

fn compute_grid_size(hydrothermal_vents: &[HydrothermalVent]) -> u32 {
    let max_x = hydrothermal_vents
        .iter()
        .map(HydrothermalVent::max_x)
        .max()
        .unwrap();
    let max_y = hydrothermal_vents
        .iter()
        .map(HydrothermalVent::max_y)
        .max()
        .unwrap();

    max_x.max(max_y) + 1
}

fn parse_input(input: &str) -> Vec<HydrothermalVent> {
    let lines: Vec<&str> = input.split('\n').map(str::trim).collect();
    lines
        .iter()
        .map(|&line| {
            line.split(" -> ")
                .map(str::trim)
                .map(Point::parse)
                .collect()
        })
        .map(|point_pair: Vec<Point>| HydrothermalVent {
            start: point_pair[0],
            end: point_pair[1],
        })
        .collect()
}
