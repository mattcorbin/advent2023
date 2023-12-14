use std::fs;

fn load(grid: &Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    let reversed = grid.clone().into_iter().rev().collect::<Vec<Vec<char>>>();
    for (idx, row) in reversed.into_iter().enumerate() {
        for rock in row {
            if rock == 'O' {
                sum += idx + 1
            }
        }
    }
    sum
}

fn tilt(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut retval = grid.clone();

    for y in 0..retval.len() {
        if y == 0 {
            continue;
        }
        for x in 0..retval[y].len() {
            if retval[y][x] == 'O' {
                let mut local = y as isize - 1;
                while local >= 0 && retval[local as usize][x] == '.' {
                    retval[local as usize][x] = 'O';
                    retval[local as usize + 1][x] = '.';
                    local -= 1;
                }
            }
        }
    }

    retval
}

fn spin_inner(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut retval = grid.clone();

    // North
    for y in 0..retval.len() {
        if y == 0 {
            continue;
        }
        for x in 0..retval[y].len() {
            if retval[y][x] == 'O' {
                let mut local = y as isize - 1;
                while local >= 0 && retval[local as usize][x] == '.' {
                    retval[local as usize][x] = 'O';
                    retval[local as usize + 1][x] = '.';
                    local -= 1;
                }
            }
        }
    }
    // West
    for x in 0..retval[0].len() {
        if x == 0 {
            continue;
        }
        for y in 0..retval.len() {
            if retval[y][x] == 'O' {
                let mut local = x as isize - 1;
                while local >= 0 && retval[y][local as usize] == '.' {
                    retval[y][local as usize] = 'O';
                    retval[y][local as usize + 1] = '.';
                    local -= 1;
                }
            }
        }
    }

    // South
    for y in (0..retval.len()).rev() {
        if y == (retval.len() - 1) {
            continue;
        }
        for x in 0..retval[y].len() {
            if retval[y][x] == 'O' {
                let mut local = y + 1;
                while local < retval.len() && retval[local][x] == '.' {
                    retval[local][x] = 'O';
                    retval[local - 1][x] = '.';
                    local += 1;
                }
            }
        }
    }

    // East
    for x in (0..retval[0].len()).rev() {
        if x == (retval[0].len() - 1) {
            continue;
        }
        for y in 0..retval.len() {
            if retval[y][x] == 'O' {
                let mut local = x + 1;
                while local < retval[y].len() && retval[y][local] == '.' {
                    retval[y][local] = 'O';
                    retval[y][local - 1] = '.';
                    local += 1;
                }
            }
        }
    }

    retval
}

fn spin(grid: &Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut retval = grid.clone();
    let mut states = Vec::new();
    let mut cycle_start_index = 0;

    for _ in 0..cycles {
        states.push(retval.clone());
        retval = spin_inner(retval);
        if states.contains(&retval) {
            cycle_start_index = states
                .iter()
                .position(|x| x.clone() == retval)
                .expect("exists");
            break;
        }
    }

    states[cycle_start_index..][(cycles - cycle_start_index) % (states.len() - cycle_start_index)]
        .clone()
}

fn part1(grid: &Vec<Vec<char>>) {
    println!("part1: {}", load(&tilt(grid)))
}

fn part2(grid: &Vec<Vec<char>>) {
    println!("part2: {}", load(&spin(grid, 1000000000)))
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut grid = Vec::new();
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    part1(&grid);
    part2(&grid);
}
