use std::fs;

fn part1(input: &str) {
    println!("part1: {}", 0)
}

fn part2(input: &str) {
    println!("part2: {}", 0)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
