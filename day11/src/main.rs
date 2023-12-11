use std::fs;

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<char>>,
    empty_rows: Vec<usize>,
    empty_columns: Vec<usize>,
    galaxies: Vec<(usize, usize)>,
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
            galaxies: Vec::new(),
            empty_rows: Vec::new(),
            empty_columns: Vec::new(),
        }
    }
}

impl Grid {
    fn find_empty_space(&mut self) {
        for (y, row) in self.grid.iter().enumerate() {
            if row.iter().all(|&c| c == '.') {
                self.empty_rows.push(y);
            }
        }
        for x in 0..self.grid[0].len() {
            let mut empty = true;
            for y in 0..self.grid.len() {
                if self.grid[y][x] == '#' {
                    empty = false;
                }
            }
            if empty {
                self.empty_columns.push(x);
            }
        }
    }
    fn expand(&mut self) {
        let mut new_grid = self.grid.clone();
        self.find_empty_space();
        for (index, &x) in self.empty_columns.iter().enumerate() {
            for y in 0..new_grid.len() {
                new_grid[y].insert(x + index, '.');
            }
        }

        for (index, &y) in self.empty_rows.iter().enumerate() {
            new_grid.insert(y + index, vec!['.'; new_grid[y].len()])
        }
        self.grid = new_grid;
    }

    fn pinpoint_galaxies(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                if self.grid[y][x] == '#' {
                    self.galaxies.push((x, y));
                }
            }
        }
    }

    fn sum_distances(&self) -> usize {
        let mut sum = 0;
        for (idx, &(x1, y1)) in self.galaxies.iter().enumerate() {
            for i in idx + 1..self.galaxies.len() {
                let (x2, y2) = self.galaxies[i];
                sum +=
                    ((y2 as isize - y1 as isize).abs() + (x2 as isize - x1 as isize).abs()) as usize
            }
        }
        sum
    }

    fn sum_giant_distances(&self) -> usize {
        let mut sum = 0;
        for (idx, &(x1, y1)) in self.galaxies.iter().enumerate() {
            let x1_offset = self
                .empty_columns
                .iter()
                .filter(|&&col| col < x1)
                .collect::<Vec<&usize>>()
                .len()
                * 999999;
            let y1_offset = self
                .empty_rows
                .iter()
                .filter(|&&col| col < y1)
                .collect::<Vec<&usize>>()
                .len()
                * 999999;
            for i in idx + 1..self.galaxies.len() {
                let (x2, y2) = self.galaxies[i];
                let x2_offset = self
                    .empty_columns
                    .iter()
                    .filter(|&&col| col < x2)
                    .collect::<Vec<&usize>>()
                    .len()
                    * 999999;
                let y2_offset = self
                    .empty_rows
                    .iter()
                    .filter(|&&col| col < y2)
                    .collect::<Vec<&usize>>()
                    .len()
                    * 999999;
                sum += (((y2 + y2_offset) as isize - (y1 + y1_offset) as isize).abs()
                    + ((x2 + x2_offset) as isize - (x1 + x1_offset) as isize).abs())
                    as usize
            }
        }
        sum
    }
}

fn part1(mut grid: Grid) {
    grid.expand();
    grid.pinpoint_galaxies();
    println!("part1: {}", grid.sum_distances())
}

fn part2(mut grid: Grid) {
    grid.find_empty_space();
    grid.pinpoint_galaxies();
    println!("part2: {}", grid.sum_giant_distances())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let grid = Grid::from(&input);
    part1(grid.clone());
    part2(grid.clone());
}
