use std::fs;

use pathfinding::matrix::{directions, Matrix};
use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    position: (usize, usize),
    direction: (isize, isize),
    distance: usize,
}

#[derive(Clone, Debug)]
struct Map {
    matrix: Matrix<usize>,
}

impl From<&String> for Map {
    fn from(value: &String) -> Self {
        Map {
            matrix: value
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
                .collect(),
        }
    }
}

impl Map {
    fn compute_successors<const MIN: usize, const MAX: usize>(
        &self,
        state: &State,
    ) -> Vec<(State, usize)> {
        let mut successors = Vec::new();
        if state.direction == (0, 0) || state.distance >= MIN {
            let possible_directions = [directions::N, directions::W, directions::S, directions::E];
            let previous_position = self.matrix.move_in_direction(
                state.position,
                (-1 * state.direction.0, -1 * state.direction.1),
            );
            for dir in possible_directions {
                if let Some(new_position) = self.matrix.move_in_direction(state.position, dir) {
                    if state.direction == (0, 0) || !(new_position == previous_position.unwrap()) {
                        let distance = match state.direction == dir {
                            true => state.distance + 1,
                            false => 1,
                        };
                        if distance <= MAX {
                            successors.push((
                                State {
                                    position: new_position,
                                    direction: dir,
                                    distance,
                                },
                                *self.matrix.get(new_position).unwrap(),
                            ))
                        }
                    }
                }
            }
        } else {
            if let Some(position) = self
                .matrix
                .move_in_direction(state.position, state.direction)
            {
                let cost = *self.matrix.get(position).expect("valid position");
                let new_state = State {
                    position,
                    direction: state.direction,
                    distance: state.distance + 1,
                };
                successors.push((new_state, cost))
            }
        }
        successors
    }

    fn solve<const MIN: usize, const MAX: usize>(&self) -> usize {
        let start = State {
            position: (0, 0),
            direction: (0, 0),
            distance: 0,
        };

        let finish = (self.matrix.rows - 1, self.matrix.columns - 1);
        dijkstra(
            &start,
            |state| self.compute_successors::<MIN, MAX>(state),
            |state| state.position == finish && state.distance >= MIN,
        )
        .expect("should be a path")
        .1
    }
}

fn part1(map: &Map) -> usize {
    map.solve::<1, 3>()
}

fn part2(map: &Map) -> usize {
    map.solve::<4, 10>()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let map = Map::from(&input);
    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let map = Map::from(&input);
        assert_eq!(102, part1(&map));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let map = Map::from(&input);
        assert_eq!(94, part2(&map));
    }
}
