use color_eyre::{eyre::eyre, Result};
use winnow::{
    ascii::{dec_uint, space1},
    combinator::{alt, separated},
    prelude::*,
};

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn parse(input: &mut &str) -> PResult<Self> {
        let mut round = Round::default();
        let value = (
            dec_uint,
            space1,
            alt((
                "red".map(|_| Color::Red),
                "green".map(|_| Color::Green),
                "blue".map(|_| Color::Blue),
            )),
        )
            .map(|(num, _, color): (u32, _, _)| (num, color));

        let values: Vec<(u32, Color)> = separated(1.., value, (",", space1)).parse_next(input)?;
        for &(num, color) in values.iter() {
            match color {
                Color::Red => round.red = num,
                Color::Green => round.green = num,
                Color::Blue => round.blue = num,
            }
        }

        Ok(round)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(s: &mut &str) -> PResult<Self> {
        (
            "Game",
            space1,
            dec_uint,
            ":",
            space1,
            separated(1.., Round::parse, (";", space1)),
        )
            .map(|(_, _, id, _, _, rounds)| Game { id, rounds })
            .parse_next(s)
    }
}

fn part1(input: &str, red_limit: u32, green_limit: u32, blue_limit: u32) -> Result<u32> {
    input
        .lines()
        .map(|line| {
            Game::parse.parse(line).map_err(|e| eyre!("Failed to parse game: {}", e.to_string()))
        })
        .try_fold(0u32, |acc, game| {
            let game = game?;
            if game.rounds.iter().all(|round| {
                round.red <= red_limit && round.green <= green_limit && round.blue <= blue_limit
            }) {
                Ok(acc + game.id)
            } else {
                Ok(acc)
            }
        })
}

fn part2(input: &str) -> Result<u32> {
    input
        .lines()
        .map(|mut line| {
            Game::parse(&mut line).map_err(|e| eyre!("Failed to parse game: {}", e.to_string()))
        })
        .try_fold(0u32, |acc, game| {
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            game?.rounds.iter().for_each(|round| {
                min_red = min_red.max(round.red);
                min_green = min_green.max(round.green);
                min_blue = min_blue.max(round.blue);
            });

            Ok(acc + min_red * min_green * min_blue)
        })
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input, 12, 13, 14)?);
    println!("Part 2: {}", part2(input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_round() {
        let mut x = "1 red, 3 blue, 11 green";
        assert_eq!(
            Round::parse(&mut x).unwrap(),
            Round {
                red: 1,
                green: 11,
                blue: 3
            }
        )
    }

    #[test]
    fn test_parse_game() {
        let mut x = include_str!("input.txt").lines().next().unwrap();
        assert_eq!(
            Game::parse(&mut x).unwrap(),
            Game {
                id: 1,
                rounds: vec![
                    Round {
                        red: 1,
                        green: 11,
                        blue: 3
                    },
                    Round {
                        red: 5,
                        green: 0,
                        blue: 1
                    },
                    Round {
                        red: 13,
                        green: 5,
                        blue: 3
                    },
                    Round {
                        red: 6,
                        green: 4,
                        blue: 1
                    },
                    Round {
                        red: 16,
                        green: 12,
                        blue: 0
                    },
                ]
            }
        )
    }
}
