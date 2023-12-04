use std::fs;
use std::str::FromStr;

#[derive(Clone)]
struct Card {
    id: usize,
    copies: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(": ");
        let id = splits
            .next()
            .unwrap()
            .replace("Card", "")
            .replace(" ", "")
            .parse()
            .unwrap();
        let mut splits = splits.next().unwrap().split(" | ");
        let winning_numbers = splits
            .next()
            .unwrap()
            .trim_start()
            .trim_end()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.replace(" ", "").parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let my_numbers = splits
            .next()
            .unwrap()
            .trim_start()
            .trim_end()
            .replace("  ", " ")
            .split(" ")
            .map(|x| x.replace(" ", "").parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Ok(Card {
            id,
            copies: 1,
            winning_numbers,
            my_numbers,
        })
    }
}

impl Card {
    fn get_value(&self) -> usize {
        let mut val = 0;
        for num in &self.winning_numbers {
            if self.my_numbers.contains(num) {
                if val == 0 {
                    val = 1;
                } else {
                    val *= 2;
                }
            }
        }
        val
    }

    fn num_matches(&self) -> usize {
        let mut val = 0;
        for num in &self.winning_numbers {
            if self.my_numbers.contains(num) {
                val += 1;
            }
        }
        val
    }
}

fn part1(input: &str) {
    let mut sum = 0;
    let mut cards = Vec::new();
    for line in input.lines() {
        cards.push(Card::from_str(line).expect("card parses"));
    }
    for card in cards {
        sum += card.get_value();
    }
    println!("part1: {}", sum)
}

fn part2(input: &str) {
    let mut cards = Vec::new();
    for line in input.lines() {
        cards.push(Card::from_str(line).expect("card parses"));
    }
    let mut i = 0;
    while i < cards.len() {
        let matches = cards[i].num_matches();
        for j in 1..=matches {
            if i + j >= cards.len() {
                break;
            }
            cards[i + j].copies += cards[i].copies
        }
        i += 1;
    }
    println!("part2: {}", cards.iter().map(|x| x.copies).sum::<usize>())
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    part1(&input);
    part2(&input);
}
