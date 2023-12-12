use color_eyre::eyre::eyre;

use std::{cmp::Ordering, collections::BTreeMap};

use winnow::{
    ascii::{dec_uint, newline, space1},
    combinator::{alt, repeat, terminated},
    error::ContextError,
    PResult, Parser,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse<'a>(use_jokers: bool) -> impl Parser<&'a str, Self, ContextError> {
        move |input: &mut &'a str| -> PResult<Self> {
            alt((
                "A".map(|_| Card::Ace),
                "2".map(|_| Card::Two),
                "3".map(|_| Card::Three),
                "4".map(|_| Card::Four),
                "5".map(|_| Card::Five),
                "6".map(|_| Card::Six),
                "7".map(|_| Card::Seven),
                "8".map(|_| Card::Eight),
                "9".map(|_| Card::Nine),
                "T".map(|_| Card::Ten),
                "J".map(|_| if use_jokers { Card::Joker } else { Card::Jack }),
                "Q".map(|_| Card::Queen),
                "K".map(|_| Card::King),
            ))
            .parse_next(input)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum HandType {
    HighCard(Card),
    OnePair(Card),
    TwoPair { major: Card, minor: Card },
    ThreeOfAKind(Card),
    FullHouse { major: Card, minor: Card },
    FourOfAKind(Card),
    FullSet(Card),
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    hand: [Card; 5],
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_cmp = self
            .hand
            .into_iter()
            .zip(other.hand)
            .find_map(|(x, y)| match x.cmp(&y) {
                Ordering::Equal => None,
                x => Some(x),
            })
            .unwrap_or(Ordering::Equal);
        match (&self.hand_type, &other.hand_type) {
            (HandType::HighCard(_), HandType::HighCard(_)) => hand_cmp,
            (HandType::HighCard(_), HandType::OnePair(_)) => Ordering::Less,
            (HandType::HighCard(_), HandType::TwoPair { .. }) => Ordering::Less,
            (HandType::HighCard(_), HandType::ThreeOfAKind(_)) => Ordering::Less,
            (HandType::HighCard(_), HandType::FullHouse { .. }) => Ordering::Less,
            (HandType::HighCard(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::HighCard(_), HandType::FullSet(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::HighCard(_)) => Ordering::Greater,
            (HandType::OnePair(_), HandType::OnePair(_)) => hand_cmp,
            (HandType::OnePair(_), HandType::TwoPair { .. }) => Ordering::Less,
            (HandType::OnePair(_), HandType::ThreeOfAKind(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::FullHouse { .. }) => Ordering::Less,
            (HandType::OnePair(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::OnePair(_), HandType::FullSet(_)) => Ordering::Less,
            (HandType::TwoPair { .. }, HandType::HighCard(_)) => Ordering::Greater,
            (HandType::TwoPair { .. }, HandType::OnePair(_)) => Ordering::Greater,
            (HandType::TwoPair { .. }, HandType::TwoPair { .. }) => hand_cmp,
            (HandType::TwoPair { .. }, HandType::ThreeOfAKind(_)) => Ordering::Less,
            (HandType::TwoPair { .. }, HandType::FullHouse { .. }) => Ordering::Less,
            (HandType::TwoPair { .. }, HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::TwoPair { .. }, HandType::FullSet(_)) => Ordering::Less,
            (HandType::ThreeOfAKind(_), HandType::HighCard(_)) => Ordering::Greater,
            (HandType::ThreeOfAKind(_), HandType::OnePair(_)) => Ordering::Greater,
            (HandType::ThreeOfAKind(_), HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::ThreeOfAKind(_), HandType::ThreeOfAKind(_)) => hand_cmp,
            (HandType::ThreeOfAKind(_), HandType::FullHouse { .. }) => Ordering::Less,
            (HandType::ThreeOfAKind(_), HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::ThreeOfAKind(_), HandType::FullSet(_)) => Ordering::Less,
            (HandType::FullHouse { .. }, HandType::HighCard(_)) => Ordering::Greater,
            (HandType::FullHouse { .. }, HandType::OnePair(_)) => Ordering::Greater,
            (HandType::FullHouse { .. }, HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::FullHouse { .. }, HandType::ThreeOfAKind(_)) => Ordering::Greater,
            (HandType::FullHouse { .. }, HandType::FullHouse { .. }) => hand_cmp,
            (HandType::FullHouse { .. }, HandType::FourOfAKind(_)) => Ordering::Less,
            (HandType::FullHouse { .. }, HandType::FullSet(_)) => Ordering::Less,
            (HandType::FourOfAKind(_), HandType::HighCard(_)) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::OnePair(_)) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::ThreeOfAKind(_)) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::FullHouse { .. }) => Ordering::Greater,
            (HandType::FourOfAKind(_), HandType::FourOfAKind(_)) => hand_cmp,
            (HandType::FourOfAKind(_), HandType::FullSet(_)) => Ordering::Less,
            (HandType::FullSet(_), HandType::HighCard(_)) => Ordering::Greater,
            (HandType::FullSet(_), HandType::OnePair(_)) => Ordering::Greater,
            (HandType::FullSet(_), HandType::TwoPair { .. }) => Ordering::Greater,
            (HandType::FullSet(_), HandType::ThreeOfAKind(_)) => Ordering::Greater,
            (HandType::FullSet(_), HandType::FullHouse { .. }) => Ordering::Greater,
            (HandType::FullSet(_), HandType::FourOfAKind(_)) => Ordering::Greater,
            (HandType::FullSet(_), HandType::FullSet(_)) => hand_cmp,
        }
    }
}

impl Hand {
    fn parse<'a>(use_jokers: bool) -> impl Parser<&'a str, Self, ContextError> {
        move |input: &mut &'a str| -> PResult<Self> {
            let (a, b, c, d, e) = (
                Card::parse(use_jokers),
                Card::parse(use_jokers),
                Card::parse(use_jokers),
                Card::parse(use_jokers),
                Card::parse(use_jokers),
            )
                .parse_next(input)?;

            let hand = [a, b, c, d, e];

            let mut card_count: BTreeMap<Card, u8> = Default::default();
            for card in &hand {
                *card_count.entry(*card).or_default() += 1u8;
            }
            if let Some(joker_value) = card_count.get(&Card::Joker).cloned() {
                // we need to normalize jokers to their highest value, which is always going to be
                // the most frequent card available in the hand
                if let Some(value) = card_count
                    .iter_mut()
                    .filter(|(k, _)| **k != Card::Joker)
                    .max_by(|x, y| x.1.cmp(&(*y).1))
                    .map(|(_, v)| v)
                {
                    *value += joker_value;
                } else {
                    card_count.insert(Card::Ace, joker_value);
                }
                card_count.remove(&Card::Joker);
            }

            let hand_type = match card_count.values().filter(|v| **v == 1).count() {
                0 => {
                    if let Some(k) = card_count.iter().find(|(_, v)| **v == 5).map(|x| x.0) {
                        HandType::FullSet(*k)
                    } else {
                        let three = card_count
                            .iter()
                            .find(|(_, v)| **v == 3)
                            .map(|x| x.0)
                            .expect("Three-of-a-kind should exist");
                        let two = card_count
                            .iter()
                            .find(|(_, v)| **v == 2)
                            .map(|x| x.0)
                            .expect("Pair should exist");
                        HandType::FullHouse {
                            major: *three,
                            minor: *two,
                        }
                    }
                }
                1 => {
                    // we can either have a four-of-a-kind or two pair
                    if let Some(four_card) = card_count.iter().find(|(_, v)| **v == 4).map(|x| x.0)
                    {
                        // four-of-a-kind
                        HandType::FourOfAKind(*four_card)
                    } else {
                        // two pair
                        let cards: Vec<Card> = card_count
                            .iter()
                            .filter(|(_, v)| **v == 2)
                            .map(|x| *x.0)
                            .collect();
                        assert_eq!(cards.len(), 2, "Should have two pair");
                        let a = cards[0];
                        let b = cards[1];
                        let major = a.max(b);
                        let minor = a.min(b);
                        HandType::TwoPair { major, minor }
                    }
                }
                2 => {
                    // should be a three-of-a-kind, since otherwise we would've had another single
                    // card, and we'd be in the 3 branch, not here.
                    let triple_card = card_count
                        .iter()
                        .find(|(_, v)| **v == 3)
                        .map(|x| x.0)
                        .expect("Should be a three-of-a-kind");
                    HandType::ThreeOfAKind(*triple_card)
                }
                3 => {
                    let pair_card = card_count
                        .iter()
                        .find(|(_, v)| **v == 2)
                        .map(|x| x.0)
                        .expect("Should be a pair card");
                    HandType::OnePair(*pair_card)
                }
                5 => {
                    let highest_card = a.max(b).max(c).max(d).max(e);
                    HandType::HighCard(highest_card)
                }
                _ => unreachable!("This shouldn't be possible..."),
            };

            Ok(Hand { hand_type, hand })
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Round {
    hand: Hand,
    bid: u64,
}

impl Round {
    fn parse_part1(input: &mut &str) -> PResult<Self> {
        (Hand::parse(false), space1, dec_uint)
            .map(|(hand, _, bid)| Round { hand, bid })
            .parse_next(input)
    }

    fn parse_part2(input: &mut &str) -> PResult<Self> {
        (Hand::parse(true), space1, dec_uint)
            .map(|(hand, _, bid)| Round { hand, bid })
            .parse_next(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse_part1(input: &mut &str) -> PResult<Self> {
        repeat(1.., terminated(Round::parse_part1, newline))
            .map(|rounds| Game { rounds })
            .parse_next(input)
    }

    fn parse_part2(input: &mut &str) -> PResult<Self> {
        repeat(1.., terminated(Round::parse_part2, newline))
            .map(|rounds| Game { rounds })
            .parse_next(input)
    }

    fn part1(mut self) -> u64 {
        self.rounds.sort_by(|x, y| x.hand.cmp(&y.hand));
        self.rounds
            .iter()
            .enumerate()
            .map(|(i, round)| (i as u64 + 1) * round.bid)
            .sum()
    }

    fn part2(mut self) -> u64 {
        self.rounds.sort_by(|x, y| x.hand.cmp(&y.hand));
        self.rounds
            .iter()
            .enumerate()
            .map(|(i, round)| (i as u64 + 1) * round.bid)
            .sum()
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let game1 = Game::parse_part1
        .parse(input)
        .map_err(|e| eyre!("Failed to parse input: {}", e.to_string()))?;
    let game2 = Game::parse_part2
        .parse(input)
        .map_err(|e| eyre!("Failed to parse input: {}", e.to_string()))?;

    println!("part 1: {}", game1.part1());
    println!("part 2: {}", game2.part2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str =
    "32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n";

    #[test]
    fn test_parse1() {
        let game = Game::parse_part1
            .parse(INPUT)
            .expect("Failed to parse input");
        let actual = Game {
            rounds: vec![
                Round {
                    hand: Hand {
                        hand_type: HandType::OnePair(Card::Three),
                        hand: [Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                    },
                    bid: 765,
                },
                Round {
                    hand: Hand {
                        hand_type: HandType::ThreeOfAKind(Card::Five),
                        hand: [Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                    },
                    bid: 684,
                },
                Round {
                    hand: Hand {
                        hand_type: HandType::TwoPair {
                            major: Card::King,
                            minor: Card::Seven,
                        },
                        hand: [Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                    },
                    bid: 28,
                },
                Round {
                    hand: Hand {
                        hand_type: HandType::TwoPair {
                            major: Card::Jack,
                            minor: Card::Ten,
                        },
                        hand: [Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                    },
                    bid: 220,
                },
                Round {
                    hand: Hand {
                        hand_type: HandType::ThreeOfAKind(Card::Queen),
                        hand: [Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                    },
                    bid: 483,
                },
            ],
        };
        assert_eq!(&game, &actual);
    }

    #[test]
    fn test_part1() {
        let game = Game::parse_part1
            .parse(INPUT)
            .expect("Failed to parse input");
        assert_eq!(game.part1(), 6440)
    }

    #[test]
    fn test_part2() {
        let game = Game::parse_part2
            .parse(INPUT)
            .expect("Failed to parse input");
        assert_eq!(game.part1(), 5905)
    }
}
