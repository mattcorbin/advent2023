use std::cmp::min;
use std::fs;
use std::str::FromStr;

use rayon::prelude::*;

struct Maps {
    seeds: Vec<u64>,
    seeds_to_soil: Vec<(u64, u64, u64)>,
    soil_to_fertilizer: Vec<(u64, u64, u64)>,
    fertilizer_to_water: Vec<(u64, u64, u64)>,
    water_to_light: Vec<(u64, u64, u64)>,
    light_to_temperature: Vec<(u64, u64, u64)>,
    temperature_to_humidity: Vec<(u64, u64, u64)>,
    humidity_to_location: Vec<(u64, u64, u64)>,
}

impl FromStr for Maps {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seeds = Vec::new();
        let mut seeds_to_soil = Vec::new();
        let mut soil_to_fertilizer = Vec::new();
        let mut fertilizer_to_water = Vec::new();
        let mut water_to_light = Vec::new();
        let mut light_to_temperature = Vec::new();
        let mut temperature_to_humidity = Vec::new();
        let mut humidity_to_location = Vec::new();
        let mut map = 0;
        for line in s.lines() {
            if line.is_empty() {
                continue;
            } else if line.starts_with("seeds: ") {
                seeds = line
                    .trim_start_matches("seeds: ")
                    .split(" ")
                    .map(|x| x.parse::<u64>().expect("unable to parse seed"))
                    .collect();
            } else if line.contains("map") {
                map += 1;
            } else {
                let nums: Vec<u64> = line
                    .split(" ")
                    .map(|x| x.parse::<u64>().expect("unable to parse almanac"))
                    .collect();
                let (dst, src, range) = (nums[0], nums[1], nums[2]);
                match map {
                    1 => {
                        seeds_to_soil.push((dst, src, range));
                    }
                    2 => {
                        soil_to_fertilizer.push((dst, src, range));
                    }
                    3 => {
                        fertilizer_to_water.push((dst, src, range));
                    }
                    4 => {
                        water_to_light.push((dst, src, range));
                    }
                    5 => {
                        light_to_temperature.push((dst, src, range));
                    }
                    6 => {
                        temperature_to_humidity.push((dst, src, range));
                    }
                    7 => {
                        humidity_to_location.push((dst, src, range));
                    }
                    _ => panic!("wtf"),
                }
            }
        }
        Ok(Maps {
            seeds,
            seeds_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Maps {
    fn find_location(&self, seed: u64) -> u64 {
        let mut soil = seed;
        for &(dst, src, range) in &self.seeds_to_soil {
            if seed >= src && seed < src + range {
                soil = dst + (seed - src);
            }
        }
        let mut fertilizer = soil;
        for &(dst, src, range) in &self.soil_to_fertilizer {
            if soil >= src && soil < src + range {
                fertilizer = dst + (soil - src);
            }
        }
        let mut water = fertilizer;
        for &(dst, src, range) in &self.fertilizer_to_water {
            if fertilizer >= src && fertilizer < src + range {
                water = dst + (fertilizer - src);
            }
        }
        let mut light = water;
        for &(dst, src, range) in &self.water_to_light {
            if water >= src && water < src + range {
                light = dst + (water - src);
            }
        }
        let mut temperature = light;
        for &(dst, src, range) in &self.light_to_temperature {
            if light >= src && light < src + range {
                temperature = dst + (light - src);
            }
        }
        let mut humidity = temperature;
        for &(dst, src, range) in &self.temperature_to_humidity {
            if temperature >= src && temperature < src + range {
                humidity = dst + (temperature - src);
            }
        }
        let mut location = humidity;
        for &(dst, src, range) in &self.humidity_to_location {
            if humidity >= src && humidity < src + range {
                location = dst + (humidity - src);
            }
        }
        location
    }
}

fn part1(maps: &Maps) {
    let mut res = u64::MAX;
    for &seed in &maps.seeds {
        res = min(res, maps.find_location(seed))
    }
    println!("part1: {}", res)
}

fn part2(maps: &Maps) {
    let mut res = u64::MAX;
    let mut seed_ranges = Vec::new();
    for i in 0..maps.seeds.len() {
        if i % 2 == 0 {
            continue;
        } else {
            seed_ranges.push((maps.seeds[i - 1], maps.seeds[i]))
        }
    }
    for (start, range) in seed_ranges {
        res = min(
            res,
            (start..start + range)
                .into_par_iter()
                .map(|x| maps.find_location(x))
                .min()
                .expect("wtf"),
        )
    }
    println!("part2: {}", res)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let maps = Maps::from_str(&input).unwrap();
    part1(&maps);
    part2(&maps);
}
