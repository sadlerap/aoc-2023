use color_eyre::Result;
use winnow::{
    ascii::{dec_uint, digit1, newline, space1},
    combinator::separated,
    PResult, Parser,
};

struct Game {
    time: u64,
    record: u64,
}

fn game_parser_part_1(input: &mut &str) -> PResult<Vec<Game>> {
    (
        "Time:",
        space1,
        separated::<_, _, Vec<u64>, _, _, _, _>(1.., dec_uint::<_, u64, _>, space1),
        newline,
        "Distance:",
        space1,
        separated::<_, _, Vec<u64>, _, _, _, _>(1.., dec_uint::<_, u64, _>, space1),
        newline,
    )
        .map(
            |(_, _, times, _, _, _, records, _): (_, _, Vec<u64>, _, _, _, Vec<u64>, _)| {
                times
                    .iter()
                    .zip(&records)
                    .map(|(&time, &record)| Game { time, record })
                    .collect()
            },
        )
        .parse_next(input)
}

fn solve(input: &[Game]) -> u64 {
    input
        .iter()
        .map(|Game { time, record }| {
            // requirements:
            // h + t == time
            // h * t >= record
            (1..=(time - 1))
                .map(|h| {
                    let t = time - h;
                    (h, t)
                })
                .filter(|(h, t)| h * t > *record)
                .count() as u64
        })
        .product()
}

fn game_parser_part_2(input: &mut &str) -> PResult<Game> {
    (
        ("Time:", space1, separated(1.., digit1, space1), newline)
            .try_map(|(_, _, x, _): (_, _, String, _)| x.parse::<u64>()),
        ("Distance:", space1, separated(1.., digit1, space1), newline)
            .try_map(|(_, _, x, _): (_, _, String, _)| x.parse::<u64>()),
    )
        .map(|(time, record): (u64, u64)| Game { time, record })
        .parse_next(input)
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("input.txt");
    let part1 = game_parser_part_1
        .parse(input)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse input: {}", e.to_string()))?;
    let part2 = game_parser_part_2.parse(input)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to parse input: {}", e.to_string()))?;

    println!("part 1: {}", solve(&part1));
    println!("part 2: {}", solve(&[part2]));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200\n";
        let games = game_parser_part_1.parse(input).unwrap();
        assert_eq!(solve(&games), 288);
    }

    #[test]
    fn test_part2() {
        let input = "Time:      7  15   30\n\
                     Distance:  9  40  200\n";
        let games = game_parser_part_2.parse(input).unwrap();
        assert_eq!(solve(&[games]), 71503);
    }
}
