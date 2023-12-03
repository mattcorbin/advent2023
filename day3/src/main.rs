use std::fs;

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const NOT_SYMBOL: [char; 11] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];

fn get_adjacent(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut retval = Vec::new();
    let min_x;
    let max_x;
    let min_y;
    let max_y;

    if x == 0 {
        min_x = 0;
    } else {
        min_x = x - 1;
    }
    if x == 139 {
        max_x = 0;
    } else {
        max_x = x + 1;
    }

    if y == 0 {
        min_y = 0;
    } else {
        min_y = y - 1;
    }

    if y == 139 {
        max_y = 0;
    } else {
        max_y = y + 1;
    }

    // Behind
    retval.push((min_x, min_y));
    retval.push((min_x, y));
    retval.push((min_x, max_y));

    // Above/Below
    retval.push((x, min_y));
    retval.push((x, max_y));

    // Ahead
    retval.push((max_x, min_y));
    retval.push((max_x, y));
    retval.push((max_x, max_y));

    retval
}

fn parse_number(start_x: usize, y: usize, grid: &[[char; 140]; 140]) -> (usize, usize) {
    let mut x = start_x;
    let mut buf = String::new();
    while DIGITS.contains(&grid[y][x]) {
        buf.push(grid[y][x]);
        x += 1;
        if x == 140 {
            break;
        }
    }
    (buf.parse().unwrap(), x - start_x)
}

fn is_part_number(start_x: usize, end_x: usize, y: usize, grid: &[[char; 140]; 140]) -> bool {
    for x in start_x..end_x {
        let adjacent = get_adjacent(x, y);
        for (check_x, check_y) in adjacent {
            if !NOT_SYMBOL.contains(&grid[check_y][check_x]) {
                return true;
            }
        }
    }
    false
}

fn part1(grid: &[[char; 140]; 140]) {
    let mut sum = 0;
    for y in 0..140 {
        let mut x = 0;
        while x < 140 {
            if DIGITS.contains(&grid[y][x]) {
                let (number, size) = parse_number(x, y, grid);
                if is_part_number(x, x + size, y, grid) {
                    sum += number;
                }
                x += size;
            } else {
                x += 1;
            }
        }
    }
    println!("part1: {}", sum)
}

fn find_start(x: usize, y: usize, grid: &[[char; 140]; 140]) -> usize {
    if x == 0 {
        return x;
    }
    let mut new_x = x;
    while DIGITS.contains(&grid[y][new_x]) {
        if new_x == 0 {
            return new_x;
        }
        new_x -= 1;
    }
    new_x + 1
}

fn find_end(x: usize, y: usize, grid: &[[char; 140]; 140]) -> usize {
    if x == 139 {
        return x;
    }
    let mut new_x = x;
    while DIGITS.contains(&grid[y][new_x]) {
        if new_x == 139 {
            return new_x;
        }
        new_x += 1;
    }
    new_x - 1
}

fn is_maybe_gear(adjacent: &Vec<(usize, usize)>, grid: &[[char; 140]; 140]) -> bool {
    let mut numbers = 0;
    for &(x, y) in adjacent {
        if DIGITS.contains(&grid[y][x]) {
            numbers += 1;
        }
    }
    numbers >= 2
}

fn part2(grid: &[[char; 140]; 140]) {
    let mut sum = 0;
    for y in 0..140 {
        for x in 0..140 {
            if grid[y][x] == '*' {
                let mut adjacent = get_adjacent(x, y);
                if is_maybe_gear(&adjacent, &grid) {
                    let mut numbers = Vec::new();
                    while !adjacent.is_empty() {
                        let (test_x, test_y) = adjacent.pop().unwrap();
                        if DIGITS.contains(&grid[test_y][test_x]) {
                            let start_x = find_start(test_x, test_y, grid);
                            let end_x = find_end(test_x, test_y, grid);
                            let (number, _) = parse_number(start_x, test_y, grid);
                            numbers.push(number);
                            for remove_x in start_x..=end_x {
                                if let Some(pos) = adjacent
                                    .iter()
                                    .position(|coord| coord.0 == remove_x && coord.1 == test_y)
                                {
                                    adjacent.remove(pos);
                                }
                            }
                        }
                    }
                    if numbers.len() == 2 {
                        sum += numbers.pop().unwrap() * numbers.pop().unwrap();
                    }
                }
            }
        }
    }
    println!("part2: {}", sum)
}

fn main() {
    let mut grid: [[char; 140]; 140] = [['.'; 140]; 140];
    let mut x = 0;
    let mut y = 0;
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    for line in input.lines() {
        for char in line.chars() {
            grid[y][x] = char;
            x += 1;
        }
        x = 0;
        y += 1;
    }
    part1(&grid);
    part2(&grid);
}
