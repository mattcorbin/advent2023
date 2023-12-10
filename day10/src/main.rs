use std::collections::HashSet;
use std::fs;

struct Grid {
    grid: Vec<Vec<char>>,
    tunnel: HashSet<(usize, usize)>,
}

impl From<&String> for Grid {
    fn from(value: &String) -> Self {
        let mut grid = Vec::new();
        for line in value.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            grid.push(row)
        }
        Grid {
            grid,
            tunnel: HashSet::new(),
        }
    }
}

impl Grid {
    fn find_start(&self) -> (usize, usize) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == 'S' {
                    return (x, y);
                }
            }
        }
        (0, 0)
    }

    fn find_connected_to_start(&self, x: usize, y: usize) -> ((usize, usize), (usize, usize)) {
        let mut connected = Vec::new();
        if vec!['-', 'J', '7'].contains(&self.grid[y][x + 1]) {
            connected.push((x + 1, y));
        }
        if vec!['-', 'L', 'F'].contains(&self.grid[y][x - 1]) {
            connected.push((x - 1, y));
        }
        if vec!['|', '7', 'F'].contains(&self.grid[y - 1][x]) {
            connected.push((x, y - 1));
        }
        if vec!['|', 'L', 'J'].contains(&self.grid[y + 1][x]) {
            connected.push((x, y + 1));
        }
        (connected[0], connected[1])
    }

    fn connections(&self, x: usize, y: usize) -> ((usize, usize), (usize, usize)) {
        match self.grid[y][x] {
            '|' => ((x, y - 1), (x, y + 1)),
            '-' => ((x - 1, y), (x + 1, y)),
            'L' => ((x + 1, y), (x, y - 1)),
            'J' => ((x - 1, y), (x, y - 1)),
            '7' => ((x - 1, y), (x, y + 1)),
            'F' => ((x + 1, y), (x, y + 1)),
            _ => panic!("wtf"),
        }
    }

    fn build_tunnel(&mut self) {
        let (start_x, start_y) = self.find_start();
        self.tunnel.insert((start_x, start_y));

        let mut next = Vec::new();
        let (a, b) = self.find_connected_to_start(start_x, start_y);
        next.push(a);
        next.push(b);
        while let Some((current_x, current_y)) = next.pop() {
            self.tunnel.insert((current_x, current_y));
            let (a, b) = self.connections(current_x, current_y);
            if self.tunnel.contains(&a) && self.tunnel.contains(&b) {
                break;
            } else if self.tunnel.contains(&a) {
                next.push(b);
            } else {
                next.push(a);
            }
        }
    }

    fn only_loop(&self) -> Grid {
        let max_y = self.grid.len();
        let max_x = self.grid[0].len();
        let mut retval = Grid {
            grid: vec![vec!['.'; max_x]; max_y],
            tunnel: self.tunnel.clone(),
        };
        for &(x, y) in self.tunnel.iter() {
            retval.grid[y][x] = self.grid[y][x];
        }
        retval
    }

    fn expand(&mut self) {
        let max_y = self.grid.len() * 2;
        let max_x = self.grid[0].len() * 2;
        let mut new_grid = vec![vec!['.'; max_x]; max_y];
        for y in (1..new_grid.len()).step_by(2) {
            for x in (1..new_grid[y].len()).step_by(2) {
                new_grid[y][x] = self.grid[y / 2][x / 2];
                match self.grid[y / 2][x / 2] {
                    '|' => {
                        new_grid[y + 1][x] = '|';
                        new_grid[y - 1][x] = '|'
                    }
                    '-' => {
                        new_grid[y][x + 1] = '-';
                        new_grid[y][x - 1] = '-';
                    }
                    '7' => {
                        new_grid[y + 1][x] = '|';
                        new_grid[y][x - 1] = '-';
                    }
                    'L' => {
                        new_grid[y - 1][x] = '|';
                        new_grid[y][x + 1] = '-';
                    }
                    'F' => {
                        new_grid[y + 1][x] = '|';
                        new_grid[y][x + 1] = '-';
                    }
                    'J' => {
                        new_grid[y - 1][x] = '|';
                        new_grid[y][x - 1] = '-';
                    }
                    _ => (),
                };
            }
        }
        self.grid = new_grid;
    }

    fn categorize(&mut self) {
        self.grid[0][0] = '0';
        let mut changed = true;
        while changed {
            changed = false;
            for y in 0..self.grid.len() {
                for x in 0..self.grid[y].len() {
                    if self.grid[y][x] == '0' {
                        if !(x + 1 == self.grid[y].len()) && self.grid[y][x + 1] == '.' {
                            self.grid[y][x + 1] = '0';
                            changed = true;
                        }
                        if !(y + 1 == self.grid.len()) && self.grid[y + 1][x] == '.' {
                            self.grid[y + 1][x] = '0';
                            changed = true;
                        }
                    }
                }
            }
            for y in (0..self.grid.len()).rev() {
                for x in (0..self.grid[y].len()).rev() {
                    if self.grid[y][x] == '0' {
                        if !(x == 0) && self.grid[y][x - 1] == '.' {
                            self.grid[y][x - 1] = '0';
                            changed = true;
                        }
                        if !(y == 0) && self.grid[y - 1][x] == '.' {
                            self.grid[y - 1][x] = '0';
                            changed = true;
                        }
                    }
                }
            }
        }
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                if self.grid[y][x] == '.' {
                    self.grid[y][x] = 'I'
                }
            }
        }
    }

    fn shrink(&mut self) {
        let max_y = self.grid.len() / 2;
        let max_x = self.grid[0].len() / 2;
        let mut new_grid = vec![vec!['.'; max_x]; max_y];
        for y in (1..self.grid.len()).step_by(2) {
            for x in (1..self.grid[y].len()).step_by(2) {
                new_grid[y / 2][x / 2] = self.grid[y][x];
            }
        }
        self.grid = new_grid;
    }

    fn count_enclosed(&self) -> usize {
        let mut count = 0;
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == 'I' {
                    count += 1;
                }
            }
        }
        count
    }
}

fn part1(input: &Grid) {
    println!("part1: {}", input.tunnel.len() / 2);
}

fn part2(input: &Grid) {
    let mut only_loop = input.only_loop();
    only_loop.expand();
    only_loop.categorize();
    only_loop.shrink();
    println!("part2: {}", only_loop.count_enclosed())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut grid = Grid::from(&input);
    grid.build_tunnel();
    part1(&grid);
    part2(&grid);
}
