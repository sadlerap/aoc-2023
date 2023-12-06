use std::collections::BTreeSet;

use color_eyre::{eyre::eyre, Result};
use winnow::{
    ascii::{dec_uint, space1},
    combinator::separated,
    error::ParseError,
    PResult, Parser,
};

struct Card {
    #[allow(unused)]
    id: u32,
    winning_numbers: BTreeSet<u32>,
    given_numbers: BTreeSet<u32>,
}

impl Card {
    fn parse(input: &mut &str) -> PResult<Self> {
        (
            "Card",
            space1,
            dec_uint,
            ":",
            space1,
            separated(1.., dec_uint, space1),
            space1,
            "|",
            space1,
            separated(1.., dec_uint, space1),
        )
            .map(
                |(_, _, id, _, _, winning_numbers, _, _, _, given_numbers)| Card {
                    id,
                    winning_numbers,
                    given_numbers,
                },
            )
            .parse_next(input)
    }

    fn score_part1(&self) -> u32 {
        let intersection_size = self
            .winning_numbers
            .intersection(&self.given_numbers)
            .count();
        (1 << intersection_size) >> 1
    }
}

fn part1(input: &str) -> Result<u32> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Card::parse.parse(line).map(|card| card.score_part1()))
        .try_fold(0, |acc, x| Ok(acc + x?))
        .map_err(|e: ParseError<_, _>| eyre!("Failed to parse input: {}", e.to_string()))
}

fn part2(input: &str) -> Result<u32> {
    let cards: Vec<Card> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Card::parse.parse(line))
        .collect::<Result<Vec<Card>, _>>()
        .map_err(|e| eyre!("Failed to parse input: {}", e.to_string()))?;

    let mut card_counts = vec![1u32; cards.len()];
    for card in cards {
        let num_intersections = card
            .winning_numbers
            .intersection(&card.given_numbers)
            .count();
        let current_multiplier = card_counts[(card.id - 1) as usize];
        for i in 0..num_intersections {
            if let Some(x) = card_counts.get_mut(card.id as usize + i) {
                *x += current_multiplier;
            }
        }
    }

    Ok(card_counts.into_iter().sum())
}

fn main() -> Result<()> {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input)?);
    println!("Part 2: {}", part2(input)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_given() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input).unwrap(), 13)
    }

    #[test]
    fn part2_given() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part2(input).unwrap(), 30);
    }
}
