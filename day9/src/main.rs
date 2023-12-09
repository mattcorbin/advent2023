use std::fs;

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut retval = Vec::new();
    for line in input.lines() {
        retval.push(
            line.split(" ")
                .map(|v| v.parse::<isize>().expect("should be an int"))
                .collect(),
        );
    }
    retval
}

fn part1(input: &str) {
    let mut sum = 0;
    let sequences = parse_input(input);
    for sequence in sequences {
        let mut subs = Vec::new();
        let mut current = sequence.clone();
        subs.push(current.clone());
        while current
            .clone()
            .into_iter()
            .filter(|&v| v != 0)
            .collect::<Vec<isize>>()
            .len()
            > 0
        {
            let mut diffs = Vec::new();
            for window in current.windows(2) {
                diffs.push(window[1] - window[0]);
            }
            current = diffs.clone();
            subs.push(diffs);
        }
        for i in (0..subs.len()).rev() {
            if subs[i].iter().all(|&x| x == 0) {
                subs[i].push(0);
                continue;
            }
            let a = *subs[i].last().expect("item?");
            let b = *subs[i + 1].last().expect("item??");
            subs[i].push(a + b);
        }
        sum += subs[0].last().expect("item???");
    }
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let mut sum = 0;
    let sequences = parse_input(input);
    for sequence in sequences {
        let mut subs = Vec::new();
        let mut current = sequence.clone();
        subs.push(current.clone());
        while current
            .clone()
            .into_iter()
            .filter(|&v| v != 0)
            .collect::<Vec<isize>>()
            .len()
            > 0
        {
            let mut diffs = Vec::new();
            for window in current.windows(2) {
                diffs.push(window[1] - window[0]);
            }
            current = diffs.clone();
            subs.push(diffs);
        }
        for i in (0..subs.len()).rev() {
            if subs[i].iter().all(|&x| x == 0) {
                subs[i].push(0);
                continue;
            }
            let a = *subs[i].first().expect("item?");
            let b = *subs[i + 1].first().expect("item??");
            let mut temp = subs[i].clone();
            temp.reverse();
            temp.push(a - b);
            temp.reverse();
            subs[i] = temp.clone();
        }
        sum += subs[0].first().expect("item???");
    }
    println!("part2: {}", sum)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
