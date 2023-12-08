use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    // let mut res = step1(&content);
    // println!("{res}");
    let res = step2(&content);
    println!("{res}");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Card {
    symbol: String,
}

impl Card {
    pub fn new(symbol: char) -> Self {
        Self {
            symbol: String::from(symbol),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.symbol.parse::<u64>(), other.symbol.parse::<u64>()) {
            (Ok(x), Ok(y)) => x.partial_cmp(&y),
            (Ok(_), Err(_)) => {
                if other.symbol == "J" {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Less)
                }
            }
            (Err(_), Ok(_)) => {
                if self.symbol == "J" {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Greater)
                }
            }
            (Err(_), Err(_)) => {
                let order;
                if self.symbol == other.symbol {
                    order = Ordering::Equal;
                } else {
                    order = match (self.symbol.as_str(), other.symbol.as_str()) {
                        ("J", _) => Ordering::Less,
                        ("T", x) if ["Q", "K", "A"].contains(&x) => Ordering::Less,
                        ("Q", x) if ["K", "A"].contains(&x) => Ordering::Less,
                        ("K", "A") => Ordering::Less,
                        _ => Ordering::Greater,
                    };
                }
                return Some(order);
            }
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum HandKind {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

impl PartialOrd for HandKind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = *self as u32;
        let o = *other as u32;
        s.partial_cmp(&o)
    }
}

#[derive(PartialEq)]
struct Hand {
    cards: Vec<Card>,
}

impl std::fmt::Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in self.cards.iter() {
            let _ = f.write_str(&format!("{}", c.symbol));
        }
        let _ = f.write_str(&format!(" ({:?})", self.kind()));
        Ok(())
    }
}

impl Hand {
    pub fn new(cards_str: &str) -> Self {
        let mut cards = vec![];
        for c in cards_str.chars() {
            cards.push(Card::new(c));
        }
        Self { cards }
    }

    pub fn kind(&self) -> HandKind {
        let mut map: HashMap<&Card, usize> = HashMap::new();
        let mut jokers = 0;
        for i in 0..5 {
            let card = self.cards.get(i).unwrap();
            if &card.symbol == "J" {
                jokers += 1;
            } else {
                *map.entry(card).or_default() += 1;
            }
        }

        let counts: Vec<usize> = map.values().map(|&c| c).collect();
        let mut max = *counts.iter().max().unwrap_or(&0);
        max += jokers;
        match max {
            5 => HandKind::Five,
            4 => HandKind::Four,
            3 => {
                if counts.len() == 2 {
                    HandKind::Full
                } else {
                    HandKind::Three
                }
            }
            2 => {
                if counts.len() == 3 {
                    HandKind::Two
                } else {
                    HandKind::One
                }
            }
            _ => HandKind::High,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_kind = self.kind();
        let other_kind = other.kind();

        if self_kind == other_kind {
            for i in 0..5 {
                let self_card = self.cards.get(i).unwrap();
                let other_card = other.cards.get(i).unwrap();
                if self_card != other_card {
                    return self_card.partial_cmp(other_card);
                }
            }
            Some(Ordering::Equal)
        } else {
            self_kind.partial_cmp(&other_kind)
        }
    }
}

fn step2(content: &str) -> u64 {
    // First: collect hands and bids
    let mut hands = vec![];
    for line in content.split('\n') {
        let parts = line.split(' ').collect::<Vec<&str>>();
        hands.push((
            Hand::new(parts.first().unwrap()),
            parts.get(1).unwrap().parse::<u64>().unwrap(),
        ));
    }
    hands.sort_by(|one, other| one.0.partial_cmp(&other.0).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .reduce(|a, b| a + b)
        .unwrap()
    // 5905
}

#[allow(dead_code)]
fn step1(content: &str) -> u64 {
    // First: collect hands and bids
    let mut hands = vec![];
    for line in content.split('\n') {
        let parts = line.split(' ').collect::<Vec<&str>>();
        hands.push((
            Hand::new(parts.first().unwrap()),
            parts.get(1).unwrap().parse::<u64>().unwrap(),
        ));
    }
    hands.sort_by(|one, other| one.0.partial_cmp(&other.0).unwrap());
    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .reduce(|a, b| a + b)
        .unwrap()
    // 6440
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{Card, Hand, HandKind};

    #[test]
    fn test_card_ord() {
        assert!(Card::new('2') < Card::new('3'));
        assert!(Card::new('2') == Card::new('2'));
        assert!(!(Card::new('3') < Card::new('2')));
        assert!(Card::new('3') < Card::new('T'));
        assert!(Card::new('T') < Card::new('J'));
        assert!(Card::new('2') < Card::new('K'));
        assert!(Card::new('J') < Card::new('A'));
        assert!(Card::new('A') == Card::new('A'));
    }

    #[test]
    fn test_kind() {
        let mut hand = Hand::new("12345");
        assert_eq!(hand.kind(), HandKind::High);
        hand = Hand::new("11234");
        assert_eq!(hand.kind(), HandKind::One);
        hand = Hand::new("11223");
        assert_eq!(hand.kind(), HandKind::Two);
        hand = Hand::new("11123");
        assert_eq!(hand.kind(), HandKind::Three);
        hand = Hand::new("11122");
        assert_eq!(hand.kind(), HandKind::Full);
        hand = Hand::new("11112");
        assert_eq!(hand.kind(), HandKind::Four);
        hand = Hand::new("11111");
        assert_eq!(hand.kind(), HandKind::Five);
    }

    #[test]
    fn test_hand_ord() {
        let mut a = HandKind::High;
        let mut b = HandKind::One;
        assert!(a < b);
        a = HandKind::Two;
        assert!(a > b);
        b = HandKind::Three;
        assert!(a < b);
        a = HandKind::Full;
        assert!(a > b);
        b = HandKind::Four;
        assert!(a < b);
        a = HandKind::Five;
        assert!(a > b);
    }

    #[test]
    fn paranoid_test() {
        let a = Hand::new("T55J5");
        assert_eq!(HandKind::Three, a.kind());
        let b = Hand::new("32T3K");
        assert_eq!(HandKind::One, b.kind());
        assert!(b < a);
        assert!(a > b);
    }

    // #[test]
    // fn test_step1() {
    //     use crate::step1;
    //     let content: String = fs::read_to_string("test.txt").unwrap();

    //     assert_eq!(6440, step1(&content));
    // }

    #[test]
    fn test_jokers() {
        let a = Hand::new("JJJJJ");
        assert_eq!(HandKind::Five, a.kind());
        let b = Hand::new("AJJJJ");
        assert_eq!(HandKind::Five, b.kind());
        assert!(a < b);
        let c = Hand::new("JJJJA");
        assert_eq!(HandKind::Five, c.kind());
        assert!(a < c);
        assert!(c < b);
    }

    #[test]
    fn test_step2() {
        use crate::step2;
        let content: String = fs::read_to_string("test.txt").unwrap();

        assert_eq!(5905, step2(&content));
    }
}
