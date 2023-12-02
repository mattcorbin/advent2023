use std::cmp::max;
use std::fs;
use std::str::FromStr;

enum Colour {
    Red,
    Green,
    Blue,
}

impl FromStr for Colour {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "red" => Colour::Red,
            "blue" => Colour::Blue,
            "green" => Colour::Green,
            _ => panic!("wtf"),
        })
    }
}

struct Game {
    id: usize,
    cubes: Vec<Vec<(usize, Colour)>>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(": ");
        let mut cubes = Vec::new();
        let id = splits
            .next()
            .unwrap()
            .replace("Game ", "")
            .parse::<usize>()
            .unwrap();
        let mut splits = splits.next().unwrap().split("; ");
        for split in splits {
            let mut new = Vec::new();
            let mut cubesets = split.split(", ");
            for item in cubesets {
                let mut pairs = item.split(" ");
                let count = pairs.next().unwrap().parse::<usize>().unwrap();
                let colour = Colour::from_str(pairs.next().unwrap()).unwrap();
                new.push((count, colour));
            }
            cubes.push(new);
        }

        Ok(Game { id, cubes })
    }
}

impl Game {
    fn possible(&self) -> bool {
        for set in &self.cubes {
            for (count, colour) in set {
                match colour {
                    Colour::Red => {
                        if *count > 12 {
                            return false;
                        }
                    }
                    Colour::Green => {
                        if *count > 13 {
                            return false;
                        }
                    }
                    Colour::Blue => {
                        if *count > 14 {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }

    fn power(&self) -> usize {
        let mut max_r = 0;
        let mut max_g = 0;
        let mut max_b = 0;
        for set in &self.cubes {
            for (count, colour) in set {
                match colour {
                    Colour::Red => max_r = max(max_r, *count),
                    Colour::Green => max_g = max(max_g, *count),
                    Colour::Blue => max_b = max(max_b, *count),
                }
            }
        }
        max_r * max_g * max_b
    }
}

fn part1(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::from_str(line).unwrap();
        if game.possible() {
            sum += game.id;
        }
    }
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let mut sum = 0;
    for line in input.lines() {
        let game = Game::from_str(line).unwrap();
        sum += game.power();
    }
    println!("part1: {}", sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
