use std::{collections::HashSet, vec};

#[derive(Debug)]
struct HydrothermalVent {
    start: Point,
    end: Point
}

impl HydrothermalVent {
    fn intersection(&self, other: &HydrothermalVent) -> Vec<Point> {
        if self.is_horizontal() && other.is_horizontal() && self.start.y == other.start.y {
            let min_x = self.min_x();
            let max_x = self.max_x();
            

            let x_intersection_start = min_x.max(other.min_x());
            let x_intersection_end = max_x.min(other.max_x());

            (x_intersection_start..x_intersection_end + 1).map(|x| Point {x, y: self.start.y }).collect()
        } else if self.is_vertical() && other.is_vertical() && self.start.x == other.start.x {
            let min_y = self.min_y();
            let max_y = self.max_y();
            
            let y_intersection_start = min_y.max(other.min_y());
            let y_intersection_end = max_y.min(other.max_y());

            (y_intersection_start..y_intersection_end + 1).map(|y| Point {x: self.start.x, y }).collect()
        } else {
            if self.is_horizontal() && other.is_vertical() {
                return self.intersection_with_vertical(other)
            } else if self.is_vertical() && other.is_horizontal() {
                return self.intersetion_with_horizontal(other)
            }
            Vec::new()
        }
    }

    fn intersection_with_vertical(&self, other: &HydrothermalVent) -> Vec<Point> {
        if (self.min_x()..self.max_x() + 1).contains(&other.min_x())
            && (other.min_y()..other.max_y() + 1).contains(&self.min_y()) {
                return vec![Point { x: other.min_x(), y: self.min_y() }]
        }
        Vec::new()
    }

    fn intersetion_with_horizontal(&self, other: &HydrothermalVent) -> Vec<Point> {
        if (other.min_x()..other.max_x() + 1).contains(&self.min_x())
            && (self.min_y()..self.max_y() + 1).contains(&other.min_y()) {
                return vec![Point { x: self.min_x(), y: other.min_y() }]
        }
        Vec::new()
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
    y: u32
}

impl Point {
    fn parse(line: &str) -> Point {
        let points: Vec<u32> = line
            .trim()
            .split(',')
            .map(|c| c.parse().unwrap())
            .take(2)
            .collect();

        Point { x: points[0], y: points[1] }        
    }
}

fn main() {
    let input = include_str!("../input").trim();
    
    let hydrothermal_vents = parse_input(input);
    let mut points_with_intersection = HashSet::new();
    for i in 0..hydrothermal_vents.len() {
        for j in i + 1..hydrothermal_vents.len() {
            let first = &hydrothermal_vents[i];
            let second = &hydrothermal_vents[j];

            let intersection = first.intersection(second);
            for ele in intersection {
                points_with_intersection.insert(ele);
            }
        }
    }

    println!("{}", points_with_intersection.len());
}

fn parse_input(input: &str) -> Vec<HydrothermalVent> {
    let lines: Vec<&str> = input.split('\n').map(str::trim).collect();
    lines
        .iter()
        .map(|&line| {
            line
                .split(" -> ")
                .map(str::trim)
                .map(Point::parse)
                .collect()
        })
        .map(|point_pair: Vec<Point> | HydrothermalVent { start: point_pair[0], end: point_pair[1] } )
        .filter(|hydrothermal_vent| hydrothermal_vent.start.x == hydrothermal_vent.end.x || hydrothermal_vent.start.y == hydrothermal_vent.end.y)
        .collect()
}
