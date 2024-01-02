use std::fs;

use rgeometry::data::{Line, Point};
use z3::ast::{Ast, Int};
use z3::{Config, Context, SatResult::*, Solver};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
struct Snowball {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl From<&str> for Snowball {
    fn from(value: &str) -> Self {
        let mut splits = value.split(" @ ");
        let positions = splits
            .next()
            .unwrap()
            .split(", ")
            .map(|item| item.trim().parse().unwrap())
            .collect::<Vec<f64>>();
        let velocities = splits
            .next()
            .unwrap()
            .split(", ")
            .map(|item| item.trim().parse().unwrap())
            .collect::<Vec<f64>>();
        let (px, py, pz) = (positions[0], positions[1], positions[2]);
        let (vx, vy, vz) = (velocities[0], velocities[1], velocities[2]);
        Snowball {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
struct Snowballs {
    snowballs: Vec<Snowball>,
    min: f64,
    max: f64,
}

impl From<&String> for Snowballs {
    fn from(value: &String) -> Self {
        let mut snowballs = Vec::new();
        for line in value.lines() {
            snowballs.push(Snowball::from(line));
        }
        Snowballs {
            snowballs,
            min: 0f64,
            max: 0f64,
        }
    }
}

fn has_same_sign(a: f64, b: f64) -> bool {
    (a < 0f64 && b < 0f64) || (a > 0f64 && b > 0f64)
}

fn in_past(intersection: &Point<f64>, a: &Snowball, b: &Snowball) -> bool {
    let mut retval = false;
    let &x = intersection.x_coord();
    let &y = intersection.y_coord();
    let tests = [
        ((x - a.px, y - a.py), (a.vx, a.vy)),
        ((x - b.px, y - b.py), (b.vz, b.vy)),
    ];
    for test in tests {
        if !has_same_sign(test.0 .0, test.1 .0) || !has_same_sign(test.0 .1, test.1 .1) {
            retval = true;
            break;
        }
    }
    retval
}

impl Snowballs {
    fn intersections(&self) -> usize {
        let mut intersections = 0;
        for i in 0..self.snowballs.len() {
            let p1 = Point::<f64, 2>::new([self.snowballs[i].px, self.snowballs[i].py]);
            let p2 = Point::<f64, 2>::new([
                self.snowballs[i].px + self.snowballs[i].vx,
                self.snowballs[i].py + self.snowballs[i].vy,
            ]);
            let test = Line::new_through(&p1, &p2);
            for j in i + 1..self.snowballs.len() {
                let p1 = Point::<f64, 2>::new([self.snowballs[j].px, self.snowballs[j].py]);
                let p2 = Point::<f64, 2>::new([
                    self.snowballs[j].px + self.snowballs[j].vx,
                    self.snowballs[j].py + self.snowballs[j].vy,
                ]);
                let check = Line::new_through(&p1, &p2);
                if let Some(point) = test.intersection_point(&check) {
                    let &x = point.x_coord();
                    let &y = point.y_coord();
                    if self.min <= x
                        && x <= self.max
                        && self.min <= y
                        && y <= self.max
                        && !in_past(&point, &self.snowballs[i], &self.snowballs[j])
                    {
                        intersections += 1;
                    }
                }
            }
        }
        intersections
    }
}

fn part1(snowballs: &Snowballs) -> usize {
    snowballs.intersections()
}

#[derive(Debug)]
struct Z3Snowball<'a> {
    px: Int<'a>,
    py: Int<'a>,
    pz: Int<'a>,
    vx: Int<'a>,
    vy: Int<'a>,
    vz: Int<'a>,
}

impl Snowball {
    fn to_z3<'a>(&self, ctx: &'a Context) -> Z3Snowball<'a> {
        Z3Snowball {
            px: Int::from_i64(ctx, self.px as i64),
            py: Int::from_i64(ctx, self.py as i64),
            pz: Int::from_i64(ctx, self.pz as i64),
            vx: Int::from_i64(ctx, self.vx as i64),
            vy: Int::from_i64(ctx, self.vy as i64),
            vz: Int::from_i64(ctx, self.vz as i64),
        }
    }
}

fn part2(snowballs: &Snowballs) -> usize {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let rock_px = Int::new_const(&ctx, "rock_px");
    let rock_py = Int::new_const(&ctx, "rock_py");
    let rock_pz = Int::new_const(&ctx, "rock_pz");
    let rock_vx = Int::new_const(&ctx, "rock_vx");
    let rock_vy = Int::new_const(&ctx, "rock_vy");
    let rock_vz = Int::new_const(&ctx, "rock_vz");
    let zero = Int::from_i64(&ctx, 0);

    let selected = snowballs.snowballs.iter().take(3);

    for (i, hail) in (0..).zip(selected.map(|h| h.to_z3(&ctx))) {
        let t = Int::new_const(&ctx, format!("t{}", i));
        solver.assert(&t.gt(&zero));
        solver.assert(&(&rock_px + &rock_vx * &t)._eq(&(hail.px + hail.vx * &t)));
        solver.assert(&(&rock_py + &rock_vy * &t)._eq(&(hail.py + hail.vy * &t)));
        solver.assert(&(&rock_pz + &rock_vz * &t)._eq(&(hail.pz + hail.vz * &t)));
    }
    let res = if let (Sat, Some(model)) = (solver.check(), solver.get_model()) {
        model
            .eval(&(rock_px + rock_py + rock_pz), true)
            .unwrap()
            .as_u64()
            .unwrap() as usize
    } else {
        0
    };
    res
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut snowballs = Snowballs::from(&input);
    snowballs.min = 200000000000000f64;
    snowballs.max = 400000000000000f64;
    println!("{}", part1(&snowballs)); //12015
    println!("{}", part2(&snowballs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let mut snowballs = Snowballs::from(&input);
        snowballs.min = 7f64;
        snowballs.max = 27f64;
        assert_eq!(2, part1(&snowballs));
    }

    #[test]
    fn test_p2() {
        let input = fs::read_to_string("test.txt").expect("test.txt should exist");
        let snowballs = Snowballs::from(&input);
        assert_eq!(47, part2(&snowballs));
    }
}
