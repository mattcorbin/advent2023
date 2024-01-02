use std::cmp::max;
use std::fs;

use petgraph::{algo, prelude::*};

use crate::Tile::*;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Tile {
    Path,
    Forest,
    NSlope,
    ESlope,
    SSlope,
    WSlope,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Path,
            '#' => Forest,
            '^' => NSlope,
            '>' => ESlope,
            'v' => SSlope,
            '<' => WSlope,
            _ => panic!("at the disco!"),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State {
    position: (usize, usize),
}

#[derive(Clone, Debug)]
struct Map {
    start: (usize, usize),
    finish: (usize, usize),
    digraph: DiGraphMap<(usize, usize), usize>,
    ungraph: UnGraphMap<(usize, usize), usize>,
}

impl From<&String> for Map {
    fn from(value: &String) -> Self {
        let map = value
            .lines()
            .map(|line| line.chars().map(|c| Tile::from(c)).collect())
            .collect::<Vec<Vec<Tile>>>();
        let no_slopes = map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|item| {
                        if ![Path, Forest].contains(item) {
                            Path
                        } else {
                            *item
                        }
                    })
                    .collect()
            })
            .collect();
        let mut start = (0, 0);
        let mut finish = (0, 0);

        for (idx, item) in map[0].iter().enumerate() {
            if *item == Path {
                start = (idx, 0);
            }
        }
        for (idx, item) in map.last().unwrap().iter().enumerate() {
            if *item == Path {
                finish = (idx, map.len() - 1);
            }
        }

        let mut digraph = DiGraphMap::<(usize, usize), usize>::new();
        let mut ungraph = UnGraphMap::<(usize, usize), usize>::new();

        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] != Forest {
                    for neighbour in calculate_neighbours(&map, x, y) {
                        digraph.add_edge((x, y), neighbour, 1);
                    }
                    for neighbour in calculate_neighbours(&no_slopes, x, y) {
                        ungraph.add_edge((x, y), neighbour, 1);
                    }
                }
            }
        }

        Map {
            start,
            finish,
            digraph,
            ungraph,
        }
    }
}

fn calculate_neighbours(map: &Vec<Vec<Tile>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut retval = Vec::new();
    let tile = map[y][x];
    match tile {
        Path => {
            if x != 0 {
                if ![Forest, ESlope].contains(&map[y][x - 1]) {
                    retval.push((x - 1, y));
                }
            }
            if y != 0 {
                if ![Forest, SSlope].contains(&map[y - 1][x]) {
                    retval.push((x, y - 1));
                }
            }
            if x != map[0].len() - 1 {
                if ![Forest, WSlope].contains(&map[y][x + 1]) {
                    retval.push((x + 1, y));
                }
            }
            if y != map.len() - 1 {
                if ![Forest, NSlope].contains(&map[y + 1][x]) {
                    retval.push((x, y + 1));
                }
            }
        }
        Forest => (),
        NSlope => {
            if y != 0 {
                if ![Forest, SSlope].contains(&map[y - 1][x]) {
                    retval.push((x, y - 1));
                }
            }
        }
        ESlope => {
            if x != map[0].len() - 1 {
                if ![Forest, WSlope].contains(&map[y][x + 1]) {
                    retval.push((x + 1, y));
                }
            }
        }
        SSlope => {
            if y != map.len() - 1 {
                if ![Forest, NSlope].contains(&map[y + 1][x]) {
                    retval.push((x, y + 1));
                }
            }
        }
        WSlope => {
            if x != 0 {
                if ![Forest, ESlope].contains(&map[y][x - 1]) {
                    retval.push((x - 1, y));
                }
            }
        }
    }
    retval
}

impl Map {
    fn solve(&self, directed: bool) -> usize {
        let mut distance = 0;
        if directed {
            for path in
                algo::all_simple_paths::<Vec<_>, _>(&self.digraph, self.start, self.finish, 0, None)
            {
                distance = max(distance, path.len());
            }
        } else {
            for path in
                algo::all_simple_paths::<Vec<_>, _>(&self.ungraph, self.start, self.finish, 0, None)
            {
                distance = max(distance, path.len());
            }
        }
        distance - 1
    }
}

fn part1(map: &Map) -> usize {
    map.solve(true)
}

fn part2(map: &Map) -> usize {
    map.solve(false)
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
        assert_eq!(94, part1(&map));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let map = Map::from(&input);
        assert_eq!(154, part2(&map));
    }
}
