use std::collections::HashSet;
use std::fs;

use itertools::Itertools;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Reflection {
    axis: Axis,
    index: usize,
}

struct Pattern {
    graph: Vec<Vec<char>>,
}

impl From<&String> for Pattern {
    fn from(value: &String) -> Self {
        let mut graph = Vec::new();
        for line in value.lines() {
            graph.push(line.chars().collect());
        }
        Pattern { graph }
    }
}

impl Pattern {
    fn find_reflection_horizontal(&self, skip: Option<Reflection>) -> Result<Reflection, String> {
        let axis = Axis::Horizontal;
        for i in 1..self.graph.len() {
            if self.graph[i] == self.graph[i - 1] {
                let mut reflection = true;
                let mut forward = i + 1;
                let mut backward = i as isize - 2;
                while forward < self.graph.len() && backward >= 0 {
                    if self.graph[forward] != self.graph[backward as usize] {
                        reflection = false;
                        break;
                    }
                    forward += 1;
                    backward -= 1;
                }
                if reflection {
                    let ret = Reflection { axis, index: i };
                    if let Some(s) = skip {
                        if !(s == ret) {
                            return Ok(ret);
                        }
                    } else {
                        return Ok(ret);
                    }
                }
            }
        }
        Err("no reflection".to_string())
    }

    fn find_reflection_vertical(&self, skip: Option<Reflection>) -> Result<Reflection, String> {
        let axis = Axis::Vertical;
        for i in 1..self.graph[0].len() {
            let a = self.graph.iter().map(|x| x[i]).collect::<Vec<char>>();
            let b = self.graph.iter().map(|x| x[i - 1]).collect::<Vec<char>>();
            if a == b {
                let mut reflection = true;
                let mut forward = i + 1;
                let mut backward = i as isize - 2;
                while forward < self.graph[0].len() && backward >= 0 {
                    let a = self.graph.iter().map(|x| x[forward]).collect::<Vec<char>>();
                    let b = self
                        .graph
                        .iter()
                        .map(|x| x[backward as usize])
                        .collect::<Vec<char>>();
                    if a != b {
                        reflection = false;
                        break;
                    }
                    forward += 1;
                    backward -= 1;
                }
                if reflection {
                    let ret = Reflection { axis, index: i };
                    if let Some(s) = skip {
                        if !(s == ret) {
                            return Ok(ret);
                        }
                    } else {
                        return Ok(ret);
                    }
                }
            }
        }
        Err("no reflection".to_string())
    }

    fn find_reflection(&self) -> Reflection {
        match self.find_reflection_horizontal(None) {
            Ok(reflection) => reflection,
            Err(_) => self.find_reflection_vertical(None).unwrap(),
        }
    }

    fn smudges(&self) -> Vec<Pattern> {
        let mut patterns = Vec::new();
        let points = (0..self.graph.len()).cartesian_product(0..self.graph[0].len());
        for (y, x) in points {
            let mut graph = self.graph.clone();
            match self.graph[y][x] {
                '#' => graph[y][x] = '.',
                '.' => graph[y][x] = '#',
                _ => panic!("at the disco"),
            }
            patterns.push(Pattern { graph });
        }
        patterns
    }
}

fn part1(patterns: &Vec<Pattern>) {
    let mut sum = 0;
    for pattern in patterns.iter() {
        let reflection = pattern.find_reflection();
        match reflection.axis {
            Axis::Horizontal => sum += reflection.index * 100,
            Axis::Vertical => sum += reflection.index,
        }
    }
    println!("part1: {}", sum)
}

fn part2(patterns: &Vec<Pattern>) {
    let mut sum = 0;
    for (idx, item) in patterns.iter().enumerate() {
        let original = item.find_reflection();
        let mut reflections = HashSet::new();
        for pattern in item.smudges() {
            if let Ok(reflection) = pattern.find_reflection_horizontal(Some(original)) {
                reflections.insert(reflection);
            }
            if let Ok(reflection) = pattern.find_reflection_vertical(Some(original)) {
                reflections.insert(reflection);
            }
        }
        for reflection in &reflections {
            match reflection.axis {
                Axis::Horizontal => sum += reflection.index * 100,
                Axis::Vertical => sum += reflection.index,
            }
        }
        if reflections.len() == 0 {
            println!("{idx}")
        }
    }
    println!("part2: {}", sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut buf = String::new();
    let mut patterns = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            patterns.push(Pattern::from(&buf));
            buf = String::new();
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    patterns.push(Pattern::from(&buf));
    part1(&patterns);
    part2(&patterns);
}
