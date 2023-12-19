use std::{fs, usize};

use crate::Direction::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!("at the disco"),
        }
    }
}

impl From<u32> for Direction {
    fn from(value: u32) -> Self {
        match value {
            3 => Up,
            1 => Down,
            2 => Left,
            0 => Right,
            _ => panic!("at the disco"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn travel(&self, direction: Direction, distance: isize) -> Point {
        match direction {
            Up => Point {
                x: self.x,
                y: self.y - distance,
            },
            Down => Point {
                x: self.x,
                y: self.y + distance,
            },
            Left => Point {
                x: self.x - distance,
                y: self.y,
            },
            Right => Point {
                x: self.x + distance,
                y: self.y,
            },
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Trench {
    start: Point,
    end: Point,
    colour: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Grid {
    trenches: Vec<Trench>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            trenches: Vec::new(),
        }
    }

    fn swap(&mut self) {
        let mut new_trenches = Vec::new();
        let mut current = Point { x: 0, y: 0 };
        for trench in &self.trenches {
            let inst = format!("{:06x}", trench.colour);
            let distance = isize::from_str_radix(&inst[..5], 16).expect("should be a number");
            let direction = Direction::from(
                inst.chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .expect("should be a number"),
            );
            let next = current.travel(direction, distance);
            new_trenches.push(Trench {
                start: current,
                end: next,
                colour: 0,
            });
            current = next
        }
        self.trenches = new_trenches;
    }

    fn calculate_perimeter(&self) -> usize {
        let mut vertices = Vec::new();
        vertices.push(self.trenches[0].start);
        for trench in &self.trenches {
            vertices.push(trench.end)
        }
        let mut perimeter = 0;
        for i in 0..vertices.len() - 1 {
            perimeter += (vertices[i + 1].x - vertices[i].x).abs()
                + (vertices[i + 1].y - vertices[i].y).abs();
        }
        perimeter += (vertices.last().unwrap().x - vertices[0].x).abs()
            + (vertices.last().unwrap().y - vertices[0].y).abs();
        perimeter as usize
    }

    fn calculate_area(&self) -> usize {
        let mut vertices = Vec::new();
        vertices.push(self.trenches[0].start);
        for trench in &self.trenches {
            vertices.push(trench.end)
        }
        let mut area = 0;
        for i in 0..vertices.len() - 1 {
            area += vertices[i].x * vertices[i + 1].y - vertices[i].y * vertices[i + 1].x;
            if i % 2 == 0 {
                //area += 1;
            }
        }
        area +=
            vertices.last().unwrap().x * vertices[0].y - vertices[0].x * vertices.last().unwrap().y;
        area as usize / 2
    }
}

impl From<String> for Grid {
    fn from(value: String) -> Self {
        let mut current = Point { x: 0, y: 0 };
        let mut grid = Grid::new();
        for line in value.lines() {
            let mut splits = line.split(" ");
            let direction = Direction::from(splits.next().expect("no direction"));
            let distance = splits
                .next()
                .expect("no distance")
                .parse::<isize>()
                .expect("should be a number");
            let colour = usize::from_str_radix(
                splits
                    .next()
                    .expect("no colour")
                    .trim_start_matches("(#")
                    .trim_end_matches(")"),
                16,
            )
            .expect("should be a hex string");
            let next = current.travel(direction, distance);
            grid.trenches.push(Trench {
                start: current,
                end: next,
                colour,
            });
            current = next;
        }
        grid
    }
}

fn part1(grid: Grid) -> usize {
    let area = grid.calculate_area();
    let perimeter = grid.calculate_perimeter();
    area + (perimeter / 2) + 1
}

fn part2(mut grid: Grid) -> usize {
    grid.swap();
    let area = grid.calculate_area();
    let perimeter = grid.calculate_perimeter();
    area + (perimeter / 2) + 1
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let grid = Grid::from(input);
    println!("{}", part1(grid.clone()));
    println!("{}", part2(grid.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let grid = Grid::from(input);
        assert_eq!(62, part1(grid));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let grid = Grid::from(input);
        assert_eq!(952408144115, part2(grid));
    }
}
