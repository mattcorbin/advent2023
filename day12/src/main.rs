use std::fs;

use cached::proc_macro::cached;
use itertools::{repeat_n, Itertools};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringState {
    fn from(value: char) -> Self {
        match value {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("wtf"),
        }
    }
}

#[derive(Clone, Debug, Hash)]
struct Springs {
    springs: Vec<SpringState>,
    groups: Vec<usize>,
}

impl From<&str> for Springs {
    fn from(value: &str) -> Self {
        let mut splits = value.split(" ");
        let springs = splits
            .next()
            .unwrap()
            .chars()
            .map(|c| SpringState::from(c))
            .collect();
        let groups = splits
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        Springs { springs, groups }
    }
}

impl Springs {
    fn valid(&self, arrangement: &Vec<SpringState>) -> bool {
        let mut groups = Vec::new();
        let mut count = 0;
        for &item in arrangement {
            if item == SpringState::Operational && count != 0 {
                groups.push(count);
                count = 0;
            } else if item == SpringState::Damaged {
                count += 1;
            }
        }
        if count > 0 {
            groups.push(count);
        }
        groups == self.groups
    }

    fn valid_arrangements(&self) -> usize {
        let mut valid_arrangements = 0;
        let unknowns = self
            .springs
            .iter()
            .filter(|&&v| v == SpringState::Unknown)
            .collect::<Vec<&SpringState>>()
            .len();
        let opts = vec![SpringState::Damaged, SpringState::Operational];
        let arrangements = repeat_n(opts, unknowns).multi_cartesian_product();
        for arrangement in arrangements {
            let mut local = arrangement.clone();
            let mut test = self.springs.clone();
            while let Some(pos) = test.iter().position(|&x| x == SpringState::Unknown) {
                test[pos] = local.pop().unwrap();
            }
            if self.valid(&test) {
                valid_arrangements += 1;
            }
        }
        valid_arrangements
    }

    fn unfold(&mut self) {
        let mut new_springs = Vec::new();
        let mut new_groups = Vec::new();
        for _ in 0..4 {
            new_springs.append(&mut self.springs.clone());
            new_springs.push(SpringState::Unknown);
            new_groups.append(&mut self.groups.clone());
        }
        new_springs.append(&mut self.springs.clone());
        new_groups.append(&mut self.groups.clone());
        self.springs = new_springs;
        self.groups = new_groups
    }
}

fn part1(input: &Vec<Springs>) {
    let mut sum = 0;
    for item in input {
        sum += item.valid_arrangements()
    }
    println!("part1: {}", sum)
}

#[cached]
fn valid_arrangements_inner(springs: Vec<SpringState>, groups: Vec<usize>) -> usize {
    let first = groups[0];
    let rest = &groups[1..];
    let after = rest.iter().sum::<usize>() + rest.len();
    let mut count = 0;

    for before in 0..(springs.len() - after - first + 1) {
        if springs[before..before + first]
            .iter()
            .all(|x| [SpringState::Damaged, SpringState::Unknown].contains(x))
        {
            if rest.len() == 0 {
                if springs[before + first..]
                    .iter()
                    .all(|x| [SpringState::Operational, SpringState::Unknown].contains(x))
                {
                    count += 1
                }
            } else if [SpringState::Operational, SpringState::Unknown]
                .contains(&springs[before + first])
            {
                count +=
                    valid_arrangements_inner(springs[before + first + 1..].to_vec(), rest.to_vec())
            }
        }
        if ![SpringState::Operational, SpringState::Unknown].contains(&springs[before]) {
            break;
        }
    }

    count
}

fn valid_arrangements(item: &Springs) -> usize {
    valid_arrangements_inner(item.springs.clone(), item.groups.clone())
}

fn part2(input: &Vec<Springs>) {
    let mut sum = 0;
    for item in input {
        sum += valid_arrangements(item);
    }
    println!("part2: {}", sum);
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut springs = Vec::new();
    for line in input.lines() {
        springs.push(Springs::from(line))
    }
    part1(&springs);
    let mut unfolded_springs = springs.clone();
    unfolded_springs.iter_mut().for_each(|v| v.unfold());
    part2(&unfolded_springs);
}
