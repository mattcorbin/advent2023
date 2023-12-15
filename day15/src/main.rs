use std::collections::HashMap;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Op {
    Remove,
    Add,
}

#[derive(Debug)]
struct Lens {
    label: Vec<char>,
    focal_length: usize,
}

struct Instruction {
    code: Vec<char>,
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        Instruction {
            code: value.chars().collect(),
        }
    }
}

fn HASH(input: &Vec<char>) -> usize {
    let mut current_value = 0;
    for c in input {
        current_value += *c as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}

impl Instruction {
    fn HASH(&self) -> usize {
        HASH(&self.code)
    }

    fn HASHMAP(&self) -> (usize, Op, Lens) {
        let b;
        let op;
        let lens;
        if self.code.contains(&'=') {
            let mut label = self.code.clone();
            let focal_length = label.pop().unwrap().to_digit(10).unwrap() as usize;
            label.pop();

            b = HASH(&label);
            op = Op::Add;
            lens = Lens {
                label,
                focal_length,
            }
        } else {
            let mut label = self.code.clone();
            label.pop();
            b = HASH(&label);
            op = Op::Remove;
            lens = Lens {
                label,
                focal_length: 0,
            }
        }
        (b, op, lens)
    }
}

fn part1(instructions: &Vec<Instruction>) {
    let mut sum = 0;
    for inst in instructions {
        sum += inst.HASH();
    }
    println!("part1: {}", sum)
}

fn part2(instructions: &Vec<Instruction>) {
    let mut boxes: HashMap<usize, Vec<Lens>> = HashMap::new();
    for inst in instructions {
        let (key, op, lens) = inst.HASHMAP();
        match op {
            Op::Remove => {
                if let Some(lenses) = boxes.get_mut(&key) {
                    if let Some(index) = lenses.iter().position(|l| l.label == lens.label) {
                        lenses.remove(index);
                    }
                }
            }
            Op::Add => {
                if let Some(lenses) = boxes.get_mut(&key) {
                    if let Some(index) = lenses.iter().position(|l| l.label == lens.label) {
                        lenses[index] = lens;
                    } else {
                        lenses.push(lens);
                    }
                } else {
                    let mut lenses = Vec::new();
                    lenses.push(lens);
                    boxes.insert(key, lenses);
                }
            }
        }
    }
    let mut focusing_power = 0;
    for (key, value) in boxes.iter() {
        for (index, lens) in value.iter().enumerate() {
            focusing_power += (*key + 1) * (index + 1) * lens.focal_length;
        }
    }
    println!("part2: {}", focusing_power)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut instructions = Vec::new();
    for item in input.split(",") {
        instructions.push(Instruction::from(item));
    }
    part1(&instructions);
    part2(&instructions);
}
