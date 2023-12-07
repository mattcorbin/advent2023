use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should always ord")
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("wtf"),
        }
    }
}

impl From<JokerCard> for Card {
    fn from(value: JokerCard) -> Self {
        match value {
            JokerCard::Ace => Card::Ace,
            JokerCard::King => Card::King,
            JokerCard::Queen => Card::Queen,
            JokerCard::Joker => panic!("this is wrong"),
            JokerCard::Ten => Card::Ten,
            JokerCard::Nine => Card::Nine,
            JokerCard::Eight => Card::Eight,
            JokerCard::Seven => Card::Seven,
            JokerCard::Six => Card::Six,
            JokerCard::Five => Card::Five,
            JokerCard::Four => Card::Four,
            JokerCard::Three => Card::Three,
            JokerCard::Two => Card::Two,
        }
    }
}

impl Card {
    fn value(&self) -> usize {
        match self {
            Card::Ace => 12,
            Card::King => 11,
            Card::Queen => 10,
            Card::Jack => 9,
            Card::Ten => 8,
            Card::Nine => 7,
            Card::Eight => 6,
            Card::Seven => 5,
            Card::Six => 4,
            Card::Five => 3,
            Card::Four => 2,
            Card::Three => 1,
            Card::Two => 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd<Self> for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self {
            HandType::FiveOfAKind => match other {
                HandType::FiveOfAKind => Ordering::Equal,
                HandType::FourOfAKind => Ordering::Greater,
                HandType::FullHouse => Ordering::Greater,
                HandType::ThreeOfAKind => Ordering::Greater,
                HandType::TwoPair => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::FourOfAKind => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Equal,
                HandType::FullHouse => Ordering::Greater,
                HandType::ThreeOfAKind => Ordering::Greater,
                HandType::TwoPair => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::FullHouse => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Less,
                HandType::FullHouse => Ordering::Equal,
                HandType::ThreeOfAKind => Ordering::Greater,
                HandType::TwoPair => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::ThreeOfAKind => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Less,
                HandType::FullHouse => Ordering::Less,
                HandType::ThreeOfAKind => Ordering::Equal,
                HandType::TwoPair => Ordering::Greater,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::TwoPair => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Less,
                HandType::FullHouse => Ordering::Less,
                HandType::ThreeOfAKind => Ordering::Less,
                HandType::TwoPair => Ordering::Equal,
                HandType::OnePair => Ordering::Greater,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::OnePair => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Less,
                HandType::FullHouse => Ordering::Less,
                HandType::ThreeOfAKind => Ordering::Less,
                HandType::TwoPair => Ordering::Less,
                HandType::OnePair => Ordering::Equal,
                HandType::HighCard => Ordering::Greater,
            },
            HandType::HighCard => match other {
                HandType::FiveOfAKind => Ordering::Less,
                HandType::FourOfAKind => Ordering::Less,
                HandType::FullHouse => Ordering::Less,
                HandType::ThreeOfAKind => Ordering::Less,
                HandType::TwoPair => Ordering::Less,
                HandType::OnePair => Ordering::Less,
                HandType::HighCard => Ordering::Equal,
            },
        })
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should always cmp")
    }
}

impl From<Vec<Card>> for HandType {
    fn from(value: Vec<Card>) -> Self {
        let mut cards_map: HashMap<Card, usize> = HashMap::new();
        for card in &value {
            if let Some(&count) = cards_map.get(&card) {
                cards_map.insert(*card, count + 1);
            } else {
                cards_map.insert(*card, 1);
            }
        }
        let values: Vec<usize> = cards_map.iter().map(|(_, &y)| y).collect();
        let hand_type;
        if values[0] == 5 {
            hand_type = HandType::FiveOfAKind;
        } else if values[0] == 4 || values[1] == 4 {
            hand_type = HandType::FourOfAKind;
        } else if (values[0] == 3 && values[1] == 2) || (values[1] == 3 && values[0] == 2) {
            hand_type = HandType::FullHouse;
        } else if values[0] == 3 || values[1] == 3 || values[2] == 3 {
            hand_type = HandType::ThreeOfAKind;
        } else if (values[0] == 2 && values[1] == 2)
            || (values[0] == 2 && values[2] == 2)
            || (values[1] == 2 && values[2] == 2)
        {
            hand_type = HandType::TwoPair;
        } else if values[0] == 2 || values[1] == 2 || values[2] == 2 || values[3] == 2 {
            hand_type = HandType::OnePair;
        } else {
            hand_type = HandType::HighCard;
        }
        hand_type
    }
}

