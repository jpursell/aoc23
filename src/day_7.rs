use std::{convert::TryFrom, str::FromStr};

use counter::Counter;

#[derive(Hash, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl TryFrom<char> for Card {
    type Error = ParseCardError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::J),
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

enum HandType {
    FiveOAK,
    FourOAK,
    FullHouse,
    ThreeOAK,
    TwoPair,
    OnePair,
    HighCard,
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
    fn is_five_of_a_kind(&self) -> bool {
        assert_eq!(self.cards.len(), 5);
        self.cards[1..].iter().all(|card| *card == self.cards[0])
    }
    fn is_four_of_a_kind(&self) -> bool {
        assert_eq!(self.cards.len(), 5);
        let count = self.cards.iter().collect::<Counter<_>>();
        if count.len() == 2 &&
        self.cards[1..].iter().all(|card| *card == self.cards[0])
    }
    fn determine_hand_type(&self) -> Result<HandType, ParseHandTypeError> {
        if self.cards.len() != 5 {return Err(ParseHandTypeError);}
        if self.is_five_of_a_kind() {
            return Ok(HandType::FiveOAK);
        } else if self.is_four_of_a_kind() { 
            return Ok(HandType::FourOAK);
        }
        // TODO add other hands
        Err(ParseHandTypeError)
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
    hands: Vec<(Hand, Bid)>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseGameError;

impl FromStr for Game {
    type Err = ParseGameError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands = s
            .lines()
            .map(|line| {
                let (hand, bid) = line.split_once(" ").unwrap();
                (
                    hand.parse::<Hand>().unwrap(),
                    Bid {
                        amount: bid.parse::<u64>().unwrap(),
                    },
                )
            })
            .collect::<Vec<_>>();
        Ok(Game { hands })
    }
}

pub fn day_7() {
    let input = include_str!("day_7_data.txt");
    println!("day 7 a {}", day_7_a(input));
    println!("day 7 b {}", day_7_b(input));
}

fn day_7_a(input: &str) -> u32 {
    dbg!(input.parse::<Game>());
    0
}

fn day_7_b(input: &str) -> u32 {
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
        assert_eq!(super::day_7_a(input), 0);
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
            Card::A,
            Card::K,
            Card::Q,
            Card::J,
            Card::T,
            Card::C9,
            Card::C8,
            Card::C7,
            Card::C6,
            Card::C5,
            Card::C4,
            Card::C3,
            Card::C2,
        ];
        arr.sort();
        assert_eq!(arr, sorted_arr);
    }
}
