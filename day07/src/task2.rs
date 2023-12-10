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

pub fn hehe() {
    let path = Path::new("src/day7.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read file.");
    let mut hands = parse_file(&contents).unwrap().1;
    hands.sort();
    println!("Day7 Task2: {}", weight_hands(&hands));
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
            handle_joker(HandType::FourOfAKind, &self.cards)
        } else if *counts[0] == 3 && *counts[1] == 2 {
            handle_joker(HandType::FullHouse, &self.cards)
        } else if *counts[0] == 3 {
            handle_joker(HandType::ThreeOfAKind, &self.cards)
        } else if *counts[0] == 2 && *counts[1] == 2 {
            handle_joker(HandType::TwoPair, &self.cards)
        } else if *counts[0] == 1 {
            handle_joker(HandType::HighCard, &self.cards)
        } else {
            handle_joker(HandType::OnePair, &self.cards)
        }
    }
}

fn handle_joker(current_type: HandType, cards: &String) -> HandType {
    let num_joker = cards.chars().filter(|char| *char == 'J').count();
    if num_joker == 0 {
        return current_type;
    }
    let after_joker = match current_type {
        HandType::FourOfAKind => HandType::FiveOfAKind,
        HandType::FullHouse => if num_joker == 1 {
            HandType::FourOfAKind
        } else {
            HandType::FiveOfAKind
        },
        HandType::ThreeOfAKind => if num_joker == 1 {
            HandType::FourOfAKind
        } else if num_joker == 3 {
            HandType::FourOfAKind
        } else {
            HandType::FiveOfAKind
        }
        HandType::TwoPair => if num_joker == 2 {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        },
        HandType::OnePair => HandType::ThreeOfAKind,
        HandType::HighCard => HandType::OnePair,
        default => default
    };
    after_joker
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
            ('J', 1),
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
    fn test_hand_types2() {
        let hand1 = Hand { cards: "424KT".to_string(), bid: 464 };
        let hand2 = Hand { cards: "3J4QA".to_string(), bid: 464 };
        let hand3 = Hand { cards: "AAAAA".to_string(), bid: 464 };

        assert_eq!(hand1.hand_type(), HandType::OnePair);
        assert_eq!(hand2.hand_type(), HandType::OnePair);
        assert_eq!(hand3.hand_type(), HandType::FiveOfAKind);

        assert_eq!(hand3.partial_cmp(&hand1), Some(Ordering::Greater));
        assert_eq!(hand2.partial_cmp(&hand1), Some(Ordering::Less));
        assert_eq!(hand2.partial_cmp(&hand2), Some(Ordering::Equal));
    }

    #[test]
    fn test_example2() {
        let hand1 = Hand {cards: "32T3K".to_string(), bid: 765};
        let hand2 = Hand {cards: "T55J5".to_string(), bid: 684};
        let hand3 = Hand {cards: "KK677".to_string(), bid: 28};
        let hand4 = Hand {cards: "KTJJT".to_string(), bid: 220};
        let hand5 = Hand {cards: "QQQJA".to_string(), bid: 483};


        let mut hands = vec![hand1, hand2, hand3, hand4, hand5];
        hands.sort();

        assert_eq!(weight_hands(&hands), 5905);
    }

    #[test]
    fn four_queens_less_four_kings() {
        let hand4 = Hand {cards: "KTJJT".to_string(), bid: 220};
        let hand5 = Hand {cards: "QQQJA".to_string(), bid: 483};
        assert_eq!(hand5.hand_type(), HandType::FourOfAKind, "Four tens");
        assert_eq!(hand4.hand_type(), HandType::FourOfAKind);
        assert_eq!(hand5.partial_cmp(&hand4), Some(Ordering::Less));
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