use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::Path;

use nom::{
    IResult,
    bytes::complete::tag,
    sequence::tuple,
    character::complete as cc,
    multi::separated_list1,
};

mod task2;

fn main() {
    let path = Path::new("day07/day7.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read file.");
    let mut hands = parse_file(&contents).unwrap().1;
    hands.sort();
    println!("Day7 Task1: {}", weight_hands(&hands));
    task2::hehe();
}


fn weight_hands(hands: &Vec<Hand>) -> i32 {
    let mut sum = 0;
    for (index, hand) in hands.into_iter().enumerate() {
        sum += hand.bid * (index + 1) as i32;
    }
    sum
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Eq, Debug)]
struct Hand {
    cards: String,
    bid: i32,
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let mut count_map: HashMap<char, i32> = HashMap::new();

        for card_char in self.cards.chars() {
            *count_map.entry(card_char).or_insert(0) += 1;
        }

        let mut counts = count_map.values().collect::<Vec<_>>();
        counts.sort_by(|a, b| b.cmp(a));

        if *counts[0] == 5 {
            HandType::FiveOfAKind
        } else if *counts[0] == 4 {
            HandType::FourOfAKind
        } else if *counts[0] == 3 && *counts[1] == 2 {
            HandType::FullHouse
        } else if *counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if *counts[0] == 2 && *counts[1] == 2 {
            HandType::TwoPair
        } else if *counts[0] == 1 {
            HandType::HighCard
        } else {
            HandType::OnePair
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let lookup: HashMap<char, i32> = [
            ('A', 14),
            ('K', 13),
            ('Q', 12),
            ('J', 11),
            ('T', 10),
            ('9', 9),
            ('8', 8),
            ('7', 7),
            ('6', 6),
            ('5', 5),
            ('4', 4),
            ('3', 3),
            ('2', 2),
        ].into();
        return if self.hand_type().eq(&other.hand_type()) {
            for (self_char, other_char) in zip(self.cards.chars(), other.cards.chars()) {
                if self_char != other_char {
                    return Some(lookup.get(&self_char).unwrap().cmp(lookup.get(&other_char).unwrap()))
                }
            }
            Some(Ordering::Equal)
        } else {
            self.hand_type().partial_cmp(&other.hand_type())
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type() > other.hand_type() {
            return Greater
        } else if self.hand_type() < other.hand_type() {
            return Less
        }
        self.partial_cmp(other).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use super::Hand;
    use super::HandType;
    use super::weight_hands;

    #[test]
    fn test_hand_types() {
        let hand1 = Hand { cards: "424KT".to_string(), bid: 464 };
        let hand2 = Hand { cards: "3J4QA".to_string(), bid: 464 };
        let hand3 = Hand { cards: "AAAAA".to_string(), bid: 464 };

        assert_eq!(hand1.hand_type(), HandType::OnePair, "Two 4's");
        assert_eq!(hand2.hand_type(), HandType::HighCard, "All different");
        assert_eq!(hand3.hand_type(), HandType::FiveOfAKind, "Five of a kind");

        assert_eq!(hand3.partial_cmp(&hand1), Some(Ordering::Greater));
        assert_eq!(hand2.partial_cmp(&hand1), Some(Ordering::Less));
        assert_eq!(hand2.partial_cmp(&hand2), Some(Ordering::Equal));
    }

    #[test]
    fn test_example() {
        let hand1 = Hand {cards: "32T3K".to_string(), bid: 765};
        let hand2 = Hand {cards: "T55J5".to_string(), bid: 684};
        let hand3 = Hand {cards: "KK677".to_string(), bid: 28};
        let hand4 = Hand {cards: "KTJJT".to_string(), bid: 220};
        let hand5 = Hand {cards: "QQQJA".to_string(), bid: 483};

        let mut hands = vec![hand1, hand2, hand3, hand4, hand5];
        hands.sort();
        assert_eq!(weight_hands(&hands), 6440);
    }
}

fn parse_file(i: &str) -> IResult<&str, Vec<Hand>> {
    let (i, hands) = separated_list1(tag("\n"), parse_hand)(i)?;
    Ok((i, hands))
}

fn parse_hand(i: &str) -> IResult<&str, Hand> {
    let (i, (cards, _, bid)) = tuple((cc::alphanumeric1, cc::space1, cc::i32))(i)?;
    Ok((i, Hand { cards: cards.to_string(), bid}))
}