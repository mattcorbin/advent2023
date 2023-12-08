use std::collections::HashMap;
use std::fs;

use num::integer;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("wtf"),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Instructions {
    instructions: Vec<Direction>,
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let mut instructions = Vec::new();
        for c in value.chars() {
            instructions.push(Direction::from(c));
        }
        Instructions { instructions }
    }
}

fn parse_input(input: &str) -> (Instructions, HashMap<String, (String, String)>) {
    let mut instructions = Instructions::default();
    let mut map = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if line.contains(" = ") {
            let mut splits = line.split(" = ");
            let key = splits.next().unwrap().to_string();
            let values: Vec<String> = splits
                .next()
                .unwrap()
                .trim_matches(|c| c == '(' || c == ')')
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            map.insert(key, (values[0].clone(), values[1].clone()));
        } else {
            instructions = Instructions::from(line);
        }
    }
    (instructions, map)
}

fn part1(input: &str) {
    let (instruction, map) = parse_input(input);
    let end = "ZZZ".to_string();
    let mut steps = 0;
    let mut idx = 0;
    let mut current = "AAA".to_string();
    while current != end {
        if idx == instruction.instructions.len() {
            idx = 0;
        }
        let opts = map.get(&current).unwrap().clone();
        match instruction.instructions[idx] {
            Direction::Left => current = opts.0,
            Direction::Right => current = opts.1,
        }
        steps += 1;
        idx += 1;
    }
    println!("part1: {}", steps)
}

fn part2(input: &str) {
    let (instruction, map) = parse_input(input);
    let current: Vec<String> = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| k.to_string())
        .collect();
    let mut cycles: Vec<usize> = Vec::new();
    for val in current.iter() {
        let mut inst = 0;
        let mut first_z = 0;
        let mut second_z = 0;
        let mut steps = 0;
        let mut active = val.clone();
        loop {
            if active.ends_with("Z") {
                if first_z == 0 {
                    first_z = steps;
                } else {
                    second_z = steps;
                    break;
                }
            }
            if inst == instruction.instructions.len() {
                inst = 0;
            }
            let opts = map.get(&active).unwrap().clone();
            match instruction.instructions[inst] {
                Direction::Left => active = opts.0,
                Direction::Right => active = opts.1,
            }
            steps += 1;
            inst += 1;
        }
        cycles.push(second_z - first_z);
    }
    println!(
        "part2: {}",
        cycles
            .into_iter()
            .reduce(|a, b| integer::lcm(a, b))
            .unwrap()
    )
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
