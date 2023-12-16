use std::collections::HashSet;
use std::fs;

use crate::Direction::*;
use crate::Space::*;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Space {
    Empty,
    RightMirror,
    LeftMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Empty,
            '/' => RightMirror,
            '\\' => LeftMirror,
            '-' => HorizontalSplitter,
            '|' => VerticalSplitter,
            _ => panic!("at the disco!"),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Contraption {
    spaces: Vec<Vec<Space>>,
}

impl From<&String> for Contraption {
    fn from(value: &String) -> Self {
        let mut spaces = Vec::new();
        for line in value.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(Space::from(c));
            }
            spaces.push(row);
        }
        Contraption { spaces }
    }
}

impl Contraption {
    fn next(&self, x: usize, y: usize, direction: Direction) -> Option<(usize, usize)> {
        match direction {
            Up => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Down => {
                if y + 1 == self.spaces.len() {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Left => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Right => {
                if x + 1 == self.spaces[y].len() {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
        }
    }

    fn travel(
        &self,
        x: usize,
        y: usize,
        direction: Direction,
        visited: &mut HashSet<(usize, usize, Direction)>,
    ) {
        visited.insert((x, y, direction));
        let current_space = self.spaces[y][x];
        match current_space {
            Empty => {
                if let Some((next_x, next_y)) = self.next(x, y, direction) {
                    if visited.contains(&(next_x, next_y, direction)) {
                        return;
                    }
                    self.travel(next_x, next_y, direction, visited)
                }
            }
            RightMirror => match direction {
                Up => {
                    if let Some((next_x, next_y)) = self.next(x, y, Right) {
                        if visited.contains(&(next_x, next_y, Right)) {
                            return;
                        }
                        self.travel(next_x, next_y, Right, visited)
                    }
                }
                Down => {
                    if let Some((next_x, next_y)) = self.next(x, y, Left) {
                        if visited.contains(&(next_x, next_y, Left)) {
                            return;
                        }
                        self.travel(next_x, next_y, Left, visited)
                    }
                }
                Left => {
                    if let Some((next_x, next_y)) = self.next(x, y, Down) {
                        if visited.contains(&(next_x, next_y, Down)) {
                            return;
                        }
                        self.travel(next_x, next_y, Down, visited)
                    }
                }
                Right => {
                    if let Some((next_x, next_y)) = self.next(x, y, Up) {
                        if visited.contains(&(next_x, next_y, Up)) {
                            return;
                        }
                        self.travel(next_x, next_y, Up, visited)
                    }
                }
            },
            LeftMirror => match direction {
                Up => {
                    if let Some((next_x, next_y)) = self.next(x, y, Left) {
                        if visited.contains(&(next_x, next_y, Left)) {
                            return;
                        }
                        self.travel(next_x, next_y, Left, visited)
                    }
                }
                Down => {
                    if let Some((next_x, next_y)) = self.next(x, y, Right) {
                        if visited.contains(&(next_x, next_y, Right)) {
                            return;
                        }
                        self.travel(next_x, next_y, Right, visited)
                    }
                }
                Left => {
                    if let Some((next_x, next_y)) = self.next(x, y, Up) {
                        if visited.contains(&(next_x, next_y, Up)) {
                            return;
                        }
                        self.travel(next_x, next_y, Up, visited)
                    }
                }
                Right => {
                    if let Some((next_x, next_y)) = self.next(x, y, Down) {
                        if visited.contains(&(next_x, next_y, Down)) {
                            return;
                        }
                        self.travel(next_x, next_y, Down, visited)
                    }
                }
            },
            HorizontalSplitter => match direction {
                Up => {
                    if let Some((next_x, next_y)) = self.next(x, y, Right) {
                        if visited.contains(&(next_x, next_y, Right)) {
                            return;
                        }
                        self.travel(next_x, next_y, Right, visited)
                    }
                    if let Some((next_x, next_y)) = self.next(x, y, Left) {
                        if visited.contains(&(next_x, next_y, Left)) {
                            return;
                        }
                        self.travel(next_x, next_y, Left, visited)
                    }
                }
                Down => {
                    if let Some((next_x, next_y)) = self.next(x, y, Right) {
                        if visited.contains(&(next_x, next_y, Right)) {
                            return;
                        }
                        self.travel(next_x, next_y, Right, visited)
                    }
                    if let Some((next_x, next_y)) = self.next(x, y, Left) {
                        if visited.contains(&(next_x, next_y, Left)) {
                            return;
                        }
                        self.travel(next_x, next_y, Left, visited)
                    }
                }
                Left => {
                    if let Some((next_x, next_y)) = self.next(x, y, direction) {
                        if visited.contains(&(next_x, next_y, direction)) {
                            return;
                        }
                        self.travel(next_x, next_y, direction, visited)
                    }
                }
                Right => {
                    if let Some((next_x, next_y)) = self.next(x, y, direction) {
                        if visited.contains(&(next_x, next_y, direction)) {
                            return;
                        }
                        self.travel(next_x, next_y, direction, visited)
                    }
                }
            },
            VerticalSplitter => match direction {
                Up => {
                    if let Some((next_x, next_y)) = self.next(x, y, direction) {
                        if visited.contains(&(next_x, next_y, direction)) {
                            return;
                        }
                        self.travel(next_x, next_y, direction, visited)
                    }
                }
                Down => {
                    if let Some((next_x, next_y)) = self.next(x, y, direction) {
                        if visited.contains(&(next_x, next_y, direction)) {
                            return;
                        }
                        self.travel(next_x, next_y, direction, visited)
                    }
                }
                Left => {
                    if let Some((next_x, next_y)) = self.next(x, y, Up) {
                        if visited.contains(&(next_x, next_y, Up)) {
                            return;
                        }
                        self.travel(next_x, next_y, Up, visited)
                    }
                    if let Some((next_x, next_y)) = self.next(x, y, Down) {
                        if visited.contains(&(next_x, next_y, Down)) {
                            return;
                        }
                        self.travel(next_x, next_y, Down, visited)
                    }
                }
                Right => {
                    if let Some((next_x, next_y)) = self.next(x, y, Up) {
                        if visited.contains(&(next_x, next_y, Up)) {
                            return;
                        }
                        self.travel(next_x, next_y, Up, visited)
                    }
                    if let Some((next_x, next_y)) = self.next(x, y, Down) {
                        if visited.contains(&(next_x, next_y, Down)) {
                            return;
                        }
                        self.travel(next_x, next_y, Down, visited)
                    }
                }
            },
        }
    }

    fn count_energized_points(&self, x: usize, y: usize, direction: Direction) -> usize {
        let mut visited = HashSet::new();
        self.travel(x, y, direction, &mut visited);
        let mut energized = HashSet::new();
        for point in visited.into_iter().map(|(x, y, _)| (x, y)) {
            energized.insert(point);
        }
        energized.len()
    }

    fn direction(&self, x: usize, y: usize) -> Vec<Direction> {
        let mut retval = Vec::new();
        if x == 0 {
            retval.push(Right);
        }
        if x + 1 == self.spaces[y].len() {
            retval.push(Left);
        }
        if y == 0 {
            retval.push(Down);
        }
        if y + 1 == self.spaces.len() {
            retval.push(Up);
        }
        retval
    }
}

fn part1(contraption: &Contraption) -> usize {
    contraption.count_energized_points(0, 0, Right)
}

fn part2(contraption: &Contraption) -> usize {
    let mut configurations = Vec::new();
    let mut points = HashSet::new();
    for y in 0..contraption.spaces.len() {
        points.insert((0, y));
        points.insert((contraption.spaces[y].len() - 1, y));
    }
    for x in 0..contraption.spaces[0].len() {
        points.insert((x, 0));
        points.insert((x, contraption.spaces.len() - 1));
    }
    for (x, y) in points {
        for dir in contraption.direction(x, y) {
            configurations.push(contraption.count_energized_points(x, y, dir))
        }
    }
    configurations.into_iter().max().unwrap()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let contraption = Contraption::from(&input);
    println!("part1: {}", part1(&contraption));
    println!("part2: {}", part2(&contraption));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let contraption = Contraption::from(&input);
        assert_eq!(46, part1(&contraption));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let contraption = Contraption::from(&input);
        assert_eq!(51, part2(&contraption));
    }
}
