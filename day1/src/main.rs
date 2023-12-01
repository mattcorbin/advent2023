use std::collections::HashMap;
use std::fs;

fn part1(input: &str) {
    let mut sum = 0;
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    for line in input.lines() {
        let mut calib = String::new();
        let mut first = 'a';
        let mut last = 'z';
        for c in line.chars() {
            if digits.contains(&c) {
                if first == 'a' {
                    first = c;
                }
                last = c;
            }
        }
        calib.push(first);
        calib.push(last);
        sum += calib.parse::<usize>().unwrap();
    }
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let map = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);
    let mut sum = 0;
    let digits = vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    for line in input.lines() {
        let mut calib = String::new();
        let mut first = String::new();
        let mut last = String::new();
        let mut buf = String::new();
        for c in line.chars() {
            if digits.contains(&c) {
                if first.is_empty() {
                    first = c.to_string();
                }
                last = c.to_string();
                buf = String::new();
            } else {
                buf.push(c);
                for (k, v) in map.iter() {
                    if buf.contains(k) {
                        if first.is_empty() {
                            first = v.to_string();
                        }
                        last = v.to_string();
                        buf = String::new();
                        buf.push(c);
                    }
                }
            }
        }
        calib.push_str(&first);
        calib.push_str(&last);
        sum += calib.parse::<usize>().unwrap();
    }
    println!("part2: {}", sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