#[derive(PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<Card>,
    bid: usize,
    hand_type: HandType,
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");
        let cards: Vec<Card> = splits
            .next()
            .unwrap()
            .chars()
            .map(|x| Card::from(x))
            .collect();
        let bid = splits.next().unwrap().parse().expect("should be a number");
        let hand_type = HandType::from(cards.clone());
        Ok(Hand {
            cards,
            bid,
            hand_type,
        })
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let mut order = Ordering::Equal;
                for i in 0..self.cards.len() {
                    order = self.cards[i].cmp(&other.cards[i]);
                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            }
            Ordering::Greater => Ordering::Greater,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should always cmp")
    }
}

fn part1(input: &Vec<Hand>) {
    let mut winnings = 0;
    let mut sorted_hands = input.clone();
    sorted_hands.sort();
    for (idx, hand) in sorted_hands.into_iter().enumerate() {
        winnings += (idx + 1) * hand.bid;
    }
    println!("part1: {}", winnings)
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum JokerCard {
    Ace,
    King,
    Queen,
    Joker,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl PartialOrd<Self> for JokerCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for JokerCard {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should always ord")
    }
}

impl From<char> for JokerCard {
    fn from(value: char) -> Self {
        match value {
            'A' => JokerCard::Ace,
            'K' => JokerCard::King,
            'Q' => JokerCard::Queen,
            'J' => JokerCard::Joker,
            'T' => JokerCard::Ten,
            '9' => JokerCard::Nine,
            '8' => JokerCard::Eight,
            '7' => JokerCard::Seven,
            '6' => JokerCard::Six,
            '5' => JokerCard::Five,
            '4' => JokerCard::Four,
            '3' => JokerCard::Three,
            '2' => JokerCard::Two,
            _ => panic!("wtf"),
        }
    }
}

impl JokerCard {
    fn value(&self) -> usize {
        match self {
            JokerCard::Ace => 12,
            JokerCard::King => 11,
            JokerCard::Queen => 10,
            JokerCard::Ten => 9,
            JokerCard::Nine => 8,
            JokerCard::Eight => 7,
            JokerCard::Seven => 6,
            JokerCard::Six => 5,
            JokerCard::Five => 4,
            JokerCard::Four => 3,
            JokerCard::Three => 2,
            JokerCard::Two => 1,
            JokerCard::Joker => 0,
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
struct JokerHand {
    cards: Vec<JokerCard>,
    bid: usize,
    hand_type: HandType,
}

impl FromStr for JokerHand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split(" ");
        let cards: Vec<JokerCard> = splits
            .next()
            .unwrap()
            .chars()
            .map(|x| JokerCard::from(x))
            .collect();
        let bid = splits.next().unwrap().parse().expect("should be a number");
        let hand_type = HandType::from(cards.clone());
        Ok(JokerHand {
            cards,
            bid,
            hand_type,
        })
    }
}

impl PartialOrd<Self> for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let mut order = Ordering::Equal;
                for i in 0..self.cards.len() {
                    order = self.cards[i].cmp(&other.cards[i]);
                    if order != Ordering::Equal {
                        break;
                    }
                }
                order
            }
            Ordering::Greater => Ordering::Greater,
        })
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("should always cmp")
    }
}

const JOKER_CARDS: [Card; 12] = [
    Card::Ace,
    Card::King,
    Card::Queen,
    Card::Ten,
    Card::Nine,
    Card::Eight,
    Card::Seven,
    Card::Six,
    Card::Five,
    Card::Four,
    Card::Three,
    Card::Two,
];

impl From<Vec<JokerCard>> for HandType {
    fn from(value: Vec<JokerCard>) -> Self {
        if !value.iter().any(|card| *card == JokerCard::Joker) {
            let cards: Vec<Card> = value.iter().map(|card| Card::from(*card)).collect();
            return HandType::from(cards);
        }
        let non_jokers: Vec<Card> = value
            .iter()
            .filter(|card| !(**card == JokerCard::Joker))
            .map(|card| Card::from(*card))
            .collect();
        let cards_to_add = 5 - non_jokers.len();
        let mut best_hand_type = HandType::HighCard;
        let combinations = JOKER_CARDS
            .into_iter()
            .combinations_with_replacement(cards_to_add);
        for combination in combinations {
            let mut test = non_jokers.clone();
            test.append(&mut combination.clone());
            let new_hand_type = HandType::from(test);
            if best_hand_type < new_hand_type {
                best_hand_type = new_hand_type
            }
        }
        best_hand_type
    }
}

fn part2(input: &Vec<JokerHand>) {
    let mut winnings = 0;
    let mut sorted_hands = input.clone();
    sorted_hands.sort();
    for (idx, hand) in sorted_hands.into_iter().enumerate() {
        winnings += (idx + 1) * hand.bid;
    }
    println!("part2: {}", winnings)
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("input.txt should exist");
    let mut hands = Vec::new();
    let mut joker_hands = Vec::new();
    for line in input.lines() {
        hands.push(Hand::from_str(line).unwrap());
        joker_hands.push(JokerHand::from_str(line).unwrap());
    }
    part1(&hands);
    part2(&joker_hands);
}
