use itertools::Itertools;
use std::{convert::TryFrom, str::FromStr};

use counter::Counter;

#[derive(Hash, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl TryFrom<char> for Card {
    type Error = ParseCardError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::C9),
            '8' => Ok(Card::C8),
            '7' => Ok(Card::C7),
            '6' => Ok(Card::C6),
            '5' => Ok(Card::C5),
            '4' => Ok(Card::C4),
            '3' => Ok(Card::C3),
            '2' => Ok(Card::C2),
            _ => Err(ParseCardError),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandTypeError;

impl TryFrom<Hand> for HandType {
    type Error = ParseHandTypeError;

    fn try_from(hand: Hand) -> Result<Self, Self::Error> {
        hand.determine_hand_type()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn determine_hand_type(&self) -> Result<HandType, ParseHandTypeError> {
        if self.cards.len() != 5 {
            return Err(ParseHandTypeError);
        }
        let most_common = self
            .cards
            .iter()
            .collect::<Counter<_>>()
            .most_common_ordered();
        let signature = most_common
            .iter()
            .map(|(_card, count)| count)
            .collect::<Vec<_>>();
        match signature.len() {
            1 => Ok(HandType::FiveOAK),
            2 => match signature.iter().collect_tuple().unwrap() {
                (4, 1) => Ok(HandType::FourOAK),
                (3, 2) => Ok(HandType::FullHouse),
                _ => Err(ParseHandTypeError),
            },
            3 => match signature.iter().collect_tuple().unwrap() {
                (3, 1, 1) => Ok(HandType::ThreeOAK),
                (2, 2, 1) => Ok(HandType::TwoPair),
                _ => Err(ParseHandTypeError),
            },
            4 => match signature.iter().collect_tuple().unwrap() {
                (2, 1, 1, 1) => Ok(HandType::OnePair),
                _ => Err(ParseHandTypeError),
            },
            5 => match signature.iter().collect_tuple().unwrap() {
                (1, 1, 1, 1, 1) => Ok(HandType::HighCard),
                _ => Err(ParseHandTypeError),
            },
            _ => Err(ParseHandTypeError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseHandError;

impl FromStr for Hand {
    type Err = ParseHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(ParseHandError);
        }
        Ok(Hand {
            cards: s
                .chars()
                .map(|c| c.try_into().unwrap())
                .collect::<Vec<Card>>(),
        })
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Bid {
    amount: u64,
}

#[derive(Debug)]
struct Game {
    hands: Vec<(HandType, Hand, Bid)>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hands = s
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                let hand = hand.parse::<Hand>().unwrap();
                let hand_type = hand
                    .determine_hand_type()
                    .expect(format!("failed to determine hand type for {:?}", hand).as_str());
                (
                    hand_type,
                    hand,
                    Bid {
                        amount: bid.parse::<u64>().unwrap(),
                    },
                )
            })
            .collect::<Vec<_>>();
        hands.sort();
        Ok(Game { hands })
    }
}

impl Game {
    fn total_winnings(&self) -> u64 {
        self.hands
            .iter()
            .enumerate()
            .map(|(rank, (_, _, bid))| bid.amount * (rank as u64 + 1))
            .sum()
    }
}

pub fn day_7() {
    let input = include_str!("day_7_data.txt");
    println!("day 7 a {}", day_7_a(input));
    println!("day 7 b {}", day_7_b(input));
}

fn day_7_a(input: &str) -> u64 {
    let game = input.parse::<Game>().unwrap();
    game.total_winnings()
}

fn day_7_b(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::Card;

    #[test]
    fn test1() {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        assert_eq!(super::day_7_a(input), 6440);
        assert_eq!(super::day_7_b(input), 0);
    }

    #[test]
    fn test_card_sort() {
        let mut arr = vec![
            Card::K,
            Card::A,
            Card::J,
            Card::Q,
            Card::C9,
            Card::T,
            Card::C7,
            Card::C8,
            Card::C5,
            Card::C6,
            Card::C3,
            Card::C4,
            Card::C2,
        ];
        let sorted_arr = vec![
            Card::C2,
            Card::C3,
            Card::C4,
            Card::C5,
            Card::C6,
            Card::C7,
            Card::C8,
            Card::C9,
            Card::T,
            Card::J,
            Card::Q,
            Card::K,
            Card::A,
        ];
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }
}
